use clap::Parser;
use cli::{Cli, Commands};
use database::Database;

mod cli;
mod database;
mod enron_row;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let database = Database::open().await?;

    match cli.command {
        Commands::LoadCsv => enron_row::read_enron_csv(&database).await?,
        Commands::Search { query } => {
            let posts = database.search_posts(&query).await?;
            for post in posts {
                println!("FILE");
                println!("-------------");
                println!("{}", post.0.unwrap_or_else(|| "<NULL>".to_string()));
                println!("BODY");
                println!("-------------");
                println!("{}", post.1.unwrap_or_else(|| "<NULL>".to_string()));
                println!("=============");
            }
        }
    }

    Ok(())
}
