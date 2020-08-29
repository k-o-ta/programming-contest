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
    type Element = String;
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

    let input: Vec<Element> = read_line_into_vec(&mut reader);
    let mut stuck: Stuck = Stuck::new();
    for e in input.iter() {
        match e.parse::<i32>() {
            Ok(result) => {
                // println!("{}", result);
                stuck.push(result)
            }
            Err(_) => {
                match e.as_str() {
                    "+" => {
                        let a = stuck.pop();
                        let b = stuck.pop();
                        // println!("{} {} {}", b, e, a);
                        stuck.push(b + a)
                    }
                    "-" => {
                        let a = stuck.pop();
                        let b = stuck.pop();
                        // println!("{} {} {}", b, e, a);
                        stuck.push(b - a)
                    }
                    "*" => {
                        let a = stuck.pop();
                        let b = stuck.pop();
                        // println!("{} {} {} = {}", b, e, a, b * a);
                        stuck.push(b * a)
                    }
                    _ => {}
                }
            }
        }
    }
    println!("{}", stuck.pop());
}
struct Stuck {
    v: [i32; 200],
    top: usize,
}
impl Stuck {
    fn new() -> Stuck {
        Stuck {
            top: 0,
            v: [0; 200],
        }
    }
    fn is_empty(&self) -> bool {
        self.top == 0
    }
    fn is_full(&self) -> bool {
        self.top == 200
    }
    fn push(&mut self, value: i32) {
        if self.is_full() {
            panic!("stuck is full");
        }
        self.top = self.top + 1;
        self.v[self.top] = value;
    }
    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            panic!("stuck is empty");
        }
        self.top = self.top - 1;
        self.v[self.top + 1]
    }
}
