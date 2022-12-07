use std::{env, error, fs, process};

use chrono::prelude::*;

// The cycle used year round, until Christmas makes things difficult
const BIN_CYCLE: [&str; 4] = ["🟩⬛", "🟩🟦🟫", "🟩⬛", "🟩🟦"];

const COLLECTION_FILEPATH: &str = "./previous_collection.txt";

fn main() -> Result<(), Box<dyn error::Error>> {
    // Check what last week's collection was
    let previous_collection_index: usize = fs::read_to_string(COLLECTION_FILEPATH)?.parse()?;

    // Add one to it, or wrap back around to the start of the cycle if we were at the end
    let next_collection_index = (previous_collection_index + 1) % BIN_CYCLE.len();

    // Declare this var in an inner scope to avoid evil mutability leaking further than it needs to
    let next_collection = {
        let mut next_collection = BIN_CYCLE[next_collection_index].to_string();

        // Elegantly handle all of the differences during the Christmas period
        if Utc::now().ordinal() > 350 {
            next_collection += " 🎄 It's close to Christmas. This message may be incorrect!";
        };

        next_collection
    };

    // The ntfy topic ID is essentially a password, so it's kept secret! 🤫
    let topic_id = env::var("BIN_TOPIC")?;

    // This should probably use `reqwest` or similar really,
    // but this way is one less dependency to worry about
    process::Command::new("curl")
        .arg("-d")
        .arg(next_collection)
        .arg(format!("ntfy.sh/{topic_id}"))
        .output()?;

    // Write today's collection index, so that next time we read the previous
    // collection we see this one
    fs::write(COLLECTION_FILEPATH, next_collection_index.to_string())?;

    Ok(())
}
