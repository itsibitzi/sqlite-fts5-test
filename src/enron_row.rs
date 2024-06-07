use serde::{Deserialize, Serialize};

use crate::database::Database;

#[derive(Serialize, Deserialize)]
pub struct EnronRow {
    file: String,
    message: String,
}

pub async fn read_enron_csv(database: &Database) -> anyhow::Result<()> {
    let mut rdr = csv::Reader::from_reader(std::io::stdin());

    for result in rdr.deserialize() {
        let record: EnronRow = result?;

        // Very inefficient - opens a transaction per row
        database
            .insert_enron_email(&record.file, &record.message)
            .await?;
    }

    Ok(())
}
