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

    let n: usize = read_line(&mut reader);
    let a_s: Vec<i32> = read_line_into_vec(&mut reader);
    println!(
        "{}",
        a_s.iter().map(|a| { divide_times(*a) }).min().unwrap()
    );
}
fn divide_times(n: i32) -> i32 {
    let bits = format!("{:b}", n);
    let ans = bits
        .chars()
        .rev()
        .take_while(|bit| bit.to_digit(2).unwrap() == 0)
        .count() as i32;
    // println!("{}", ans);
    ans
}
