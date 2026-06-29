use std::path::PathBuf;
use clap::{Parser, Subcommand};

trait RecorderBackend {
    fn start(&self, output_path: PathBuf) -> Result<(), String>;
}

// Search for a pattern in a file and display the lines that contain it
#[derive(Parser, Debug)]
#[command(name = "scrix", about = "Start or stop using a file path")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Start {
        path: PathBuf,
    },
    Stop,
}

fn start_recording(path: PathBuf) -> Result<(), String> {
    // find session type XDG_SESSION_TYPE
    // match session to backend

    let session = std::env::var("XDG_SESSION_TYPE");
    println!(session);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Start { path } => {
            println!("Started recording with file: {}", path.display());
            println!("Recording runs in the foreground. Press Ctrl+C to stop.")
        }
        Command::Stop => {
            println!("Stopping recording");
        }
    }
    
    return;
}
