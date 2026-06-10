use zed::serde_json;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, settings::LspSettings, Result};

const BINARY_NAME: &str = "intellij-server";

struct KotlinExtension;

impl zed::Extension for KotlinExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        if let Some(server_path) = worktree.which(BINARY_NAME) {
            Ok(zed::Command {
                command: server_path,
                args: vec!["--stdio".to_string()],
                env: Default::default(),
            })
        } else {
            Err("intellij-server not found".to_string())
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .map(|lsp_settings| lsp_settings.settings)
    }
}

zed::register_extension!(KotlinExtension);
