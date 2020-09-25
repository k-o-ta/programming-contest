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

    let n: u32 = read_line(&mut reader);
    let mut from = Point::new(0, 0, 0);
    for _ in 0..n {
        let t_x_y: Vec<u32> = read_line_into_vec(&mut reader);
        let to = Point::new(t_x_y[0], t_x_y[1], t_x_y[2]);
        if from.is_reacheable(&to) {
            // println! {"from: {:?}, to: {:?}", from, to}
            from = to;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
    t: u32,
}

impl Point {
    fn new(t: u32, x: u32, y: u32) -> Point {
        Point { t, x, y }
    }
    fn is_reacheable(&self, dest: &Point) -> bool {
        if self.t >= dest.t {
            return false;
        }
        let diff =
            ((self.x as i32 - dest.x as i32).abs() + (self.y as i32 - dest.y as i32).abs()) as u32;
        let t_diff = dest.t - self.t;
        // println!("t_diff: {}, diff: {}", t_diff, diff,);
        if t_diff < diff {
            return false;
        }
        ((t_diff - diff) % 2) == 0
    }
}
