use std::time::Instant;

use midly::Smf;
use midox_core::{analysis::count_notes, editing::clean_track};

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    // Load bytes first
    let data = std::fs::read("song.mid").unwrap();

    // Parse the raw bytes
    let smf = Smf::parse(&data).unwrap();

    println!("Loading time: {:?}", start.elapsed());

    println!("The file has {} tracks!", smf.tracks.len());
    println!("The file has {} notes!", count_notes(&smf));

    let mut new_smf = Smf::new(
        smf.header,
    );

    let start = Instant::now();
    for track in smf.tracks {
        new_smf.tracks.push(clean_track(track, 1));
    }

    println!("Processing time: {:?}", start.elapsed());

    let _ = new_smf.save("new_song.mid");
    println!("The new file has {} notes!", count_notes(&new_smf));

    Ok(())
}
