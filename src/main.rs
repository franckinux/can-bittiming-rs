use serde::Serialize;
use serde_json;


#[derive(Debug, Serialize)]
struct BxcanResult {
    nbr_tq: u32,
    brp: u32,
    sample_point: f64,
    sample_point_error: f64,
    ts1: u32,
    ts2: u32,
    btr: u32,
}


fn bxcan(clk: u64, br: u64, sp: f64) -> Vec<BxcanResult> {
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

                let result = BxcanResult {
                    nbr_tq,
                    brp,
                    sample_point: real_sample_point / nominal_bit_time,
                    sample_point_error: sp_error,
                    ts1,
                    ts2,
                    btr,
                };
                results.push(result)
            }
        }
    }
    results
}


fn main() {
    let result = bxcan(45000000, 125000, 0.875);

    for r in &result {
        println!(
            "nbr_tq: {}, brp: {}, sample point: {:.2}%, sample point error: {:.2}%, \
            ts1: {}, ts2: {}, btr: 0x{:08x}",
            r.nbr_tq, r.brp, r.sample_point * 100.0, r.sample_point_error * 100.0, r.ts1, r.ts2, r.btr
        );
    }

    let str =  serde_json::to_string_pretty(&result).unwrap();
    println!("{str}");
}
