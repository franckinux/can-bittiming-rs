use serde::{Serialize, Serializer, ser::SerializeStruct};
use std::fmt;


pub struct BxcanTiming {
    baudrate: u32,
    nbr_tq: u32,
    brp: u32,
    sample_point: f64,
    sample_point_error: f64,
    ts1: u32,
    ts2: u32,
    btr: u32,
}


impl fmt::Display for BxcanTiming {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "baudrate: {}, nbr_tq: {}, brp: {}, sample point: {:.2}%, \
            sample point error: {:.2}%, ts1: {}, ts2: {}, btr: 0x{:08x}",
            self.baudrate, self.nbr_tq, self.brp, self.sample_point * 100.0,
            self.sample_point_error * 100.0, self.ts1, self.ts2, self.btr
        )
    }
}


impl Serialize for BxcanTiming {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // start json struct with one more field than the struct
        let mut s = serializer.serialize_struct("BxcanTiming", 9)?;

        // normal fields (integers)
        s.serialize_field("baudrate", &self.baudrate)?;
        s.serialize_field("nbr_tq", &self.nbr_tq)?;
        s.serialize_field("brp", &self.brp)?;
        s.serialize_field("ts1", &self.ts1)?;
        s.serialize_field("ts2", &self.ts2)?;
        s.serialize_field("btr", &self.btr)?;

        // floats are rounded to 3 decimals
        s.serialize_field("sample_point", &((self.sample_point * 1000.0).round() / 1000.0))?;
        s.serialize_field("sample_point_error", &((self.sample_point_error * 1000.0).round() / 1000.0))?;

        // hex numbers are not supported in json format, so we add a string field (thanks ChatGPT)
        // field computed automatically "0x..."
        let btr_hex = format!("0x{:X}", self.btr);
        s.serialize_field("btr_hex", &btr_hex)?;

        s.end()
    }
}


impl BxcanTiming {
    pub fn timings(clk: u32, br: u32, sp: f64) -> Vec<Self> {
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
                    results.push(result)
                }
            }
        }
        results
    }
}


