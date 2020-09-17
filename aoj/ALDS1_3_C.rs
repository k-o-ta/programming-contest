use std::collections::VecDeque;

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
fn read_line_into_vec<T: FromStr, U: Read>(reader: &mut BufReader<U>) -> Option<Vec<T>>
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
    let mut vec = VecDeque::with_capacity(n as usize);
    // let mut list = List::new();
    // let iter = list.iter();
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    while let Some(line) = read_line_into_vec(&mut reader) {
        let command_num: Vec<String> = line;
        if command_num[0] == "insert" {
            vec.push_front(command_num[1].parse::<i32>().expect("parse error"));
        // list.push(command_num[1].parse::<i32>().unwrap());
        } else if command_num[0] == "delete" {
            let pos = vec
                .iter()
                .position(|x| *x == command_num[1].parse::<i32>().expect("parse error"));
            match pos {
                Some(index) => {
                    let mut tail = vec.split_off(index);
                    tail.pop_front();
                    vec.append(&mut tail);
                }
                None => {}
            };
        // list.delete(&command_num[1].parse::<i32>().unwrap());
        } else if command_num[0] == "deleteFirst" {
            vec.pop_front();
        // list.pop();
        } else if command_num[0] == "deleteLast" {
            vec.pop_back();
        // list.delete_last();
        } else {
            panic!("undefined method");
        }
    }
    buff_write(
        &mut out,
        // &vec_to_string(&(list.iter().map(|&elem| elem).collect())),
        &vec_to_string(&(vec.iter().map(|&elem| elem).collect())),
        // &vec_to_string(&vec),
    );
}

pub struct List<T: std::cmp::PartialEq> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T: std::cmp::PartialEq> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn delete(&mut self, elem: &T) {
        let mut list = List::new();
        while let Some(popped_elem) = self.pop() {
            if popped_elem == *elem {
                break;
            } else {
                list.push(popped_elem);
            }
        }
        while let Some(popped_elem) = list.pop() {
            self.push(popped_elem)
        }
    }
    pub fn delete_last(&mut self) {
        let mut cur_link = self.head.as_mut();
        match cur_link {
            Some(ref cur) => match &cur.next {
                Some(next) => {}
                None => {
                    self.pop();
                    return;
                }
            },
            None => return,
        };
        let mut target = None;
        while let Some(cur) = cur_link {
            match cur.next.as_mut() {
                Some(ref next) => {
                    if next.next.is_none() {
                        target = Some(cur);
                        break;
                    }
                }
                None => {
                    panic!("unreached");
                }
            };
            cur_link = cur.next.as_mut();
        }
        // let mut link = cur_link;
        match target {
            Some(last_2th_node) => {
                (*last_2th_node).next = None;
            }
            None => panic!("unreachable"),
        };
        // cur_link.next = None;
        // match cur_link.as_mut() {
        //     Some(last_2th_node) => last_2th_node.next = None,
        //     None => panic!("unreachable"),
        // };
        // let mut next = cur_link.next.is_none()
        // while let Some(mut boxed_node) = cur_link {
        //     if boxed_node.next.is_none() {
        //         break;
        //     }
        //     cur_link = boxed_node.next.as_ref();
        // }
        // let mut last_node = cur_link.as_mut();
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<T: std::cmp::PartialEq> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T: std::cmp::PartialEq>(List<T>);

impl<T: std::cmp::PartialEq> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T: std::cmp::PartialEq> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}
