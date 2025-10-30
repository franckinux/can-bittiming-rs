use clap::{Arg, ArgAction, command};
use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
struct BxcanTiming {
    baudrate: u32,
    nbr_tq: u32,
    brp: u32,
    sample_point: f64,
    sample_point_error: f64,
    ts1: u32,
    ts2: u32,
    btr: u32,
}


#[derive(Debug, Serialize)]
enum GenericTiming {
    BxcanTiming(BxcanTiming),
}


fn bxcan(clk: u32, br: u32, sp: f64) -> Vec<GenericTiming> {
    let fclk = clk as f64;
    let bit_rate = br as f64;
    let nominal_bit_time: f64 = 1.0 / bit_rate;
    // println!("nominal bit time: {}", nominal_bit_time);

    // compute valid range of BRP
    let max_tq: f64 = nominal_bit_time / 3.0;
    // println!("maximal time quantum: {}", max_tq);
    let min_tq: f64 = nominal_bit_time / 25.0;
    // println!("mimimal time quantum: {}", min_tq);

    let min_brp = (min_tq * fclk).ceil() as u32;
    let max_brp = (max_tq * fclk).floor() as u32;
    // println!("brp range: {} - {}", min_brp, max_brp);

    let mut results = Vec::new();
    for brp in min_brp..=max_brp {
        let tq = brp as f64 / fclk;
        let nbr_tq = (nominal_bit_time / tq).round() as u32;
        let sample_point = nominal_bit_time * sp;
        let ts1 = (sample_point / tq - 1.0).round() as u32;
        let ts2 = nbr_tq - ts1 - 1;

        if 0 < ts1 && ts1 <= 16 && 0 < ts2 && ts2 <= 8 {
            let real_sample_point = (1.0 + ts1 as f64) * tq;
            let sp_error = (sample_point - real_sample_point).abs() / real_sample_point;

            let real_freq = 1.0 / (1 + ts1 + ts2) as f64 / tq;
            let f_error = ((bit_rate - real_freq) / bit_rate).abs();

            if f_error < 0.001 && sp_error < 0.05 {
                let btr: u32 = ((ts2 - 1) << 20) + ((ts1 - 1) << 16) + brp - 1;

                let result = BxcanTiming {
                    baudrate: br,
                    nbr_tq,
                    brp,
                    sample_point: real_sample_point / nominal_bit_time,
                    sample_point_error: sp_error,
                    ts1,
                    ts2,
                    btr,
                };
                results.push(GenericTiming::BxcanTiming(result))
            }
        }
    }
    results
}


fn main() {
    let matches = command!()
        .arg(
            Arg::new("device")
            .short('d')
            .long("device")
            .action(ArgAction::Set)
            .required(true)
            .help("The device the timings are for ['bxcan']")
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
            println!("valid baurates are {:?}", baud_rates);
            std::process::exit(1);
        }
    }

    if sample_point_position < 50.0 || sample_point_position > 90.0 {
        println!("sample point position must be in the interval [50.0 - 90.0]");
        std::process::exit(1);
    }
    let spp = sample_point_position / 100.0;

    let mut results = vec!();
    match device.as_str() {
        "bxcan" => {
            for baud_rate in baud_rates {
                results.append(&mut bxcan(frequency, baud_rate, spp));
            };
        },
        other => {
            println!("Device not implemented ({})", other);
            std::process::exit(1);
        },
    };

    if results.len() != 0 {
        match format {
            Some(format) => match format.as_str() {
                "json" => println!("{}", serde_json::to_string_pretty(&results).unwrap()),
                _ => println!("unknown format"),
            },
            None => for r in &results {
                match r {
                    GenericTiming::BxcanTiming(r) => println!(
                        "baudrate: {}, nbr_tq: {}, brp: {}, sample point: {:.2}%, \
                        sample point error: {:.2}%, ts1: {}, ts2: {}, btr: 0x{:08x}",
                        r.baudrate, r.nbr_tq, r.brp, r.sample_point * 100.0,
                        r.sample_point_error * 100.0, r.ts1, r.ts2, r.btr
                    ),
                }
            },
        }
    }
}
