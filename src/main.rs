use furdb::{FurColumn, FurDB, FurDBInfo, FurDataType, FurTableInfo};
use std::{collections::HashMap, error::Error, path::PathBuf};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_path = PathBuf::from("D:\\Home\\Repositories\\FurDB\\TestDBs\\Dictionary");
    let db_info = FurDBInfo::new("Dictionary")?;

    let db = FurDB::new(db_path, Some(db_info))?;

    let table_info = FurTableInfo::new(
        "Dictionary",
        Some("http://localhost:5000"),
        Some(vec![
            FurColumn::new(
                "word",
                Some("Word"),
                80,
                FurDataType::new("long_string", None)?,
            )?,
            FurColumn::new(
                "definition",
                Some("Definition"),
                240,
                FurDataType::new("long_string", None)?,
            )?,
        ]),
    )?;

    let mut table = db.get_table("dictionary", Some(table_info))?;

    let words = [
        HashMap::from([("word", "7"), ("definition", "18")]),
        HashMap::from([("word", "7"), ("definition", "18")]),
        HashMap::from([("word", "7"), ("definition", "18")]),
        HashMap::from([("word", "7"), ("definition", "18")]),
        HashMap::from([("word", "7"), ("definition", "18")]),
    ];

    table.add(&words).await?;

    table.generate_all_sortfiles().await?;

    Ok(())
}
