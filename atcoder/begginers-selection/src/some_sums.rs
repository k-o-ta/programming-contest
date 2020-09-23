#[cfg(local)]
use std::fs::File;
use std::io::prelude::*;
#[cfg(not(local))]
use std::io::stdin;
use std::io::Read;

use std::fmt::Debug;
use std::fmt::Display;
use std::io::stdout;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::StdoutLock;
use std::str::FromStr;

#[allow(dead_code)]
fn read_line<T: FromStr, U: Read>(reader: &mut BufReader<U>) -> T
where
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    let _ = reader.read_line(&mut input);
    input.trim().parse().unwrap()
}

#[allow(dead_code)]
fn read_line_into_vec<T: FromStr, U: Read>(reader: &mut BufReader<U>) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    let _ = reader.read_line(&mut input);
    input
        .trim()
        .split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_lines_to_end<T: FromStr, U: Read>(reader: &mut BufReader<U>) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    let _ = reader.read_to_string(&mut input);
    input
        .split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_line_into_optional_vec<T: FromStr, U: Read>(reader: &mut BufReader<U>) -> Option<Vec<T>>
where
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    let result = reader.read_line(&mut input).unwrap();
    return if result == 0 {
        None
    } else {
        Some(
            input
                .trim()
                .split_whitespace()
                .map(|e| e.parse().unwrap())
                .collect(),
        )
    };
}

#[allow(dead_code)]
fn read_lines_to_end_into_vec<T: FromStr, U: Read>(reader: &mut BufReader<U>) -> Vec<Vec<T>>
where
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    let _ = reader.read_to_string(&mut input);
    input
        .lines()
        .map({
            |line| {
                line.split_whitespace()
                    .map({ |e| e.parse().unwrap() })
                    .collect()
            }
        })
        .collect()
}

#[allow(dead_code)]
fn buff_write<T: Display>(writer: &mut BufWriter<StdoutLock>, output: &T) {
    let _ = writeln!(writer, "{}", output);
}

#[allow(dead_code)]
fn vec_to_string<T: ToString>(vec: &Vec<T>) -> String {
    vec.iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    #[cfg(local)]
    let file = File::open("input.txt").unwrap();
    #[cfg(local)]
    let mut reader = BufReader::new(file);

    #[cfg(not(local))]
    let stdin = stdin();
    #[cfg(not(local))]
    let handle = stdin.lock();
    #[cfg(not(local))]
    let mut reader = BufReader::new(handle);

    let n_a_b: Vec<u32> = read_line_into_vec(&mut reader);
    let mut sum = 0;
    for n in 1..=n_a_b[0] {
        let digits_sum = digits_sum(n);
        if n_a_b[1] <= digits_sum && digits_sum <= n_a_b[2] {
            // println!("n: {}, digits_sum: {}", n, digits_sum);
            sum += n;
        }
    }
    println!("{}", sum);
}

fn digits_sum(num: u32) -> u32 {
    num.to_string().chars().fold(0, |acc, c| {
        // println!("acc: {}, c: {}, digits: {:?}", acc, c, c.to_digit(10));
        acc + c.to_digit(10).expect("parse error")
    })
}
fn p(num: u32) -> u32 {
    println!("{}", num);
    num
}
