use std::fs;
use zed_extension_api::{
    self as zed,
    http_client::{HttpMethod, HttpRequest, RedirectPolicy},
    Command, ContextServerId, DownloadedFileType, Project, Result,
};

const S3_BUCKET_URL: &str = "https://tangleguard-mcp-builds.s3.eu-central-1.amazonaws.com";
const BINARY_NAME: &str = "tangleguard-mcp";

struct TangleGuardExtension {
    cached_binary_path: Option<String>,
}

impl TangleGuardExtension {
    fn fetch_latest_version(&self) -> Result<String> {
        let version_url = format!("{S3_BUCKET_URL}/latest/VERSION");

        let response = zed::http_client::fetch(&HttpRequest {
            method: HttpMethod::Get,
            url: version_url.clone(),
            headers: vec![],
            body: None,
            redirect_policy: RedirectPolicy::FollowAll,
        })
        .map_err(|e| format!("failed to fetch VERSION file from {version_url}: {e}"))?;

        let version = String::from_utf8(response.body)
            .map_err(|e| format!("VERSION file contains invalid UTF-8: {e}"))?
            .trim()
            .to_string();

        Ok(version)
    }

    fn context_server_binary_path(
        &mut self,
        _context_server_id: &ContextServerId,
    ) -> Result<String> {
        // Return cached path if binary still exists
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        // Fetch the latest version from S3
        let version = self.fetch_latest_version()?;

        let (platform, arch) = zed::current_platform();

        // Build the binary name based on platform and architecture
        // Format: tangleguard-mcp_0.1.0_aarch64-apple-darwin
        let binary_suffix = format!(
            "{BINARY_NAME}_{version}_{arch}-{os}",
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X86 => "x86",
                zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "apple-darwin",
                zed::Os::Linux => "unknown-linux-gnu",
                zed::Os::Windows => "pc-windows-msvc",
            },
        );

        // Add .exe suffix for Windows
        let binary_filename = match platform {
            zed::Os::Windows => format!("{binary_suffix}.exe"),
            _ => binary_suffix.clone(),
        };

        // Download URL from S3 bucket
        let download_url = format!("{S3_BUCKET_URL}/v{version}/{binary_suffix}");

        // Create version directory
        let version_dir = format!("{BINARY_NAME}-{version}");
        fs::create_dir_all(&version_dir)
            .map_err(|err| format!("failed to create directory '{version_dir}': {err}"))?;

        let binary_path = format!("{version_dir}/{binary_filename}");

        // Download if binary doesn't exist
        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::download_file(
                &download_url,
                &binary_path,
                DownloadedFileType::Uncompressed,
            )
            .map_err(|e| format!("failed to download file from {download_url}: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            // Clean up old versions
            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory: {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry: {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for TangleGuardExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        Ok(Command {
            command: self.context_server_binary_path(context_server_id)?,
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(TangleGuardExtension);
