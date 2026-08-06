use std::fs;

use zed_extension_api::{self as zed, serde_json, LanguageServerId, Result};

const SERVER_REPO: &str = "docker/docker-language-server";
const BINARY_NAME: &str = "docker-language-server";

#[derive(Default)]
struct DockerComposeExtension {
    cached_binary_path: Option<String>,
}

impl DockerComposeExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which(BINARY_NAME) {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            SERVER_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (os, arch) = zed::current_platform();

        let asset_prefix = format!(
            "{BINARY_NAME}-{os}-{arch}-",
            os = match os {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "windows",
            },
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X8664 => "amd64",
                zed::Architecture::X86 => return Err("unsupported architecture: x86".into()),
            },
        );

        // Match on the platform prefix rather than rebuilding the whole asset
        // name, which ends in the release version in a form we'd have to guess.
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name.starts_with(&asset_prefix))
            .ok_or_else(|| {
                format!(
                    "no asset matching {asset_prefix}* in {SERVER_REPO} {}",
                    release.version
                )
            })?;

        // Windows needs the .exe suffix to execute the binary at all, so the
        // version lives in the file name rather than a containing directory.
        let extension = match os {
            zed::Os::Windows => ".exe",
            _ => "",
        };
        let binary_path = format!("{BINARY_NAME}-{}{extension}", release.version);

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &binary_path,
                zed::DownloadedFileType::Uncompressed,
            )?;
            zed::make_file_executable(&binary_path)?;

            for entry in fs::read_dir(".").into_iter().flatten().flatten() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if name.starts_with(BINARY_NAME) && name != binary_path {
                    fs::remove_file(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());

        Ok(binary_path)
    }
}

impl zed::Extension for DockerComposeExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec!["start".to_string(), "--stdio".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(serde_json::json!({
            "telemetry": "off",
            "dockercomposeExperimental": {
                "composeSupport": true,
            },
        })))
    }
}

zed::register_extension!(DockerComposeExtension);
