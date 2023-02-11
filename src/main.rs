#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;
use std::collections::HashMap;


fn main() {
  let mut map = HashMap::new();
  map.insert(1, 2);
  map.insert(2, 3);
  map.insert(3, 4);

  // sort HashMap by key
  let mut keys: Vec<_> = map.keys().collect();
  keys.sort();
  for key in keys {
    println!("{}: {}", key, map[key]);
  }

  // for (key, value) in &map {
  //   println!("{}: {}", key, value);
  // }

  //iterate
  for (key, value) in map.iter() {
    println!("{}: {}", key, value);
  }
}
