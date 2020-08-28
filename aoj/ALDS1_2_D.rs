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
    type Element = i32;
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

    let n: Element = read_line(&mut reader);
    let mut a: Vec<Element> = read_lines_to_end(&mut reader);
    let mut g_vec: Vec<Element> = Vec::new();
    let mut g: Element = 0;
    while g < n {
        g_vec.push(g);
        g = g * 3 + 1;
    }
    g_vec.reverse();
    let cnt = shell_sort(&mut a, n, &g_vec);

    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    buff_write(&mut out, &g_vec.len());
    buff_write(&mut out, &vec_to_string(&g_vec));
    buff_write(&mut out, &cnt);
    for e in a.iter() {
        buff_write(&mut out, &e);
    }
}
fn insertion_sort(a: &mut Vec<i32>, n: i32, g: i32, cnt: &mut i32) {
    for i in g..n {
        let v = a[i as usize];
        let mut j = i - g;
        while j >= 0 && a[j as usize] > v {
            a[(j + g) as usize] = a[j as usize];
            *cnt = *cnt + 1;
            j = j - g;
        }
        a[(j + g) as usize] = v;
    }
}

fn shell_sort(a: &mut Vec<i32>, n: i32, g_vec: &Vec<i32>) -> i32 {
    let mut cnt = 0;
    for e in g_vec.iter() {
        insertion_sort(a, n, *e, &mut cnt);
    }
    cnt
}
