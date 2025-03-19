use ffmpeg_next::{
    format::{self, context::Input},
    media, Dictionary, Packet, Rational,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

pub struct Map {
    map: BTreeMap<i64, Packet>,
    done: bool,
}

impl Map {
    fn insert(&mut self, key: i64, value: Packet) -> Option<Packet> {
        self.map.insert(key, value)
    }
    fn done(&mut self) {
        self.done = true;
    }
}

fn generate_map(mut ictx: Input, map: Arc<Mutex<Map>>) {
    let mut last_timestamp = None;
    let mut last_packet = Packet::empty();
    for (stream, packet) in ictx.packets() {
        if last_timestamp.is_none() {
            last_timestamp = Some(0.0);
            map.lock().unwrap().insert(0, last_packet.clone());
        } else if let Some(pts) = packet.pts() {
            let time_base = stream.time_base();
            let pts_in_seconds = (time_base.0 as i64 * pts) as f64 / time_base.1 as f64;
            if pts_in_seconds >= last_timestamp.unwrap() + 2.0 {
                map.lock().unwrap().insert(pts, last_packet.clone());
                last_timestamp = Some(pts_in_seconds);
            }
        }
        last_packet = packet;
    }

    map.lock().unwrap().done();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ffmpeg_next::init().unwrap();

    let mut ictx = format::input(&"/Users/frederik/movie_files/test.mkv")?;
    let mut octx = format::output("output.ts")?;
    let mut stream_map = Vec::new();
    for (i, istream) in ictx.streams().enumerate() {
        if istream.parameters().medium() == media::Type::Video {
            let codec =
                ffmpeg_next::codec::context::Context::from_parameters(istream.parameters())?;
            let mut out_stream = octx.add_stream_with(&codec)?;
            out_stream.set_time_base(Rational::new(1, 90000));
            stream_map.push((
                i,
                out_stream.index(),
                istream.time_base(),
                out_stream.time_base(),
            ));
        }
    }
    let mut options = Dictionary::new();
    options.set("hls_list_size", "0");
    options.set("hls_segment_type", "mpegts");
    options.set("hls_segment_filename", "/tmp/memdisk-once/segment_%03d.ts");
    octx.write_header_with(options)?;

    for (stream, mut packet) in ictx.packets() {
        if let Some(&(in_idx, out_idx, in_time_base, out_time_base)) =
            stream_map.iter().find(|(i, _, _, _)| *i == stream.index())
        {
            packet.set_stream(out_idx);

            packet.rescale_ts(in_time_base, out_time_base);

            packet.write(&mut octx)?;
        }
    }

    octx.write_trailer()?;
    Ok(())
}
