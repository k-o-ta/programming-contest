#![allow(unused)]

#[cfg(local)]
use std::fs::File;
use std::io;
use std::io::prelude::*;
#[cfg(not(local))]
use std::io::stdin;
#[cfg(not(local))]
use std::io::StdinLock;

use std::cmp;
use std::io::BufReader;
use std::str::FromStr;

fn read_line<T: std::str::FromStr, U: std::io::Read>(buf: &mut BufReader<U>) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    buf.read_line(&mut input);
    input.trim().parse().unwrap()
}

fn read_line_into_vec<T: std::str::FromStr, U: std::io::Read>(buf: &mut BufReader<U>) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    buf.read_line(&mut input);
    input
        .trim()
        .split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect()
}

fn read_lines_to_end<T: std::str::FromStr, U: std::io::Read>(buf: &mut BufReader<U>) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    buf.read_to_string(&mut input);
    input
        .split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect()
}

fn read_lines_to_end_into_vec<T: std::str::FromStr, U: std::io::Read>(
    buf: &mut BufReader<U>,
) -> Vec<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    buf.read_to_string(&mut input);
    input
        .split_whitespace()
        .map({
            |line| {
                line.trim()
                    .split_whitespace()
                    .map(|e| e.parse().unwrap())
                    .collect()
            }
        })
        .collect()
}

fn main() {
    type Element = i64;
    #[cfg(local)]
    let file = File::open("input.txt").unwrap();
    #[cfg(local)]
    let mut reader = BufReader::new(file);

    #[cfg(not(local))]
    let stdin = stdin();
    #[cfg(not(local))]
    let mut handle = stdin.lock();
    #[cfg(not(local))]
    let mut reader = BufReader::new(handle);

    let len: i64 = read_line(&mut reader);
    let input: Vec<i64> = read_lines_to_end(&mut reader);

    let mut min = input[0];
    let mut max = -9223372036854775808;
    for i in (1usize..(len as usize)) {
        max = cmp::max(max, input[i] - min);
        min = cmp::min(min, input[i]);
    }
    println!("{}", max);
}
