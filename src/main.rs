mod audio;
mod noise;
mod preset;
mod sound;

use sound::Sound;
use std::io::{self, Write};

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about = "An offline foreground ambient sound mixer", after_long_help = include_str!("../assets/README.md"))]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List available sounds
    Sounds,
    /// Play a blend until Ctrl-C (SOUND or SOUND:LEVEL, 0–100)
    Play {
        #[arg(required = true)]
        sounds: Vec<Sound>,
        #[arg(long, default_value_t = 50, value_parser = clap::value_parser!(u8).range(0..=100))]
        volume: u8,
    },
    /// Save, play, list or remove named blends
    Preset {
        #[command(subcommand)]
        command: PresetCommand,
    },
    /// Write a shell completion script to stdout
    Completions { shell: Shell },
}

#[derive(Subcommand)]
enum PresetCommand {
    Save {
        label: String,
        #[arg(required = true)]
        sounds: Vec<Sound>,
    },
    Play {
        label: String,
        #[arg(long, default_value_t = 50, value_parser = clap::value_parser!(u8).range(0..=100))]
        volume: u8,
    },
    List,
    Remove {
        label: String,
    },
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

fn main() {
    if let Err(error) = run(Cli::parse()) {
        eprintln!("noizen: {error}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let mut output = io::stdout().lock();
    match cli.command {
        Command::Sounds => writeln!(output, "{}", sound::IDS.join("\n"))?,
        Command::Play { sounds, volume } => audio::play(&sounds, volume)?,
        Command::Preset { command } => {
            let dir = preset::directory()?;
            match command {
                PresetCommand::Save { label, sounds } => preset::save(&dir, &label, sounds)?,
                PresetCommand::Play { label, volume } => {
                    audio::play(&preset::load(&dir, &label)?, volume)?
                }
                PresetCommand::List => {
                    for label in preset::list(&dir)? {
                        writeln!(output, "{label}")?;
                    }
                }
                PresetCommand::Remove { label } => preset::remove(&dir, &label)?,
            }
        }
        Command::Completions { shell } => {
            let mut script = Vec::new();
            clap_complete::generate(
                clap_complete::Shell::from(shell),
                &mut Cli::command(),
                "noizen",
                &mut script,
            );
            output.write_all(&script)?;
        }
    }
    Ok(())
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
    fn playback_arguments() {
        assert!(
            Cli::try_parse_from(["noizen", "play", "rain:70", "pink", "--volume", "80"]).is_ok()
        );
        assert!(Cli::try_parse_from(["noizen", "preset", "save", "cozy", "rain:70"]).is_ok());
        for args in [
            vec!["noizen", "play"],
            vec!["noizen", "play", "rain", "--volume", "101"],
            vec!["noizen", "play", "unknown"],
            vec!["noizen", "play", "rain", "--duration", "10"],
            vec!["noizen", "preset", "save", "cozy"],
        ] {
            assert!(Cli::try_parse_from(args).is_err());
        }
    }

    #[test]
    fn unsupported_shell_is_rejected() {
        assert!(Cli::try_parse_from(["noizen", "completions", "powershell"]).is_err());
    }
}
