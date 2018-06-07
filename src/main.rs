extern crate audiosnap;
extern crate clap;

use clap::{Arg, App};
use std::str::FromStr;

fn main() {
    let args = App::new("audiosnap")
        .version("0.1")
        .about("Audio transient splitter")
        .arg(Arg::with_name("input")
             .help("Input file")
             .short("i")
             .long("input")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("output")
             .help("Output file")
             .short("o")
             .long("output")
             .takes_value(true))
        .arg(Arg::with_name("treshold")
             .help("Sets the audio splitting treshold between 0.0 - 1.0")
             .short("t")
             .long("treshold")
             .takes_value(true))
        .arg(Arg::with_name("tolerance")
             .help("Helps clear sampling noise")
             .short("l")
             .long("tolerance")
             .takes_value(true))
        .arg(Arg::with_name("debug")
             .help("Prints debug info")
             .short("d")
             .long("debug"))
        .get_matches();

    let inputfile = args.value_of("input").unwrap();
    let treshold: f32 = f32::from_str(args.value_of("treshold")
                                      .unwrap_or("0.5")).unwrap();
    let tolerance: f32 = f32::from_str(args.value_of("tolerance")
                                       .unwrap_or("0.25")).unwrap();

    let (splits_0, max_len) = audiosnap::split(inputfile, treshold);
    let splits_1 = audiosnap::smooth(&splits_0, tolerance);

    if args.is_present("debug") {
        // some debug info
        println!("Debug info:");
        println!("input {}", inputfile);
        println!("treshold {}", treshold);
        println!("tolerance {}", tolerance);
        println!("{} splits : {} smoothed", splits_0.len(), splits_1.len());
        println!("{} samples total", max_len);

        // print spec
        audiosnap::print_spec(inputfile);

        // print first,last frames
        let frame = audiosnap::frame(0, max_len, &splits_1);
        println!("First frame: [{},{}]", frame.0, frame.1);
        let frame = audiosnap::frame(splits_1.len() - 1, max_len, &splits_1);
        println!("Last frame: [{},{}]", frame.0, frame.1);
    } else {
        println!("{} splits found", splits_1.len());
    }

    if args.is_present("output") {
        audiosnap::write_frame(
            &audiosnap::frame(1, max_len, &splits_1),
            inputfile,
            args.value_of("output").unwrap());
    }
}
