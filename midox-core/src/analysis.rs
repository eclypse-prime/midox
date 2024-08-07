use std::collections::HashMap;

use midly::{MidiMessage, Smf, TrackEvent, TrackEventKind};

pub fn count_notes(smf: &Smf) -> usize {
    smf.tracks
        .iter()
        .map(|track| {
            track
                .iter()
                .filter(|event| {
                    matches!(
                        event.kind,
                        TrackEventKind::Midi {
                            channel: _,
                            message: MidiMessage::NoteOn { key: _, vel: _ },
                        }
                    )
                })
                .count()
        })
        .sum()
}

pub fn count_velocities(track: &[TrackEvent]) -> Vec<(u8, usize)> {
    let mut counts: HashMap<u8, usize> = HashMap::new();
    for event in track {
        let kind = &event.kind;
        if let TrackEventKind::Midi {
            channel: _,
            message: MidiMessage::NoteOn { key: _, vel },
        } = kind
        {
            let vel = vel.as_int();
            match counts.get(&vel) {
                Some(count) => {
                    counts.insert(vel, count + 1);
                }
                None => {
                    counts.insert(vel, 1);
                }
            }
        }
    }
    let mut counts: Vec<(u8, usize)> = counts.iter().map(|(&k, &v)| (k, v)).collect();
    counts.sort();
    counts
}
