use zed_extension_api as zed;

struct PlyExtension;

impl zed::Extension for PlyExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        // Wasm cannot tell whether a path exists. An explicit binary wins.
        // The compiler checkout keeps using its debug build. Every other
        // worktree uses plylang-ls from PATH.
        let command = env_bin(worktree)
            .or_else(|| compiler_binary(worktree))
            .or_else(|| worktree.which("plylang-ls"));
        let Some(command) = command else {
            return Err(
                "plylang-ls was not found. Install it on PATH, set PLYLANG_LS_BIN, or run `cargo build -p plylang-ls` in the plylang repository."
                    .to_string(),
            );
        };
        Ok(zed::Command {
            command,
            args: Vec::new(),
            env: Vec::new(),
        })
    }
}

fn env_bin(worktree: &zed::Worktree) -> Option<String> {
    worktree.shell_env().into_iter().find_map(|(key, value)| {
        (key == "PLYLANG_LS_BIN" && !value.is_empty()).then_some(value)
    })
}

fn compiler_binary(worktree: &zed::Worktree) -> Option<String> {
    let manifest = worktree.read_text_file("Cargo.toml").ok()?;
    if !manifest.contains("crates/plylang-ls") {
        return None;
    }
    let root = worktree.root_path();
    if root.is_empty() {
        return None;
    }
    let sep = if root.ends_with('/') { "" } else { "/" };
    Some(format!("{root}{sep}target/debug/plylang-ls"))
}

zed::register_extension!(PlyExtension);
