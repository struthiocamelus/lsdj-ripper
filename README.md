# LSDJ-Ripper

## Introduction

```
Command line tool to rip wav files from LSDJ SRAM dumps

Usage: lsdj-ripper [OPTIONS] -l <lsdj> <input>

Arguments:
  <input>  Save file or lsdsng to rip from

Options:
  -l <lsdj>                        LSDJ rom [env: LSDJ_ROM=/Users/operator/Downloads/lsdj9_4_0-stable/lsdj9_4_0.gb]
  -o <outfile>                     Output filename [default: out.wav]
  -s, --sample-rate <sample-rate>  [default: 48000]
  -t, --time <length>              Length in seconds to record [default: 120]
  -c, --screen-capture             Capture the screen after loading the song
  -h, --help                       Print help
  -V, --version                    Print version
```

If you specify an `.lsdsng` file, it will be loaded into the active workspace and played from the start.

If you specify a `.sav` file, the first saved song in your `.sav` will be loaded.

This process is non-mutating, so your `.sav` file will still be good afterwards.

## Installation

1. Have Cargo.
   1. On most systems you can use [rustup](https://rustup.rs/)
   2. On others, it will come with your vendored Rust distribution package.
2. Install using Cargo:

```bash
cargo install --git https://github.com/struthiocamelus/lsdj-ripper
```

3. Ensure that `.cargo/bin` is in your `PATH`.

## Usage

1. Bring your own [LSDJ](https://www.littlesounddj.com/lsd/) ROM.
2. Have a `.sav` file corresponding to that ROM.
   1. Emulators like [SameBoy](https://sameboy.github.io/) use this format.
   2. [EMS Flash](https://github.com/mikeryan/ems-flasher) utilities call this an SRAM dump.

```bash
# You can locate your LSDJ rom and save it to an enviroment variable
$ export LSDJ_ROM=/path/to/lsdj/rom
$ lsdj-ripper my-save.sav
# Or address it on the command line
$ lsdj-ripper -l /path/to/lsdj/rom my-save.sav
# You can specify how long to capture audio
$ lsdj-ripper -t 60 my-save.sav # Capture for 60 seconds
# You can collect a screenshot of the song before capturing, if the "PNG" feature is enabled
$ cargo install --git https://github.com/struthiocamelus/lsdj-ripper -F png
$ lsdj-ripper -c -t 60 my-save.sav
# You can even load `.lsdsng` files
$ lsdj-ripper my-song.lsdsng
# Add `-F spinner` to get a cool terminal spinner as well
```

```
Loading song...
Loaded! Playing song...
...............................................
Finished exporting! Rendered 120 seconds in 2s realtime.
```

## Caveats

- [rboy](https://github.com/mvdnes/rboy) is used under the hood, and audio rendering is not perfect yet.
- If your save file has to upgrade, this might not work. Maybe. Best to upgrade before running this tool.

## Acknowledgements

- Thanks to Johan Kotlinski for [lsdrip](https://github.com/jkotlinski/lsdrip) prior art, feedback, and making LSDJ
- Thanks for [Mathijs van de Nes for rboy](https://github.com/mvdnes/rboy)
- Thanks to the PSG Cabal