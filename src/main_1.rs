#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};


fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
  x + y
}

fn sum_list(list: &[i32]) -> i32 {
  let mut sum = 0;
  for i in list {
    sum += i;
  }
  sum
}


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

  let st4: &str = "Random String";

  let mut st5: String = String::from(st4);

  println!("st5: {}", st5);

  let mut st6 = &st4[2..5];

  println!("st6: {}", st6);

  let st7 = st5 + &st6;

  println!("st7: {:?}", st7.to_string());

  // Enums
  enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
  }

  impl Days {
    fn is_weekend(&self) -> bool {
      match self {
        Days::Saturday | Days::Sunday => true,
        _ => false,
      }
    }
  }

  let today:Days = Days::Monday;
  match today {
    Days::Monday => println!("Monday"),
    Days::Tuesday => println!("Tuesday"),
    Days::Wednesday => println!("Wednesday"),
    Days::Thursday => println!("Thursday"),
    Days::Friday => println!("Friday"),
    Days::Saturday => println!("Saturday"),
    Days::Sunday => println!("Sunday"),
  }

  // Vectors
  let str3 = String::from("x r t b f g h j k k l");
  let mut v1: Vec<char> = str3.chars().collect();

  println!("v1: {:?}", v1);

  v1.sort();

  println!("v1: {:?}", v1);

  v1.dedup();

  println!("v1: {:?}", v1);

  let vec1: Vec<i32> = Vec::new();
  
  println!("vec1: {:?}", vec1);

  let mut vec2 = vec![1, 2, 3];
  vec2.push(4);
  println!("vec2: {:?}", vec2);
  println!("1st element is: {}", vec2[0]);

  let second: &i32 = &vec2[1];
  match vec2.get(1) {
    Some(second) => println!("Second element is: {}", second),
    None => println!("There is no second element"),
  }

  for i in &mut vec2 {
    *i += 50;
  }

  for i in &vec2 {
    println!("i: {}", i);
  }

  println!("Vec length: {}", vec2.len());
  println!("Pop: {:?}", vec2.pop());


  // Functions
  let list = [1, 2, 3, 4, 5];
  println!("Sum: {}", sum_list(&list));

  println!("Sum: {}", get_sum_gen(1, 2));
  println!("Sum: {}", get_sum_gen(1.1, 2.22) as f32);

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


  // struct
  struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
  }

  let user: User = User {
    username: String::from("user"),
    email: String::from("s@mail.com"),
    sign_in_count: 1,
    active: true,
  };

  println!("User: {}", user.username);


  // shape
  trait Shape {
    fn new(length: f32, width: f32) -> Self;
    fn area(&self) -> f32;
  };

  struct Rectangle {
    length: f32,
    width: f32,
  }


  impl Shape for Rectangle {
    fn new(length: f32, width: f32) -> Self {
      Rectangle { length, width }
    }

    fn area(&self) -> f32 {
      self.length * self.width
    }
  }

  println!("Rectangle area: {}", Rectangle::new(10.0, 10.0).area());
}
