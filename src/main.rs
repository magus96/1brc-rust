use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
struct StnValues {
    min: f32,
    max: f32,
    mean: f32,
    count: u32,
}

macro_rules! print_result {
    ($res:expr) => {
        let mut ord_res: BTreeMap<String, StnValues> = BTreeMap::new();
        for (stn_name, stn_values) in $res {
            ord_res.insert(stn_name, stn_values);
        }
        let mut iterator = ord_res.iter().peekable();
        print!("{{");
        while let Some((stn_name, stn_values)) = iterator.next() {
            if iterator.peek().is_none() {
                print!(
                    "{}={:.1}/{:.1}/{:.1}}}",
                    stn_name, stn_values.min, stn_values.mean, stn_values.max
                );
            } else {
                print!(
                    "{}={:.1}/{:.1}/{:.1}, ",
                    stn_name, stn_values.min, stn_values.mean, stn_values.max
                );
            }
        }
    };
}

fn main() -> io::Result<()> {
    let start = Instant::now();
    let f = File::open("measurements.txt")?;
    let mut buf = BufReader::new(f);
    let mut str_buf = String::new();
    let mut res_map: FxHashMap<String, StnValues> = FxHashMap::default();
    while let Ok(bytes_read) = buf.read_line(&mut str_buf) {
        if bytes_read == 0 {
            break;
        }
        let line_str = str_buf.trim();
        let mut line_vec = line_str.split(";");
        let stn_name = line_vec.next().expect("Error");
        let line_int = fast_float::parse(line_vec.next().expect("Error")).expect("Error");
        res_map
            .entry(stn_name.to_owned())
            .and_modify(|e| {
                if line_int < e.min {
                    e.min = line_int
                }
                if line_int > e.max {
                    e.max = line_int
                }
                e.mean = e.mean + line_int;
                e.count += 1;
            })
            .or_insert(StnValues {
                min: line_int,
                max: line_int,
                mean: line_int,
                count: 1,
            });
        str_buf.clear();
    }
    for (_, stn_values) in res_map.iter_mut() {
        stn_values.mean = round_off(stn_values.mean / stn_values.count as f32);
        stn_values.min = round_off(stn_values.min);
        stn_values.max = round_off(stn_values.max);
    }
    let dur = start.elapsed();
    print_result!(res_map);
    println!("time elapsed: {:?}", dur);
    Ok(())
}

fn round_off(value: f32) -> f32 {
    (value * 10.0).round() / 10.0
}
