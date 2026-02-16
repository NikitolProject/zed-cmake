use zed_extension_api::{
    self as zed, download_file, latest_github_release, make_file_executable, serde_json,
    Architecture, DownloadedFileType, GithubReleaseOptions, LanguageServerId,
    LanguageServerInstallationStatus, Os, Result, Worktree,
};

const GITHUB_REPO: &str = "NikitolProject/neocmakelsp-fast";
const BINARY_NAME: &str = "neocmakelsp-fast";

struct CMakeExtension {
    cached_binary_path: Option<String>,
}

impl CMakeExtension {
    fn language_server_binary_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if std::fs::metadata(path).map(|m| m.is_file()).unwrap_or(false) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = latest_github_release(
            GITHUB_REPO,
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();

        let (asset_suffix, file_type, binary_suffix) = match platform {
            Os::Mac => ("universal-apple-darwin.tar.gz", DownloadedFileType::GzipTar, ""),
            Os::Linux => {
                let arch_str = match arch {
                    Architecture::Aarch64 => "aarch64",
                    Architecture::X8664 => "x86_64",
                    Architecture::X86 => return Err("x86 (32-bit) Linux is not supported".into()),
                };
                (
                    format!("{arch_str}-unknown-linux-gnu.tar.gz").leak() as &'static str,
                    DownloadedFileType::GzipTar,
                    "",
                )
            }
            Os::Windows => {
                let arch_str = match arch {
                    Architecture::Aarch64 => "aarch64",
                    Architecture::X8664 => "x86_64",
                    Architecture::X86 => return Err("x86 (32-bit) Windows is not supported".into()),
                };
                (
                    format!("{arch_str}-pc-windows-msvc.zip").leak() as &'static str,
                    DownloadedFileType::Zip,
                    ".exe",
                )
            }
        };

        let asset_name = format!("{BINARY_NAME}-{asset_suffix}");
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("No release asset found for {asset_name}"))?;

        let version_dir = format!("{BINARY_NAME}-{}", release.version);
        let binary_path = format!("{version_dir}/{BINARY_NAME}{binary_suffix}");

        if std::fs::metadata(&binary_path).map(|m| m.is_file()).unwrap_or(false) {
            self.cached_binary_path = Some(binary_path.clone());
            return Ok(binary_path);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::Downloading,
        );

        download_file(&asset.download_url, &version_dir, file_type)
            .map_err(|e| format!("Failed to download {}: {e}", asset.name))?;

        if platform != Os::Windows {
            make_file_executable(&binary_path)?;
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::None,
        );

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for CMakeExtension {
    fn new() -> Self {
        CMakeExtension {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let binary_path = worktree
            .which(BINARY_NAME)
            .or_else(|| self.language_server_binary_path(language_server_id).ok())
            .ok_or_else(|| {
                format!(
                    "Failed to install {BINARY_NAME}. Install manually from: https://github.com/{GITHUB_REPO}"
                )
            })?;

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
            "format": { "enable": true },
            "lint": { "enable": true },
            "scan_cmake_in_package": false,
            "semantic_token": false
        })))
    }
}

zed::register_extension!(CMakeExtension);
