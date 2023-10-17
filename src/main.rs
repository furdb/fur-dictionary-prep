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
    // prepare_converter().await?;
    prepare_database().await?;

    Ok(())
}

#[allow(dead_code)]
async fn prepare_converter() -> Result<(), Box<dyn Error>> {
    let mut count = HashMap::<char, i32>::new();

    for (word, definition) in WORDS {
        for letter in word.chars() {
            *count.entry(letter).or_insert(0) += 1;
        }

        for letter in definition.chars() {
            *count.entry(letter).or_insert(0) += 1;
        }
    }

    let mut frequencies: Vec<_> = count.iter().collect();
    frequencies.sort_by(|a, b| a.1.cmp(b.1).reverse());

    println!("{}", count.len());
    for f in frequencies {
        println!("{}: {}", f.0, f.1);
    }

    Ok(())
}

async fn prepare_database() -> Result<(), Box<dyn Error>> {
    let db_path = PathBuf::from("/furdb/Dictionary");
    let db_info = FurDBInfo::new("Dictionary")?;

    let db = FurDB::new(db_path, Some(db_info))?;

    let word_column = FurColumn::new("word", Some("Word"), 44, FurDataType::new("word", None)?)?;

    let table_info = FurTableInfo::new(
        "Dictionary",
        Some("http://localhost:5000"),
        Some(vec![
            word_column.clone(),
            FurColumn::new(
                "definition",
                Some("Definition"),
                76,
                FurDataType::new("word", None)?,
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
