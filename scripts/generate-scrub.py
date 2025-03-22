import os
import sys
import subprocess
import struct
import tempfile
from PIL import Image

def extract_frames(input_video, output_folder, width=640):
    """
    Extracts frames from a video at a set interval using ffmpeg.

    It took me 20k images to figure out that fps in "-vf" breaks the unique frames. I hate ffmpeg!!
    Im pretty sure this isnt even a bif file anymore but my own format that is somewhat inspired by bif but i dont really care anymore
    """
    os.makedirs(output_folder, exist_ok=True)
    ffmpeg_cmd = [
            "ffmpeg",
            "-r", "1",
            "-i", input_video,
            "-sn",
            "-an",
            "-vsync", "vfr",
            "-q:v", "2",
            "-vf", f"select='eq(pict_type\\,I)',scale=w={width}:-1",
            "-frame_pts", "true",
            os.path.join(output_folder, "%16d.jpg")
    ]
    subprocess.run(ffmpeg_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

def create_sprites(image_folder, sprite_folder, sprite_size=30):
    images = sorted(
        [f for f in os.listdir(image_folder)
         if f.endswith(".jpg") and os.path.getsize(os.path.join(image_folder, f)) >= 2 * 1024]
    )
    os.makedirs(sprite_folder, exist_ok=True)
    sprite_files = {}

    for i in range(0, len(images), sprite_size):
        batch = images[i:i + sprite_size]
        first_image = Image.open(os.path.join(image_folder, batch[0]))
        width, height = first_image.size

        cols = 5
        rows = (len(batch) + cols - 1) // cols
        sprite = Image.new("RGB", (cols * width, rows * height))

        for idx, img_file in enumerate(batch):
            img = Image.open(os.path.join(image_folder, img_file))
            x = (idx % cols) * width
            y = (idx // cols) * height
            sprite.paste(img, (x, y))

        sprite_path = os.path.join(sprite_folder, f"sprite_{i // sprite_size:06d}.jpg")
        sprite.save(sprite_path, "JPEG")

        sprite_files[sprite_path] = [int(img_file.split('.')[0]) for img_file in batch]

    return sprite_files


def create_bif(folder, sprites, output_bif, interval=30000):
    """Creates a .bif file from extracted images."""
    with open(output_bif, "wb") as bif:
        bif.write(b"BIF\x00\x00\x00\x00\x00")
        bif.write(struct.pack("<I", 0x0100))
        bif.write(struct.pack("<I", len(sprites)))
        bif.write(struct.pack("<I", interval))
        bif.write(b"\x00" * 44)

        bif.write(struct.pack("<Q", 32*4))

        for path, pts in sprites.items():
            with open(os.path.join(folder, path), "rb") as img:
                bytes = img.read();
                bif.write(struct.pack("<I", len(bytes)))
                bif.write(struct.pack("<I", len(pts)))
                for j in range(0, 30):
                    # todo: convert pts to ms
                    bif.write(struct.pack("<I", pts[j] if j < len(pts) else 0))
                bif.write(bytes)

        bif.write(struct.pack("<I", 0xFFFFFFFF))


def main():
    if len(sys.argv) != 3:
        print("Usage: python generate_bif.py <input_video> <output_bif>")
        sys.exit(1)

    input_video = sys.argv[1]
    output_bif = sys.argv[2]

    with tempfile.TemporaryDirectory() as temp_dir:
        frames_dir = os.path.join(temp_dir, "frames")
        sprites_dir = os.path.join(temp_dir, "sprites")

        extract_frames(input_video, frames_dir)
        sprites = create_sprites(frames_dir, sprites_dir)
        create_bif(sprites_dir, sprites, output_bif)

if __name__ == "__main__":
    main()
