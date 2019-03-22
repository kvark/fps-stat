use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let path = env::args()
        .skip(1)
        .next()
        .expect("Provide the path to TXT file in the command line");

    let data = BufReader::new(
            File::open(path).expect("unable to read file")
        )
        .lines()
        .flat_map(|line| {
            line.ok()?.parse::<f32>().ok()
        })
        .collect::<Vec<_>>();

    let mean = data
        .iter()
        .cloned()
        .sum::<f32>() / data.len() as f32;
    let variance = data
        .iter()
        .map(|&fps| (fps - mean) * (fps - mean))
        .sum::<f32>() / data.len() as f32;

    println!("n {}, mean {}, variance {}", data.len(), mean, variance);
}
