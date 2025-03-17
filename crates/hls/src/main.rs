use ffmpeg_next::{format, media, Dictionary, Rational};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ffmpeg_next::init().unwrap();

    let mut ictx = format::input(&"/Users/frederik/movie_files/file.mkv")?;

    let mut octx = format::output(&"output.m3u8")?;

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
