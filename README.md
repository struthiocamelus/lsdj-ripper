# LSDJ-Ripper

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
# Add `-F spinner` to get a cool terminal spinner as well
```

## Caveats

- [rboy](https://github.com/mvdnes/rboy) is used under the hood, and audio rendering is not perfect yet.
- If your save file has to upgrade, this might not work. Maybe. Best to upgrade before running this tool.

## Acknowledgements

- Thanks to Johan Kotlinski for [lsdrip](https://github.com/jkotlinski/lsdrip) prior art, feedback, and making LSDJ
- Thanks for [Mathijs van de Nes for rboy](https://github.com/mvdnes/rboy)
- Thanks to the PSG Cabal