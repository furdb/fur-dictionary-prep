use furdb::{FurColumn, FurDB, FurDBInfo, FurDataType, FurTableInfo};
use std::{collections::HashMap, error::Error, path::PathBuf};

static WORDS: &[(&str, &str)] = &[
    ("anomaly", "unusual"),
    ("equivocal", "ambiguous"),
    ("lucid", "clear"),
    ("opaque", "incomprehensible"),
    ("prodigal", "extravagant"),
    ("zeal", "enthusiasm"),
    ("laudable", "commendable"),
    ("vacillate", "waver"),
    ("loquacious", "talkative"),
    ("pragmatic", "practical"),
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    prepare_database().await?;

    Ok(())
}

async fn prepare_database() -> Result<(), Box<dyn Error>> {
    let db_path = PathBuf::from("D:\\Home\\Repositories\\FurDB\\TestDBs\\Dictionary");
    let db_info = FurDBInfo::new("Dictionary")?;

    let db = FurDB::new(db_path, Some(db_info))?;

    let word_column = FurColumn::new(
        "word",
        Some("Word"),
        80,
        FurDataType::new("long_string", None)?,
    )?;

    let table_info = FurTableInfo::new(
        "Dictionary",
        Some("http://localhost:5000"),
        Some(vec![
            word_column.clone(),
            FurColumn::new(
                "definition",
                Some("Definition"),
                240,
                FurDataType::new("long_string", None)?,
            )?,
        ]),
    )?;

    let mut table = db.get_table("dictionary", Some(table_info))?;

    let mut words = Vec::new();

    WORDS.iter().cloned().for_each(|(word, definition)| {
        words.push(HashMap::from([("word", word), ("definition", definition)]))
    });

    table.add(&words).await?;

    table.generate_sortfile(&word_column).await?;

    Ok(())
}
