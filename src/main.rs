#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // Vectors
    let str3 = String::from("x r t b f g h j k k l");
    let mut v1: Vec<char> = str3.chars().collect();

    println!("v1: {:?}", v1);

    v1.sort();

    println!("v1: {:?}", v1);

    v1.dedup();

    println!("v1: {:?}", v1);

    let st4: &str = "Random String";

    let mut st5: String = String::from(st4);

    println!("st5: {}", st5);

    let mut st6 = &st4[2..5];

    println!("st6: {}", st6);


}
