use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use flowdraft::ProcessOptions;

#[derive(Parser)]
#[command(name = "flowdraft", about = "Render diagrams from Lisp-style DSL to SVG")]
struct Cli {
    /// Input .fd file (reads from stdin if omitted)
    input: Option<PathBuf>,

    /// Output SVG file (writes to stdout if omitted)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Disable line-aware child ordering in flow layout
    #[arg(long)]
    no_line_aware: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Watch a file and re-render on changes
    Watch {
        /// Input .fd file to watch
        input: PathBuf,
        /// Output SVG file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Watch { input, output }) => {
            let opts = ProcessOptions { no_line_aware: cli.no_line_aware };
            watch_mode(&input, output.as_deref(), &opts)?
        }
        None => {
            let content = read_input(cli.input.as_deref())?;
            let opts = ProcessOptions { no_line_aware: cli.no_line_aware };
            let svg = flowdraft::process_with_options(&content, &opts)?;
            write_output(cli.output.as_deref(), &svg)?;
        }
    }

    Ok(())
}

fn read_input(path: Option<&std::path::Path>) -> Result<String> {
    match path {
        Some(p) => fs::read_to_string(p).with_context(|| format!("failed to read {}", p.display())),
        None => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .context("failed to read stdin")?;
            Ok(buf)
        }
    }
}

fn write_output(path: Option<&std::path::Path>, content: &str) -> Result<()> {
    match path {
        Some(p) => {
            fs::write(p, content).with_context(|| format!("failed to write {}", p.display()))
        }
        None => {
            print!("{}", content);
            Ok(())
        }
    }
}

fn watch_mode(input: &std::path::Path, output: Option<&std::path::Path>, opts: &ProcessOptions) -> Result<()> {
    use std::sync::mpsc;
    use std::time::Duration;

    let output_path = output.map(|p| p.to_path_buf()).unwrap_or_else(|| {
        input.with_extension("svg")
    });

    // Initial render
    let content = fs::read_to_string(input)?;
    match flowdraft::process_with_options(&content, opts) {
        Ok(svg) => {
            fs::write(&output_path, &svg)?;
            eprintln!("Rendered → {}", output_path.display());
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    let (tx, rx) = mpsc::channel();
    let mut debouncer = notify_debouncer_mini::new_debouncer(Duration::from_millis(200), tx)?;
    debouncer
        .watcher()
        .watch(input, notify::RecursiveMode::NonRecursive)?;

    eprintln!("Watching {} for changes...", input.display());

    loop {
        match rx.recv() {
            Ok(Ok(_events)) => {
                let content = fs::read_to_string(input)?;
                match flowdraft::process_with_options(&content, opts) {
                    Ok(svg) => {
                        fs::write(&output_path, &svg)?;
                        eprintln!("Rendered → {}", output_path.display());
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Ok(Err(e)) => eprintln!("Watch error: {:?}", e),
            Err(e) => {
                eprintln!("Channel error: {}", e);
                break;
            }
        }
    }

    Ok(())
}
