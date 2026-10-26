use std::io::{self, Write};

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about = "A lightweight ambient sound mixer (scaffold)")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List sounds planned for the MVP; playback is not implemented yet
    Sounds,
    /// Write a shell completion script to stdout
    Completions { shell: Shell },
}

#[derive(Clone, Copy, ValueEnum)]
enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl From<Shell> for clap_complete::Shell {
    fn from(shell: Shell) -> Self {
        match shell {
            Shell::Bash => Self::Bash,
            Shell::Zsh => Self::Zsh,
            Shell::Fish => Self::Fish,
        }
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mut output = io::stdout().lock();
    match cli.command {
        Command::Sounds => writeln!(
            output,
            "Planned sounds (playback not implemented):\n\
             white\npink\nbrown\nblue\nviolet\ngreen\nrain\nfireplace"
        ),
        Command::Completions { shell } => {
            let mut script = Vec::new();
            clap_complete::generate(
                clap_complete::Shell::from(shell),
                &mut Cli::command(),
                "noizen",
                &mut script,
            );
            output.write_all(&script)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_definitions_are_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn supported_shells_generate_scripts() {
        for name in ["bash", "zsh", "fish"] {
            let cli = Cli::try_parse_from(["noizen", "completions", name]).unwrap();
            let Command::Completions { shell } = cli.command else {
                panic!("expected completions");
            };
            let mut script = Vec::new();
            clap_complete::generate(
                clap_complete::Shell::from(shell),
                &mut Cli::command(),
                "noizen",
                &mut script,
            );
            assert!(String::from_utf8(script).unwrap().contains("noizen"));
        }
    }

    #[test]
    fn unsupported_shell_is_rejected() {
        assert!(Cli::try_parse_from(["noizen", "completions", "powershell"]).is_err());
    }
}
