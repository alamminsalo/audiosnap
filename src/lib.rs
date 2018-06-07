extern crate hound;

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

pub fn split(inputfile: &str, treshold: f32) -> (Vec<(usize, i16)>, usize) {
    let mut reader = hound::WavReader::open(inputfile).unwrap();
    let ceil = (std::i16::MAX as f32 * treshold) as i16;
    (reader.samples::<i16>()
        .enumerate()
        .map(|(i, s)| (i, (s.unwrap() as i16).abs()))
        .filter(|&(i, s)| {
            i == 0 || s > ceil
        })
        .collect(), reader.len() as usize)
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

pub fn write_frame(frame: &(u32,u32), inputfile: &str, outputfile: &str) {
    let count = frame.1 - frame.0;
    let mut reader = hound::WavReader::open(inputfile).unwrap();
    let mut writer = hound::WavWriter::create(outputfile, reader.spec()).unwrap();
    reader.seek(frame.0).unwrap();
    for x in reader.samples::<i16>().take(count as usize) {
        writer.write_sample(x.unwrap() as i16).unwrap();
    }
    while writer.len() % reader.spec().channels as u32 != 0 {
        writer.write_sample(0).unwrap();
    }
    println!("wrote {} samples", count);
    writer.finalize().unwrap();
}

