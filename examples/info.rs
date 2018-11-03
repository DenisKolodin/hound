// This example show information about WAV file like `sndfile-info` utility.

extern crate hound;

use std::env;
use std::ffi::OsStr;
use std::path::Path;
use hound::{SampleLoop, SamplerSpec, WavSpec};

fn main() {
    let fname = env::args().nth(1).expect("no file given");
    let path = Path::new(&fname);
    let reader = hound::WavReader::open(&path).unwrap();
    let file_name = path.file_name().and_then(OsStr::to_str).unwrap_or("- - -");
    println!("File : {}", file_name);
    //println!("Length : {}", reader.len());
    let spec = reader.spec();
    print_spec(&spec);
    let sampler_spec = reader.sampler_spec();
    if let Some(spec) = sampler_spec {
        print_sampler_spec(&spec);
    }
}

fn print_spec(spec: &WavSpec) {
    println!("fmt  :");
    println!("  Format       : {}", spec.sample_format);
    println!("  Channels     : {}", spec.channels);
    println!("  Sample Rate  : {}", spec.sample_rate);
    println!("  Bit Width    : {}", spec.bits_per_sample);
}

fn print_sampler_spec(spec: &SamplerSpec) {
    println!("smpl :");
    println!("  Manufacturer : {}", hex_joined_by(&u32_to_array(spec.manufacturer), ""));
    println!("  Product      : {}", spec.product);
    println!("  Period       : {} nsec", spec.sample_period);
    println!("  Midi Note    : {}", spec.midi_unity_note);
    println!("  Pitch Fract. : {}", spec.midi_pitch_fraction);
    println!("  SMPTE Format : {}", spec.smpte_format);
    println!("  SMPTE Offset : {}", hex_joined_by(&u32_to_array(spec.smpte_offset), ":"));
    println!("  Loop Count   : {}", spec.loops.len());
    for l in &spec.loops {
        print_loop(l);
    }
    println!("  Sampler Data : {}", spec.data.len());
    println!("    {}", hex_joined_by(&spec.data, " "));
    println!("End");
}

fn print_loop(l: &SampleLoop) {
    println!("    Cue ID: {:2}  Type : {:2}  Start : {:5}  End : {:5}  Fraction : {:5}  Count : {:5}", l.identifier, l.loop_type, l.start, l.end, l.fraction, l.play_count);
}

fn hex_joined_by(bytes: &[u8], sep: &str) -> String {
    bytes.iter()
        .map(|byte| format!("{:02X}", byte))
        .collect::<Vec<_>>()
        .join(sep)
}

fn u32_to_array(x: u32) -> [u8; 4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    [b1, b2, b3, b4]
}
