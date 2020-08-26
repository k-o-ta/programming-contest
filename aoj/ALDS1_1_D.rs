use std::cmp;
#[cfg(local)]
use std::fs::File;
use std::io;
use std::io::prelude::*;
#[cfg(local)]
use std::io::BufReader;
use std::str::FromStr;

#[cfg(local)]
fn buf_read<T: std::str::FromStr>() -> Vec<T> {
    let f = File::open("input.txt").unwrap();
    let input_lines: Vec<_> = BufReader::new(f).lines().collect();
    input_lines
        .into_iter()
        .map(|e| e.unwrap().parse().ok().unwrap())
        .collect()
}

#[cfg(not(local))]
fn buf_read<T: std::str::FromStr>() -> Vec<T> {
    let stdin = io::stdin();
    let input_lines: Vec<_> = stdin.lock().lines().collect();
    input_lines
        .into_iter()
        .map(|e| e.unwrap().parse().ok().unwrap())
        .collect()
}

fn main() {
    type Element = i64;
    let input: Vec<Element> = buf_read();
    // let mut input_vectors: Vec<Vec<Element>> = vec![];
    // for input_line in input {
    //     let input_str: Vec<String> = input_line.split(' ').map(String::from).collect();
    //     let mut input_vector: Vec<Element> = vec![];
    //     for input_elem_str in input_str {
    //         let elem: Element = Element::from_str(input_elem_str.as_str()).unwrap();
    //         input_vector.push(elem);
    //     }
    //     input_vectors.push(input_vector);
    // }
    // code
    let mut min = input[1];
    let mut max = -9223372036854775808;
    for i in (2usize..(input[0] as usize + 1)) {
        max = cmp::max(max, input[i] - min);
        min = cmp::min(min, input[i]);
        println!("min: {}, cur: {}", min, input[i]);
    }
    println!("{}", max);
}
