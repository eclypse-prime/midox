use midly::{
    num::{u28, u7},
    MidiMessage, TrackEvent, TrackEventKind,
};

pub fn clean_track<'a>(track: &'a Vec<TrackEvent<'a>>, threshold: u8) -> Vec<TrackEvent> {
    let threshold = u7::new(threshold);

    let mut new_track: Vec<TrackEvent> = vec![];
    let mut last_delta: u28 = u28::new(0);
    let mut skip_off: bool = false;
    for event in track {
        last_delta += event.delta;

        match event.kind {
            TrackEventKind::Midi { channel, message } => match message {
                MidiMessage::NoteOff { key: _, vel: _ } => {
                    if !skip_off {
                        new_track.push(TrackEvent {
                            delta: last_delta,
                            kind: TrackEventKind::Midi { channel, message },
                        });
                        last_delta = u28::new(0);
                    } else {
                        skip_off = false;
                    }
                }
                MidiMessage::NoteOn { key: _, vel } => {
                    if vel <= threshold {
                        skip_off = true;
                    } else {
                        new_track.push(TrackEvent {
                            delta: last_delta,
                            kind: TrackEventKind::Midi { channel, message },
                        });
                        last_delta = u28::new(0);
                    }
                }
                MidiMessage::Aftertouch { key: _, vel: _ } => (),
                MidiMessage::Controller {
                    controller: _,
                    value: _,
                } => {
                    new_track.push(TrackEvent {
                        delta: last_delta,
                        kind: TrackEventKind::Midi { channel, message },
                    });
                    last_delta = u28::new(0);
                }
                MidiMessage::ProgramChange { program: _ } => {
                    new_track.push(TrackEvent {
                        delta: last_delta,
                        kind: TrackEventKind::Midi { channel, message },
                    });
                    last_delta = u28::new(0);
                }
                MidiMessage::ChannelAftertouch { vel: _ } => (),
                MidiMessage::PitchBend { bend: _ } => {
                    new_track.push(TrackEvent {
                        delta: last_delta,
                        kind: TrackEventKind::Midi { channel, message },
                    });
                    last_delta = u28::new(0);
                }
            },
            TrackEventKind::SysEx(_data) => {
                new_track.push(TrackEvent {
                    delta: last_delta,
                    kind: event.kind,
                });
                last_delta = u28::new(0);
            }
            TrackEventKind::Escape(_data) => {
                new_track.push(TrackEvent {
                    delta: last_delta,
                    kind: event.kind,
                });
                last_delta = u28::new(0);
            }
            TrackEventKind::Meta(_meta_message) => {
                new_track.push(TrackEvent {
                    delta: last_delta,
                    kind: event.kind,
                });
                last_delta = u28::new(0);
            }
        }
    }
    new_track
}
