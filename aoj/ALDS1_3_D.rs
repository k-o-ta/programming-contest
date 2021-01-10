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

    let input: Element = read_line(&mut reader);
    let mut s1 = Vec::new();
    let mut sum = 0;
    let mut s2: Vec<(usize, usize)> = Vec::new();
    // let mut s1_horizontal_count = 0;
    for (i, s) in input.chars().enumerate() {
        match (s, s1.is_empty()) {
            ('\\', _) => {
                s1.push(i);
            }
            ('/', true) | ('_', true) => {
                continue;
            }
            ('/', false) => {
                let s1_top = s1.pop().expect("s1 must not be empty");
                sum = sum + (i - s1_top);
                let mut a = i - s1_top;
                while !s2.is_empty() && (*s2.last().expect("s2 must not be empty")).0 > s1_top {
                    a = a + s2.pop().expect("s2 must not be empty").1;
                }
                s2.push((s1_top, a));
            }
            ('_', false) => {}
            _ => panic!("unreachable"),
        }
    }
    println!("{}", sum);
    // println!("{:?}", s2);
    let s2_size = s2.len();
    let mut s2_sum: Vec<usize> = s2.iter().map(|&elem| elem.1).collect();
    s2_sum.insert(0, s2_size);
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    buff_write(&mut out, &vec_to_string(&s2_sum));
}
