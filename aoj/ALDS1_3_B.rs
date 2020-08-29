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

    let n_q: Vec<i32> = read_line_into_vec(&mut reader);
    let n = n_q[0];
    let q = n_q[1];
    let name_times: Vec<(String, i32)> = read_lines_to_end_into_vec(&mut reader)
        .into_iter()
        .map(|vec: Vec<String>| (vec[0].clone(), vec[1].parse().unwrap()))
        .collect();
    // println! {"{:?}", name_times};
    let mut queue = Queue::new(name_times);
    let mut time = 0;
    while !queue.is_empty() {
        let mut e = queue.dequeue();
        // println!("e: {:?}, head: {}, tail: {}", e, queue.head, queue.tail);
        if e.1 > q {
            e.1 = e.1 - q;
            time = time + q;
            queue.enqueue(e);
        } else {
            time = time + e.1;
            println!("{} {}", e.0, time);
        }
    }
    // for mut name_time in name_times {
    //     // println! {"{:?}", name_time};
    //     let name: String = name_time.remove(0);
    //     let time: i32 = name_time[0].parse().unwrap();
    //     queue.enqueue((name, time));
    // }
}

struct Queue {
    vec: Vec<(String, i32)>,
    head: usize,
    tail: usize,
}

impl Queue {
    fn new(mut vec: Vec<(String, i32)>) -> Queue {
        let tail = vec.len();
        vec.push((String::from(""), 0));
        Queue {
            vec,
            head: 0,
            tail: tail,
        }
    }
    fn enqueue(&mut self, value: (String, i32)) {
        self.vec[self.tail] = value;
        self.tail = (self.tail + 1) % self.vec.len();
    }
    fn dequeue(&mut self) -> (String, i32) {
        let ret = self.vec[self.head].clone();
        self.head = (self.head + 1) % self.vec.len();
        ret
    }
    fn is_empty(&self) -> bool {
        self.head == self.tail
    }
}
