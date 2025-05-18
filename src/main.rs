mod wav_audio;

use clap;
use lsdj;
#[cfg(feature = "png")]
use clap::ArgAction;
use clap::{Arg, Command};
use device::Device;
use rboy;
use rboy::KeypadKey::*;
use rboy::{KeypadKey, device};
#[cfg(feature = "spinner")]
use spinner;
#[cfg(feature = "spinner")]
use spinner::SpinnerBuilder;
use std::fs::File;
#[allow(unused_imports)] // Write is only used when spinner is disabled.
use std::io::{Write};
use std::io::{BufWriter, Read};
use std::time::SystemTime;
use lsdj::fs::{File as LsdjFile};
use lsdj::lsdsng::LsdSng;

const TICKS_PER_SECOND: u32 = 4_194_304;

#[cfg(feature = "png")]
const SCREEN_WIDTH: u32 = 160;
#[cfg(feature = "png")]
const SCREEN_HEIGHT: u32 = 144;

fn main() {
    #[allow(unused_mut)] // Required by features.
    let mut command = Command::new("lsdj-ripper")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("lsdj")
                .short('l')
                .help("LSDJ rom")
                .env("LSDJ_ROM")
                .required(true),
        )
        .arg(
            Arg::new("input")
                .help("Save file or lsdsng to rip from")
                .required(true),
        )
        .arg(
            Arg::new("outfile")
                .short('o')
                .default_value("out.wav")
                .help("Output filename"),
        )
        .arg(
            Arg::new("sample-rate")
                .short('s')
                .long("sample-rate")
                .default_value("48000")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("length")
                .short('t')
                .long("time")
                .help("Length in seconds to record")
                .default_value("120")
                .value_parser(clap::value_parser!(u32)),
        );
    #[cfg(feature = "png")]
    {
        command = command.arg(
            Arg::new("screen-capture")
                .short('c')
                .long("screen-capture")
                .help("Capture the screen after loading the song")
                .action(ArgAction::SetTrue)
                .default_value("false"),
        );
    }
    let matches = command.get_matches();
    let rom = matches.get_one::<String>("lsdj").unwrap().as_str();
    let save = matches.get_one::<String>("input").unwrap().clone();
    let outfile = matches.get_one::<String>("outfile").unwrap().clone();
    let sample_rate = matches.get_one::<u32>("sample-rate").unwrap().to_owned();
    let length = matches.get_one::<u32>("length").unwrap().to_owned();
    #[cfg(feature = "png")]
    let screen_capture = matches.get_flag("screen-capture").to_owned();
    let mut romfile = Vec::new();
    File::open(rom)
        .expect("Failed to open ROM file")
        .read_to_end(&mut romfile)
        .expect("Failed to read ROM file");

    let mut device =
        Device::new_cgb_from_buffer(romfile, false, None).expect("Failed to create device");

    let mut save_buf = Vec::new();
    let mut needs_load = true;
    if save.ends_with(".sav") {
        File::open(&save)
            .expect("Failed to open save file")
            .read_to_end(&mut save_buf)
            .expect("Failed to read save file");
    } else if save.ends_with(".lsdsng") {
        needs_load = false;
        let sng = LsdSng::from_path(save)
            .expect("Failed to load save file");
        let mut sram = lsdj::sram::SRam::new();
        sram.working_memory_song = sng.decompress().unwrap();
        let writer = BufWriter::new(&mut save_buf);
        sram.to_writer(writer).expect("Failed to write save file");
    }
    device.loadram(&save_buf).expect("Failed to load save file");
    device.enable_audio(wav_audio::Player::boxed_new(outfile, sample_rate), true);
    // startup sequence
    println!("Loading song...");
    device_wait(&mut device, 20.0);
    if needs_load {
        load_song(&mut device);
    }
    println!("Loaded! Playing song...");
    #[cfg(feature = "png")]
    if screen_capture {
        take_screenshot(&mut device, "screen.png")
    }
    #[cfg(feature = "spinner")]
    let spinner = SpinnerBuilder::new("Running emulator...".into())
        .spinner(Vec::from(["-", "\\", "|", "/"]))
        .start();
    let max = length * TICKS_PER_SECOND;
    let mut ticks = 0;
    let started = SystemTime::now();
    key_press(&mut device, Start, 0.1);
    while ticks < max {
        ticks += device.do_cycle();
        #[cfg(feature = "spinner")]
        {
            let percent = ((ticks as f32 / max as f32) * 100.0).round() as u32;
            spinner.update(format!("Running emulator: steps: {:2}%...", percent));
        }
        #[cfg(not(feature = "spinner"))]
        {
            if ticks % (TICKS_PER_SECOND) == 0 {
                print!(".");
                std::io::stdout().flush().unwrap();
            }
        }
    }
    #[cfg(feature = "spinner")]
    {
        spinner.update("Running emulator: steps: 100%...".to_owned());
        spinner.close();
    }
    println!();
    let elapsed = started.elapsed().unwrap().as_secs();
    println!("Finished exporting! Rendered {length} seconds in {elapsed}s realtime.");
}

fn device_wait(device: &mut Device, seconds: f32) {
    let mut ticks = 0;
    let max_ticks = (seconds * TICKS_PER_SECOND as f32).round() as u32;
    while ticks < max_ticks {
        ticks += device.do_cycle();
    }
}

fn multi_key_press(device: &mut Device, keys: Vec<KeypadKey>, length: f32) {
    keys.iter().for_each(|key| {
        device.keydown(key.to_owned());
    });
    device_wait(device, length);
    keys.iter().for_each(|key| {
        device.keyup(key.to_owned());
    })
}

fn key_press(device: &mut Device, key: KeypadKey, length: f32) {
    multi_key_press(device, Vec::from([key]), length);
}

fn load_song(device: &mut Device) {
    multi_key_press(device, Vec::from([Select, Up]), 0.1);
    device_wait(device, 1.0);
    key_press(device, Down, 3.0);
    key_press(device, A, 0.1);
    device_wait(device, 1.0);
    key_press(device, A, 0.1);
    key_press(device, Up, 5.0);
    device_wait(device, 1.0);
    // Load song.
    key_press(device, A, 0.1);
    device_wait(device, 1.0);
    // Discard changes.
    key_press(device, Left, 0.1);
    key_press(device, A, 0.1);
    device_wait(device, 10.0);
}

#[cfg(feature = "png")]
fn take_screenshot(device: &mut Device, filename: &str) {
    let mut encoder = png::Encoder::new(
        File::create(filename).expect("Unable to open screenshot file for writing"),
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
    );
    encoder.set_color(png::ColorType::Rgb);
    encoder
        .write_header()
        .expect("Unable to write header")
        .write_image_data(&device.get_gpu_data())
        .expect("Unable to write image data");
}
