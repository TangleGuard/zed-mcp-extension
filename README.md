# TangleGuard's MCP Extension for Zed

An MCP extension for the Zed Editor.

With this extension installed, AI agents can ask questions about the codebases overall structure, without reading in the entire codebase itself.

Instead TangleGuard's MCP server provides a structured, small representation of the codebases structure.

This approach has the following advantages:

- Eliminate the need for the LLM/AI agent to scan the entire codebase itself
- Reduce the amount of read tokens required to understand the codebase
- Faster, more reliable and efficient analysis

TangleGuard caches the over codebase representation within the MCP server.
This allows for efficient, repeatable analysis of the codebase without needing to recompute the representation each time.
That way the have access to the codebase abstraction across the agent chats.
