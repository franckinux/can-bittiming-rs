mod bxcan;

use std::process;

use clap::{Arg, ArgAction, command};
use serde_json;

use bxcan::BxcanTiming;


fn main() {
    let matches = command!()
        .arg(
            Arg::new("device")
            .short('d')
            .long("device")
            .action(ArgAction::Set)
            .required(true)
            .help("The devices the timings can be computed ['bxcan']")
		)
        .arg(
            Arg::new("frequency")
            .short('f')
            .long("frequency")
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(u32))
            .required(true)
            .help("Frequency at the entry of the prescaler (Hz)")
		)
        .arg(
            Arg::new("baudrate")
            .short('b')
            .long("baudrate")
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(u32))
            .help("Bits per second")
		)
        .arg(
            Arg::new("sample-point-position")
            .short('s')
            .long("sample-point")
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(f64))
            .required(true)
            .help("Sample point position (%)")
        )
        .arg(
            Arg::new("output-format")
            .short('o')
            .long("output-format")
            .action(ArgAction::Set)
            .help("Output format ['json']")
        ).arg_required_else_help(true)
        .get_matches();

    // required arguments
    let device = matches.get_one::<String>("device").unwrap();
    let frequency = *matches.get_one::<u32>("frequency").unwrap();
    let sample_point_position = *matches.get_one::<f64>("sample-point-position").unwrap();

    // optional arguments
    let baud_rate = matches.get_one::<u32>("baudrate");
    let format = matches.get_one::<String>("output-format");

    let mut baud_rates = vec!(
        1_000_000, 500_000, 250_000, 125_000, 100_000, 83_333, 50_000, 20_000, 10_000
    );
    if let Some(baud_rate) = baud_rate {
        if baud_rates.contains(&baud_rate) {
            baud_rates = vec!(*baud_rate);
        } else {
            eprintln!("Valid baudrates are {:?}", baud_rates);
            process::exit(1);
        }
    }

    if sample_point_position < 50.0 || sample_point_position > 90.0 {
        eprintln!("Sample point position must be in the interval [50.0 - 90.0]");
        process::exit(1);
    }
    let spp = sample_point_position / 100.0;

    let mut results = vec!();
    match device.as_str() {
        "bxcan" => {
            for baud_rate in baud_rates {
                results.append(&mut BxcanTiming::timings(frequency, baud_rate, spp));
            };
        },
        other => {
            eprintln!("Device not implemented ({})", other);
            process::exit(1);
        },
    };

    match format {
        Some(format) => match format.as_str() {
            "json" => println!("{}", serde_json::to_string_pretty(&results).unwrap()),
            _ => {
                eprintln!("Unknown format");
                process::exit(1);
            },
        },
        None => for r in &results {
            println!("{}", r);
        },
    }
}
