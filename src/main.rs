use std::path;

use clap::{ Parser, Subcommand };
use dotenvy::dotenv;

mod commands;

#[derive(Parser)]
#[command(name = "hbpub", version = "1.0", about = "Tool to publish News on the HexaBlue website.")]
struct CLI {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true, env = "PUBLISHER_TOKEN")]
    publisher_token: Option<String>
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(default_value = ".")]
        dir: path::PathBuf
    },
    Publish {
        #[arg(default_value = ".")]
        dir: path::PathBuf
    },
    Delete {
        slug: String,
        locale: String
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let cli = CLI::parse();

    let require_token = || {
        cli.publisher_token.ok_or("Missing publisher token. Provide -p or set PUBLISHER_TOKEN in .env")
    };

    match cli.command {
        Commands::Init { dir } => commands::init(&dir)?,
        Commands::Publish { dir } => {
            let token = require_token()?;
            commands::publish(&dir, &token)?;
        },
        Commands::Delete { slug, locale } => {
            let token = require_token()?;
            commands::delete(slug, locale, &token)?;
        },
    }

    Ok(())
}