extern crate audiosnap;
extern crate clap;
extern crate iui;

use clap::{Arg, App};
use std::str::FromStr;

mod gui;
mod state;

fn main() {
    let args = App::new("audiosnap")
        .version("0.1")
        .about("Audio transient splitter")
        .arg(Arg::with_name("cli")
             .help("Runs in cli mode")
             .long("cli"))
        .arg(Arg::with_name("input")
             .help("Input file")
             .short("i")
             .long("input")
             .takes_value(true))
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

    if args.is_present("cli") {
        if !args.is_present("input") {
            panic!("input file missing!");
        }
        let inputfile = args.value_of("input").unwrap();
        let treshold: f32 = f32::from_str(args.value_of("treshold")
                                          .unwrap_or("0.5")).unwrap();
        let tolerance: f32 = f32::from_str(args.value_of("tolerance")
                                           .unwrap_or("0.25")).unwrap();

        let data = audiosnap::load_file(inputfile);
        let splits = audiosnap::split(&data, treshold);

        if args.is_present("debug") {
            // some debug info
            println!("Debug info:");
            println!("input {}", inputfile);
            println!("treshold {}", treshold);
            println!("tolerance {}", tolerance);
            println!("{} splits", splits.len());
            println!("{} samples total", data.len());

            // print spec
            audiosnap::print_spec(inputfile);
        }

    } else {
        // Run GUI application
        gui::start();
    }
}

