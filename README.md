# Vidascii::[Braille art converter]

#### ➡ converts image or video to braille art 🪄


https://github.com/Ilingu/vidascii/assets/57411599/6212b74a-71cc-4727-86da-f2d63db94ff4


## Origin

I was watching an anime, and in the comment section I saw yet another guy who posted a braille image.

From this point, I said to myself: "no way this is hand-made", so I done some research I found the poject interesting to make.

Thus I made it.

And voilà~~

## Purpose

1. Have fun
2. Crying at 2am because it doesn't work
3. Realizing that I'm just dumb
4. Improving my rust skills

> Like always, this is just a little side-project.

## Installation

> There is no guarantees that this CLI will works with anything other than linux

> [OPTIONAL]: you may want to install FFmpeg youself, if not installed this program will automatically installs it
>
> ```bash
> sudo apt install ffmpeg
> ```

### Via prebuilt binary (linux only)

you can go grab the prebuilt binary [here](https://github.com/Ilingu/vidascii/releases).

### Via "your idenpendent and not lazy so you build it yourself"

Build from source with `cargo`

```bash
cargo build --release # will create a single executable for your os in ./target/release, named "ilix_server" (with the associated executable extension in your os)
```

## Usage

```bash
vidascii --help # you're welcome 😁
```

## Limitations

This work great.

**BUT**:

1. I recommend you to disable the `dithering` option when you set the `ratio` to anything other than `1`.
2. This is not made to convert large video file. It'll works _(perhaps?)_ but you'll have to wait a **very** long time.
   > A simple gif up to a small video is ok though.

_Note that this project have room for a **lot** of optimisation. For now I don't want to spend too much time on this and I feel that I lack a lot of optimisation skills, so maybe oneday... 😺_

## Made with:

1. **Elegance** ✅
2. `RUST` ✨🦀
3. [image](https://docs.rs/image/latest/image/): _image processing_
4. [ffmpeg](https://ffmpeg.org/): _video processing_
5. _a lot of other existing crates..._
