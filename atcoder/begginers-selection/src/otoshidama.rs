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

    let n_y: Vec<u32> = read_line_into_vec(&mut reader);

    if n_y[0] * 10000 == n_y[1] {
        println!("{} 0 0", n_y[0]);
        return;
    }
    if n_y[0] * 5000 == n_y[1] {
        println!("0 {} 0", n_y[0]);
        return;
    }
    if n_y[0] * 1000 == n_y[1] {
        println!("0 0 {}", n_y[0]);
        return;
    }

    for a in 0..(n_y[0] - 1) {
        let sum_x = (a + 1) * 10000;
        let sum_y = (n_y[0] - 1 - a) * 5000;
        if sum_x + sum_y == n_y[1] {
            println!("{} {} 0", (a + 1), n_y[0] - 1 - a);
            return;
        }
        let sum_y = (a + 1) * 5000;
        let sum_z = (n_y[0] - 1 - a) * 1000;
        if sum_y + sum_z == n_y[1] {
            println!("0 {} {}", (a + 1), (n_y[0] - 1 - a));
            return;
        }
        let sum_x = (a + 1) * 10000;
        let sum_z = (n_y[0] - 1 - a) * 1000;
        if sum_x + sum_z == n_y[1] {
            println!("{} 0 {}", a + 1, n_y[0] - 1 - a);
            return;
        }
    }

    for x in 0..(n_y[0] - 2) {
        for y in x..(n_y[0] - 1) {
            let sum_x = (x + 1) * 10000;
            let sum_y = (y - x) * 5000;
            let sum_z = (n_y[0] - 1 - y) * 1000;
            if sum_x + sum_y + sum_z == n_y[1] {
                println!("{} {} {}", x + 1, y - x, n_y[0] - 1 - y);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
