extern crate hound;

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
pub fn load_file(inputfile: &str) -> Vec<i16> {
    let mut reader = hound::WavReader::open(inputfile).unwrap();
    reader.samples::<i16>()
        .map(|s| s.unwrap() as i16)
        .collect()
}

// Split data and return vector of indexes (split points)
pub fn split(data: &Vec<i16>, treshold: f32) -> Vec<u32> {
    let ceil = (std::i16::MAX as f32 * treshold) as i16;
    data.iter()
        .enumerate()
        .filter(|&(i, s)| {
            i == 0 || s.abs() > ceil
        })
    .map(|(i,_)| i as u32)
        .collect()
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

