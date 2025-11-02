mod bxcan;

use std::process;

use clap::{Arg, ArgAction, command, error, Parser, value_parser, ValueEnum};
use serde_json;

use bxcan::BxcanTiming;


#[derive(Copy, Clone, Debug, ValueEnum)]
enum Device {
    Bxcan,
}


#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputFormat {
    Json,
}


#[derive(Parser)]
struct Args {
    #[arg(short, long, value_enum, help="The device the timings can will computed for")]
    device: Device,
    #[arg(short, long, help="Frequency at the entry of the prescaler (Hz)")]
    frequency: u32,
    #[arg(short, long, help="Bits per second")]
    baudrate: Option<u32>,
    #[arg(short, long, help="Sample point position (%)")]
    sample_point_position: f64,
    #[arg(short, long, value_enum, help="Output format")]
    output_format: Option<OutputFormat>,
    #[arg(short='w', long, default_value_t=1, help="Sync jump width")]
    sjw: u32,
}


fn main() {
    let args = Args::parse();


    // required arguments
    let device = args.device;
    let frequency = args.frequency;
    let sample_point_position = args.sample_point_position;

    // optional arguments
    let baud_rate = args.baudrate;
    let output_format = args.output_format;

    // optional arguments with default value
    let sjw = args.sjw;

    let mut baud_rates = vec!(
        1_000_000, 500_000, 250_000, 125_000, 100_000, 83_333, 50_000, 20_000, 10_000
    );
    if let Some(baud_rate) = baud_rate {
        if baud_rates.contains(&baud_rate) {
            baud_rates = vec!(baud_rate);
        } else {
            eprintln!("Valid baudrates are {:?}", baud_rates);
            process::exit(1);
        }
    }

    if sample_point_position < 50.0 || sample_point_position > 90.0 {
        eprintln!("Sample point position must be in the interval [50.0 - 90.0]");
        process::exit(1);
        // error("Sample point position must be in the interval [50.0 - 90.0]");
    }
    let spp = sample_point_position / 100.0;

    let mut results = vec!();
    match device {
        Device::Bxcan => {
            for baud_rate in baud_rates {
                results.append(&mut BxcanTiming::timings(frequency, baud_rate, spp, sjw));
            };
        },
    };

    match output_format {
        Some(OutputFormat::Json) => {
            println!("{}", serde_json::to_string_pretty(&results).unwrap());
        },
        None => for r in &results {
            println!("{}", r);
        },
    }
}
