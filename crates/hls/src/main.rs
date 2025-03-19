use ffmpeg_next::{
    format::{self, context::Input},
    media, Packet, Rational,
};
use std::collections::{HashMap,};
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

use ffmpeg_next::codec;
use ffmpeg_next::format::context::Output;
use ffmpeg_next::media::Type;

type CreateStreamResult = Result<(usize, usize, Rational, Rational, Output), Box<dyn std::error::Error>>;
#[derive(Default)]
pub struct Inputs {
    ictxs: HashMap<String, Input>,
    streams: HashMap<(String, usize), StreamData>,
}

pub struct StreamData {
    create: Box<dyn Fn(&str) -> CreateStreamResult>,
    last_index: Option<String>,
}

impl Inputs {
    pub fn write_files(&mut self, name_links: HashMap<(String, usize), String>) -> Result<(), Box<dyn std::error::Error>> {
        //TODO: convert to write chunk instead of whole file
        let mut info_map = HashMap::new();
        for (info, stream) in &self.streams {
            let name = name_links.get(info).unwrap();
            let mut out = (stream.create)(name)?;
            out.4.write_header()?;
            info_map.insert(info.clone(), out);
        }
        for (key, ictx) in &mut self.ictxs {
            for (stream, mut packet) in ictx.packets() {
                if let Some((_, out_idx, in_time_base, out_time_base, octx)) = info_map.get_mut(&(key.clone(), stream.index())){
                    packet.set_stream(*out_idx);
                    packet.rescale_ts(*in_time_base, *out_time_base);
                    packet.write(octx)?;
                }
            }
        }

        for (_, (_, _, _,_, mut octx)) in info_map {
            octx.write_trailer()?;
        }
        Ok(())
    }
    pub fn replace(
        &mut self,
        inputs: Vec<(String, usize)>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (path, _) in &inputs {
            if !self.ictxs.contains_key(path) {
                let ictx = format::input(path)?;
                self.ictxs.insert(path.to_string(), ictx);
            }
        }
        for (path, stream_index) in &inputs {
            let stream = self
                .ictxs
                .get_mut(path)
                .unwrap()
                .stream(*stream_index)
                .unwrap();
            let codec = codec::context::Context::from_parameters(stream.parameters())?;
            let in_timebase = stream.time_base();
            match stream.parameters().medium() {
                Type::Video | Type::Audio => {
                    let stream_index = *stream_index;
                    let generate_video_audio = move |file: &str| {
                        let mut octx = format::output(&format!("{file}.ts"))?;
                        let mut out_stream = octx.add_stream_with(&codec)?;
                        out_stream.set_time_base(Rational::new(1, 90000));
                        Ok::<(usize, usize, Rational, Rational, Output), Box<dyn std::error::Error>>((
                            stream_index,
                            out_stream.index(),
                            in_timebase,
                            out_stream.time_base(),
                            octx,
                        ))
                    };
                    let name =  format!("stream_{stream_index}");
                    self.streams.insert((path.clone(), stream_index), StreamData {
                        create: Box::new(generate_video_audio),
                        last_index: None,
                    });
                }
                Type::Subtitle => {
                    //TODO:
                }
                _ => {}
            }
        }
        let input_paths = inputs.iter().map(|v| v.0.clone()).collect::<Vec<_>>();
        self.ictxs.retain(|key, _| input_paths.contains(key));
        self.streams.retain(|key, _| inputs.contains(key));
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ffmpeg_next::init().unwrap();
    let mut inputs = Inputs::default();
    inputs
        .replace(vec![
            ("/Users/frederik/movie_files/test.mkv".to_string(), 0),
            ("/Users/frederik/movie_files/test.mkv".to_string(), 1),
           // ("/Users/frederik/movie_files/test.mkv".to_string(), 2),
        ])
        .expect("TODO: panic message");
    inputs.write_files(vec![
        (("/Users/frederik/movie_files/test.mkv".to_string(), 0), "stream_video".to_string()),
        (("/Users/frederik/movie_files/test.mkv".to_string(), 1), "stream_audio".to_string()),
        // ("/Users/frederik/movie_files/test.mkv".to_string(), 2),
    ].into_iter().collect())?;
    Ok(())
}
