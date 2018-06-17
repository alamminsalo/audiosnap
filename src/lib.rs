extern crate hound;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

// library version
const LIB_VERSION: &'static str = env!("CARGO_PKG_VERSION");

// contains loaded sample data
lazy_static! {
    static ref DATA: Mutex<Vec<i16>> = Mutex::new(vec![]);
    static ref SPLITS: Mutex<Vec<u32>> = Mutex::new(vec![]);
}

fn default_spec() -> hound::WavSpec {
    hound::WavSpec{
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int
    }
}

pub fn print_spec(inputfile: &str) {
    let reader = hound::WavReader::open(inputfile).unwrap();
    let spec = reader.spec();
    println!("Input details:");
    println!("Channels: {}", spec.channels);
    println!("Sample rate: {}", spec.sample_rate);
    println!("Bits per sample: {}", spec.bits_per_sample);
    match spec.sample_format {
        hound::SampleFormat::Float => println!("Sample format: float"),
        hound::SampleFormat::Int => println!("Sample format: int"),
    }
}

// Loads wav sample as vec<i16>
pub fn load_file(inputfile: &str) {
    let mut reader = hound::WavReader::open(inputfile).unwrap();

    // Set global DATA
    let mut d = DATA.lock().unwrap();
    d.clear();
    d.append(&mut reader.samples::<i16>()
        .map(|s| s.unwrap() as i16)
        .collect());
}

// Split data and return vector of indexes (split points)
pub fn split(treshold: i16) -> usize {
    let data = DATA.lock().unwrap();
    let mut s = SPLITS.lock().unwrap();
    s.clear();
    s.append(&mut data.iter()
        .enumerate()
        .filter(|&(i, s)| {
            i == 0 || *s > treshold
        })
        .map(|(i,_)| i as u32)
        .collect());
    s.len()
}

pub fn smooth(splits: &Vec<(usize, i16)>, tolerance: f32) -> Vec<(usize, i16)> {
    // Get min,max
    let min = splits.iter()
        .fold(std::i16::MAX, |x,&(_,y)| std::cmp::min(x,y));
    let max = splits.iter()
        .fold(std::i16::MIN, |x,&(_,y)| std::cmp::max(x,y));

    splits.iter()
        .filter(|(i,s)| {
            *i == 0 || *s as f32 > min as f32 + ((max - min) as f32 * tolerance)
        })
    .map(|(i,s)| (i.clone(),s.clone()))
        .collect()
}

// Returns frame of split
pub fn frame(i: usize, max_len: usize, splits: &Vec<(usize, i16)>) -> (u32, u32) {
    (splits.get(i).unwrap_or(&(0,0)).0 as u32
     , splits.get(i+1).unwrap_or(&(max_len,0)).0 as u32)
}

pub fn write_frame(frame: &(u32,u32), data: &Vec<i16>, outputfile: &str) {
    let count = frame.1 - frame.0;
    let spec = default_spec();
    let mut writer = hound::WavWriter::create(outputfile, spec).unwrap();
    for x in data.iter().skip(frame.0 as usize).take(count as usize) {
        writer.write_sample(x.clone()).unwrap();
    }
    while writer.len() % spec.channels as u32 != 0 {
        writer.write_sample(0).unwrap();
    }
    println!("wrote {} samples", count);
    writer.finalize().unwrap();
}


// c api
extern crate libc;
use libc::{c_char,int16_t,size_t};
use std::ffi::{CStr,CString};


// Check file size before allocating buffer
#[no_mangle]
pub extern fn c_file_len() -> size_t {
    DATA.lock().unwrap().len() as usize
}

#[no_mangle]
pub extern fn c_load_file(file: *const c_char) -> size_t {
    let fstr = unsafe{ CStr::from_ptr(file) }.to_str().unwrap();
    load_file(fstr);
    c_file_len()
}

#[no_mangle]
pub extern fn c_data(buf: *mut int16_t, _len: size_t) -> size_t {
    let data = DATA.lock().unwrap();
    // copy contents of values to buf
    for (i,v) in data.iter().take(_len).enumerate() {
        unsafe {
            let ptr = buf.offset(i as isize) as *mut i16;
            *ptr = v.clone();
        }
    }

    std::cmp::max(data.len(),_len)
}

#[no_mangle]
pub extern fn c_split(ceil: int16_t) -> size_t {
    split(ceil as i16)
}

#[no_mangle]
pub extern fn c_version() -> *const c_char {
    CString::new(LIB_VERSION).unwrap().into_raw()
}
