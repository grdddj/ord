use super::*;

#[derive(Debug, Parser)]
pub(crate) struct MongoExport {
  #[arg(long, help = "Mongo connection string")]
  conn: String,
  #[arg(long, help = "DB to save")]
  db: String,
  #[arg(long, help = "Collection to save")]
  collection: String,
}

impl MongoExport {
  pub(crate) fn run(self, settings: Settings) -> SubcommandResult {
    let index = Index::open(&settings)?;

    index.update()?;
    index.export_mongo(self.conn, self.db, self.collection)?;

    Ok(None)
  }
}
