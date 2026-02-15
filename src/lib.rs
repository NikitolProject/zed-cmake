use zed_extension_api::{self as zed, serde_json, LanguageServerId, Result, Worktree};

struct CMakeExtension {
    cached_binary_path: Option<String>,
}

impl zed::Extension for CMakeExtension {
    fn new() -> Self {
        CMakeExtension {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let binary_path = self
            .cached_binary_path
            .clone()
            .or_else(|| worktree.which("neocmakelsp-fast"))
            .ok_or_else(|| {
                "neocmakelsp-fast not found. Install from: https://github.com/NikitolProject/neocmakelsp-fast".to_string()
            })?;

        self.cached_binary_path = Some(binary_path.clone());

        Ok(zed::Command {
            command: binary_path,
            args: vec!["stdio".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(serde_json::json!({
            "format": {
                "enable": true
            },
            "lint": {
                "enable": true
            },
            "scan_cmake_in_package": false,
            "semantic_token": false
        })))
    }
}

zed::register_extension!(CMakeExtension);
