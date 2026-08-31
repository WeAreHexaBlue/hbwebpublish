use std::{ env, path };

use clap::{ Parser, Subcommand };
use serde::{ Serialize, Deserialize };
use dotenvy::dotenv;

mod commands;

#[derive(Parser)]
#[command(name = "hbpub", version = "1.0", about = "Tool to publish News on the HexaBlue website.")]
struct CLI {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, env = "PUBLISHER_TOKEN")]
    publisher_token: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    slug: String,
    locale: String,
    title: String,
    author: String,
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

    match cli.command {
        Commands::Init { dir } => commands::init(&dir)?,
        Commands::Publish { dir } => commands::publish(&dir)?,
        Commands::Delete { slug, locale } => commands::delete(slug, locale)?,
    }

    Ok(())
}