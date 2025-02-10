use prost::Message;
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/data.rs"));
}

#[derive(Serialize, Deserialize)]
struct Info {
    text: String,
    season: i32,
    episode: i32,
    frame_start: i32,
    frame_prefer: i32,
    frame_end: i32,
    segment_id: i32,
}

#[derive(Serialize, Deserialize)]
struct Data {
    info: Vec<Info>,
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let info_vec: Vec<Info> = serde_json::from_str(&buffer).expect("Failed to parse JSON");

    let mut proto_data = protos::Data::default();
    for info in info_vec {
        proto_data.info.push(protos::Info {
            text: info.text,
            season: info.season,
            episode: info.episode,
            frame_start: info.frame_start,
            frame_prefer: info.frame_prefer,
            frame_end: info.frame_end,
            segment_id: info.segment_id,
        });
    }

    let mut buf = Vec::new();
    proto_data.encode(&mut buf).expect("Failed to encode protobuf");

    io::stdout().write_all(&buf)?;

    Ok(())
}
