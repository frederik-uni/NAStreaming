# Streaming Concept

### Protocol: Hls
## Goals
- on the fly transcoding
  - live stream hls feature
- fast stream change
  - seperate streams
  - live stream hls feature
- fast reconnect on disconect
  - cache the data in memory, ssd
  - slice video before transcoding

## Ideas
- select specifc streams
- seperate audio, video, subtitle streams
- video live stream function
- seperate into slow(hdd), fast(ssd), veryfast(memory) cache

## Questions
- finish processing of audio stream before serving or live stream
- subitlte stream as vector graphics or srt&font files
- How to start in the middle of a video? slice video & if before: delete old + new stream with different start?!
- how long to cache, propcessing...
- When load data into/out of which cache
- What transcoding options should be used
