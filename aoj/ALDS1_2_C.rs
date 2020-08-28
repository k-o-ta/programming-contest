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

    type Element2 = String;

    let n: Element = read_line(&mut reader);
    let cards: Vec<Element2> = read_line_into_vec(&mut reader);
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    let mut bubble = cards.clone();
    bubble_sort(&mut bubble, n);
    buff_write(&mut out, &vec_to_string(&bubble));
    buff_write(&mut out, &String::from("Stable"));

    let mut selection = cards.clone();
    selection_sort(&mut selection, n);
    buff_write(&mut out, &vec_to_string(&selection));
    let is_stable = bubble.iter().zip(selection.iter()).all(|(b, s)| b == s);
    buff_write(
        &mut out,
        &String::from(if is_stable { "Stable" } else { "Not stable" }),
    );
}
fn get_value(card: &String) -> i32 {
    card.chars().last().unwrap().to_digit(10).unwrap() as i32
}
fn bubble_sort(cards: &mut Vec<String>, n: i32) {
    for i in 0usize..(n as usize) {
        for j in ((i + 1)..(n as usize)).rev() {
            if get_value(&cards[j - 1]) > get_value(&cards[j]) {
                cards.swap(j - 1, j);
            }
        }
    }
}

fn selection_sort(cards: &mut Vec<String>, n: i32) {
    for i in 0usize..(n as usize) {
        let mut minj = i;
        for j in i..(n as usize) {
            if get_value(&cards[minj]) > get_value(&cards[j]) {
                minj = j;
            }
        }
        cards.swap(i, minj);
    }
}
