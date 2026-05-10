use zed_extension_api as zed;

struct TangleGuardExtension;

impl zed::Extension for TangleGuardExtension {
    fn new() -> Self {
        TangleGuardExtension
    }
}

zed::register_extension!(TangleGuardExtension);
