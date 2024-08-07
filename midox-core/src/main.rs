use std::time::Instant;

use anyhow::Result;
use midly::Smf;
use midox_core::{analysis::count_notes, editing::clean_track, io::bytes_to_smf};

fn main() -> Result<()> {
    let start = Instant::now();

    // Load bytes first
    let data = std::fs::read("song.mid")?;

    let smf = bytes_to_smf(&data)?;

    println!("Loading time: {:?}", start.elapsed());

    println!("The file has {} tracks!", smf.tracks.len());
    println!("The file has {} notes!", count_notes(&smf));

    let mut new_smf = Smf::new(smf.header);

    let start = Instant::now();
    new_smf.tracks = smf
        .tracks
        .iter()
        .map(|track| clean_track(track, 1))
        .collect();

    println!("Processing time: {:?}", start.elapsed());

    let _ = new_smf.save("new_song.mid");
    println!("The new file has {} notes!", count_notes(&new_smf));

    Ok(())
}
