use zed_extension_api as zed;

const SERVER_PATH: &str =
    "node_modules/@microsoft/compose-language-service/bin/docker-compose-langserver";
const PACKAGE_NAME: &str = "@microsoft/compose-language-service";

#[derive(Default)]
struct DockerComposeExtension {
    cached: bool,
}

impl DockerComposeExtension {
    fn server_path(&self) -> String {
        zed_ext::sanitize_windows_path(std::env::current_dir().unwrap())
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string()
    }

    fn install_if_needed(&mut self, language_server_id: &zed::LanguageServerId) -> zed::Result<()> {
        if self.cached {
            return Ok(());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let latest = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let current = zed::npm_package_installed_version(PACKAGE_NAME)?;

        if current.is_some_and(|current| current == latest) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::None,
            );

            return Ok(());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        if let Err(err) = zed::npm_install_package(PACKAGE_NAME, &latest) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Failed(err.clone()),
            );

            return Err(err);
        }

        self.cached = true;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        Ok(())
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
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        self.install_if_needed(language_server_id)?;

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![self.server_path(), "--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(DockerComposeExtension);

/// Extensions to the Zed extension API that have not yet stabilized.
mod zed_ext {
    /// Sanitizes the given path to remove the leading `/` on Windows.
    ///
    /// On macOS and Linux this is a no-op.
    ///
    /// This is a workaround for https://github.com/bytecodealliance/wasmtime/issues/10415.
    pub fn sanitize_windows_path(path: std::path::PathBuf) -> std::path::PathBuf {
        use zed_extension_api::{current_platform, Os};

        let (os, _arch) = current_platform();
        match os {
            Os::Mac | Os::Linux => path,
            Os::Windows => path
                .to_string_lossy()
                .to_string()
                .trim_start_matches('/')
                .into(),
        }
    }
}
