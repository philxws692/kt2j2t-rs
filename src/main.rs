use std::io;
use anyhow::bail;
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator};
use kt2j2t_rs::kt2j2t;

#[derive(Parser, Debug)]
#[command(author, version, about, name = "kt2j2t")]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate shell completions
    Completion {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },

    /// Transform JSON schema
    Transform {
        #[arg(short, long)]
        /// File with JSON to transform
        input: String,

        #[arg(short, long)]
        /// File where transformed JSON should be outputted
        output: String,
    }
}

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(generator, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut cmd = Args::command();

    match args.command {
        Commands::Completion { shell } => {
            let _: () = print_completions(shell, &mut cmd);
            Ok(())
        },
        Commands::Transform { input, output } => {

            let input_file = std::fs::read_to_string(input)?;


            match kt2j2t(&input_file) {
                Ok(transformed_json) => {
                    println!("JSON was successfully transformed");
                    std::fs::write(
                        output,
                        serde_json::to_string_pretty(&transformed_json).unwrap(),
                    )?;

                    Ok(())
                }
                Err(e) => bail!("could not transform json: {}", e)
            }
        }
    }
}
