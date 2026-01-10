use std::env;
use std::process::Command;

pub trait GitConfigEditor {
    fn get(&self) -> Option<String>;
}

pub struct ActualGitConfig;

impl GitConfigEditor for ActualGitConfig {
    fn get(&self) -> Option<String> {
        let out = Command::new("git")
            .args(["config", "--get", "core.editor"])
            .output()
            .ok()?;
        if !out.status.success() {
            return None;
        }
        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if s.is_empty() {
            return None;
        }
        Some(s)
    }
}

/// Gets the editor using the same logic as Git.
/// Checks GIT_EDITOR, VISUAL, EDITOR environment variables in order,
/// then falls back to git's core.editor config, and finally defaults to vi.
pub fn get_editor(config: &dyn GitConfigEditor) -> String {
    if let Ok(editor) = env::var("GIT_EDITOR") {
        if !editor.is_empty() {
            return editor;
        }
    }
    if let Ok(editor) = env::var("VISUAL") {
        if !editor.is_empty() {
            return editor;
        }
    }
    if let Ok(editor) = env::var("EDITOR") {
        if !editor.is_empty() {
            return editor;
        }
    }

    if let Some(editor) = config.get() {
        return editor;
    }

    String::from("vi")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    struct MockGitConfig {
        pub expected: Option<String>,
    }
    impl GitConfigEditor for MockGitConfig {
        fn get(&self) -> Option<String> {
            self.expected.clone()
        }
    }

    /// Helper function to clear all editor-related environment variables
    fn clear_editor_env_vars() {
        env::remove_var("GIT_EDITOR");
        env::remove_var("VISUAL");
        env::remove_var("EDITOR");
    }

    #[test]
    #[serial]
    fn test_get_editor_to_be_env_git_editor() {
        clear_editor_env_vars();
        let mock = MockGitConfig {
            expected: Some("test-editor".to_string()),
        };

        env::set_var("GIT_EDITOR", "emacs");
        env::set_var("VISUAL", "nano");
        env::set_var("EDITOR", "vim");

        let editor = get_editor(&mock);
        assert_eq!(editor, "emacs");

        clear_editor_env_vars();
    }

    #[test]
    #[serial]
    fn test_get_editor_to_be_env_visual() {
        clear_editor_env_vars();
        let mock = MockGitConfig {
            expected: Some("test-editor".to_string()),
        };

        env::set_var("VISUAL", "nano");
        env::set_var("EDITOR", "vim");

        let editor = get_editor(&mock);
        assert_eq!(editor, "nano");

        clear_editor_env_vars();
    }

    #[test]
    #[serial]
    fn test_get_editor_to_be_env_editor() {
        clear_editor_env_vars();
        let mock = MockGitConfig {
            expected: Some("test-editor".to_string()),
        };

        env::set_var("EDITOR", "vim");

        let editor = get_editor(&mock);
        assert_eq!(editor, "vim");

        clear_editor_env_vars();
    }

    #[test]
    #[serial]
    fn test_get_editor_to_be_git_config() {
        clear_editor_env_vars();
        let mock = MockGitConfig {
            expected: Some("test-editor".to_string()),
        };

        let editor = get_editor(&mock);
        assert_eq!(editor, "test-editor");

        clear_editor_env_vars();
    }

    #[test]
    #[serial]
    fn test_get_editor_fallback_to_vi() {
        clear_editor_env_vars();
        let mock = MockGitConfig { expected: None };

        let editor = get_editor(&mock);
        // Should fall back to "vi" when no editor is configured
        assert_eq!(editor, "vi");

        clear_editor_env_vars();
    }

    #[test]
    #[serial]
    fn test_get_editor_with_empty_env_var() {
        clear_editor_env_vars();
        let mock = MockGitConfig { expected: None };

        env::set_var("GIT_EDITOR", "");
        env::set_var("VISUAL", "nano");

        let editor = get_editor(&mock);
        // Empty string should not be used, should fall through to VISUAL
        assert_eq!(editor, "nano");

        clear_editor_env_vars();
    }
}
