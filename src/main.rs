use anyhow::anyhow;
use clap::Parser;
use std::env;
use std::fs;
use std::io::IsTerminal;
use std::io::Read;
use std::io::{self, Write};
use std::process::{exit, Command, Stdio};
use tempfile::Builder;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// Gets the editor using the same logic as Git.
/// Checks GIT_EDITOR, VISUAL, EDITOR environment variables in order,
/// then falls back to git's core.editor config, and finally defaults to vi.
fn get_editor() -> String {
    if let Ok(editor) = env::var("GIT_EDITOR") {
        return editor;
    }
    if let Ok(editor) = env::var("VISUAL") {
        return editor;
    }
    if let Ok(editor) = env::var("EDITOR") {
        return editor;
    }

    let mut cmd = Command::new("git");
    cmd.args(["config", "--get", "core.editor"]);
    if let Ok(out) = cmd.output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !s.is_empty() {
                return s;
            }
        }
    }

    String::from("vi")
}

/// Launches an editor with optional initial content and returns the edited content
fn launch_editor(args: Args, input: &str) -> anyhow::Result<String> {
    let extension = args.extension;

    let temp_file_suffix = if extension.starts_with('.') {
        extension
    } else {
        format!(".{}", extension)
    };

    // Create a temp file with the specified extension
    let mut temp_file = Builder::new()
        .prefix(&format!("{}-", APP_NAME))
        .suffix(&temp_file_suffix)
        .tempfile()?;
    let tmppath = temp_file.path().to_path_buf();

    // Write stdin content
    if !input.is_empty() {
        temp_file.write_all(input.as_bytes())?;
        temp_file.flush()?;
    }

    let editor = get_editor();

    let abs_path = fs::canonicalize(&tmppath).unwrap_or_else(|_| tmppath.clone());

    // Launch the editor this shell (NOTE:Unsupported Windows)
    let mut cmd = Command::new("sh");
    cmd.args(["-c", &format!("{} {}", editor, abs_path.display())]);

    // If stdout is piped (not a TTY), redirect stdin/stdout/stderr to /dev/tty
    // so the editor can still interact with the terminal
    if !io::stdout().is_terminal() {
        let tty = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/tty")?;
        // Clone file handle for stdin and stdout, use original for stderr
        let stdin_tty = tty.try_clone()?;
        let stdout_tty = tty.try_clone()?;

        cmd.stdin(Stdio::from(stdin_tty));
        cmd.stdout(Stdio::from(stdout_tty));
        cmd.stderr(Stdio::from(tty));
    }

    let status = cmd.status()?;

    if !status.success() {
        return Err(anyhow!("Editor failed with exit code: {:?}", status.code()));
    }

    // Read edited temp file contents
    let content = fs::read_to_string(&tmppath)?;

    if content.is_empty() {
        return Err(anyhow!("aborting due to empty message"));
    }

    Ok(content)
}

#[derive(Parser, Debug)]
#[command(name = "editin", version)]
struct Args {
    #[arg(
        short = 'e',
        long = "extension",
        help = "File extension that the editor opens",
        long_help = "
        Sets file extension that the editor opens.
        Editor will try to open with this file extension.
        ",
        default_value = "txt"
    )]
    extension: String,
}

fn main() {
    let args = Args::parse();

    // receive stdin
    let mut input = String::new();
    if !io::stdin().is_terminal() {
        if let Err(e) = io::stdin().read_to_string(&mut input) {
            eprintln!("Error reading stdin: {}", e);
            exit(1);
        }
    }

    // Launch editor and get content
    match launch_editor(args, &input) {
        Ok(content) => {
            print!("{}", content)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}
