#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Hello, ";
    io::stdin().read_line(&mut name).expect("Didn't receive a name");


    println!("Hello, {}!", name.trim_end());

    // random range
    let random = rand::thread_rng().gen_range(1..101);

    for i in 1..101 {
        println!("{} ", i);
    }

    println!("Random number: {}", random);

    if (random % 2) == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }

    // max values
    println!("Max u32 value: {}", u32::MAX);
    println!("Max u64 value: {}", u64::MAX);
    println!("Max f32 value: {}", f32::MAX);


    // sum
    let num_1: f32 = 1.1111;
    let num_2: f64 = 1.1111111111111111;
    println!("num_1: {}", num_1);

    // 2.2222111
    println!("sum: {}", num_1 + num_2 as f32);
    
    // 2.222211069530911
    println!("f64: {}", num_2 + num_1 as f64);


    // IF statements

    let age = 18;

    if (age >= 21) {
        println!("You can drink");
    } else if (age >= 18) {
        println!("You can vote");
    } else {
        println!("You can't do anything");
    }


    // Pattern matching
    let age2 = 22;

    match age2 {
        0 => println!("Baby"),
        1..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        20..=65 => println!("Adult"),
        _ => println!("Senior"),
    }
        

    let age3 = 22;

    match age3.cmp(&18) {
        Ordering::Less => println!("Too young"),
        Ordering::Equal => println!("Just right"),
        Ordering::Greater => println!("Too old"),
    }


    // LOOPS

    let arr_1 = [1, 2, 3, 4, 5];
    let arr_2 = [1; 5];

    println!("arr_1: {:?}", arr_1[1]);

    for val in arr_1.iter() {
        println!("arr_1 val: {}", val);
    }

    for (index, item) in arr_1.iter().enumerate() {
        println!("arr_1 item: {} at index: {}", item, index);
    }

    println!("arr_2: {:?}", arr_2.len());

    let mut idx = 0;
    loop {
        if idx == arr_1.len() {
            break;
        }
        println!("arr_1 val: {}", arr_1[idx]);
        idx += 1;
    }

    idx = 0;
    while idx < arr_2.len() {
        println!("arr_2 val: {}", arr_2[idx]);
        idx += 1;
    }


    // Tuples
    let my_tuple = (1, "42", 3.14);

    println!("my_tuple: {:?}", my_tuple);

    // pointer
    println!("my_tuple.1: {}", my_tuple.1);

    // destructuring
    let (x, y, z) = my_tuple;
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    // Strings
    let mut str = String::new();
    str.push('A');
    str.push_str(" Adam");
    str.push_str(" Ben");

    println!("{}", str);

    for word in str.split_whitespace() {
        println!("{}", word);
    }

    for c in str.chars() {
        println!("{}", c);
    }

    for byte in str.bytes() {
        println!("{}", byte);
    }

    let str2 = str.replace("Adam", "Adam2");

    println!("{}", str2);
}
