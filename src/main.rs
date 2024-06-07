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
            let emails = database.search_enron_emails(&query).await?;
            for email in emails {
                println!("FILE");
                println!("-------------");
                println!("{}", email.0.unwrap_or_else(|| "<NULL>".to_string()));
                println!("BODY");
                println!("-------------");
                println!("{}", email.1.unwrap_or_else(|| "<NULL>".to_string()));
                println!("=============");
            }
        }
    }

    Ok(())
}
