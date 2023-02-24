mod data;

use data::stats::{CatCount, TitleType};
use flate2::bufread::GzDecoder;
use quick_xml::de::Deserializer;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::Writer;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let gz = BufReader::new(GzDecoder::new(BufReader::new(&file)));
    let mut reader = Reader::from_reader(gz);
    let mut buf = Vec::new();

    let mut junk_buf: Vec<u8> = Vec::new();
    let mut count = 0;
    let mut stats = HashMap::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!(
                "Error at position {}: {:?}",
                reader.buffer_position(),
                e
            ),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"release" => {
                        let release_bytes = read_to_end_into_buffer(
                            &mut reader,
                            &e,
                            &mut junk_buf
                        ).unwrap();
                        let str = std::str::from_utf8(&release_bytes)
                            .unwrap();
                        let mut deserializer = Deserializer::from_str(str);
                        let release = Release::deserialize(&mut deserializer)
                            .unwrap();
                        process_release(&release, &mut stats);
                        count += 1;
                        if count % 1_000_000 == 0 {
                            println!("checked {} records", count);
                        }
                    }
                    _ => (),
                }
            }
            // other unimportant Events
            _ => (),
        }
        // clear buffer to avoid leaking memory
        buf.clear();
    }

    println!("checked {} records", count);

    let mut data: Vec<(&String, &CatCount)> = stats.iter().collect();
    data.sort_by_key(|obj| obj.0);

    println!("Stats: {:#?}", data);

    Result::Ok(())
}

fn process_release(release: &Release, stats: &mut HashMap<String, CatCount>) {
    // todo
    if !release.data_quality.data_quality.is_acceptable() {
        return;
    }
    let is_main_release = release
        .master_id
        .as_ref()
        .and_then(|m| m.is_main_release)
        .unwrap_or_default();
    if !is_main_release {
        return;
    }
    let release_month = release
        .release_date
        .as_ref()
        .and_then(|s| valid_release_month(s));
    let Some(release_month) = release_month else {
        return;
    };

    // now we can do stats
    for track in &release.tracklist.track {
        let cat = categorize_track(track);
        // if cat == TitleType::LowercaseOnly {
        //     println!("{:?}: {:?}", cat, track.title);
        // }
        stats
            .entry(release_month.clone())
            .or_insert(CatCount::new())
            .increment(cat)
    }
}

fn valid_release_month(date_str: &str) -> Option<String> {
    let components: Vec<_> = date_str.split('-').collect();
    if components.len() >= 2 {
        Some(components[0..2].join("-"))
    } else {
        None
    }
}

fn categorize_track(track: &Track) -> TitleType {
    let mut seen_uppercase = false;
    let mut seen_lowercase = false;
    for chr in track.title.chars() {
        if chr.is_lowercase() {
            seen_lowercase = true;
        }
        if chr.is_uppercase() {
            seen_uppercase = true;
        }
        if seen_lowercase && seen_uppercase {
            break;
        }
    }
    match (seen_lowercase, seen_uppercase) {
        (true, true) => TitleType::MixedCase,
        (true, false) => TitleType::LowercaseOnly,
        (false, true) => TitleType::UppercaseOnly,
        (false, false) => TitleType::SomethingElse,
    }
}

fn read_to_end_into_buffer<R: BufRead>(
    reader: &mut Reader<R>,
    start_tag: &BytesStart,
    junk_buf: &mut Vec<u8>,
) -> Result<Vec<u8>, quick_xml::Error> {
    let mut depth = 0;
    let mut output_buf: Vec<u8> = Vec::new();
    let mut w = Writer::new(&mut output_buf);
    let tag_name = start_tag.name();
    w.write_event(Event::Start(start_tag.clone()))?;
    loop {
        junk_buf.clear();
        let event = reader.read_event_into(junk_buf)?;
        w.write_event(&event)?;

        match event {
            Event::Start(e) if e.name() == tag_name => depth += 1,
            Event::End(e) if e.name() == tag_name => {
                if depth == 0 {
                    return Ok(output_buf);
                }
                depth -= 1;
            }
            Event::Eof => {
                panic!("oh no")
            }
            _ => {}
        }
    }
}

#[derive(Deserialize, Debug)]
struct Release {
    #[serde(rename = "released")]
    release_date: Option<String>,
    data_quality: AnyDataQuality,
    tracklist: TrackList,
    master_id: Option<MasterId>,
}

#[derive(Deserialize, Debug)]
struct TrackList {
    #[serde(default)]
    track: Vec<Track>,
}

#[derive(Deserialize, Debug)]
struct Track {
    title: String,
}

#[derive(Deserialize, Debug)]
struct MasterId {
    #[serde(rename = "@is_main_release")]
    is_main_release: Option<bool>,
}

#[derive(Deserialize, PartialEq, Debug)]
struct AnyDataQuality {
    #[serde(rename = "$value")]
    data_quality: DataQuality,
}

#[derive(Deserialize, PartialEq, Debug, Clone, Copy)]
enum DataQuality {
    Correct,
    #[serde(rename = "Complete and Correct")]
    CompleteAndCorrect,
    #[serde(rename = "Needs Vote")]
    NeedsVote,
    #[serde(rename = "Needs Minor Changes")]
    NeedsMinorChanges,
    #[serde(rename = "Needs Major Changes")]
    NeedsMajorChanges,
    #[serde(rename = "Entirely Incorrect")]
    EntirelyIncorrect,
}

impl DataQuality {
    fn is_acceptable(&self) -> bool {
        // match &self {
        //     Self::Correct | Self::CompleteAndCorrect => true,
        //     _ => false,
        // }
        *self != DataQuality::EntirelyIncorrect
    }
}
