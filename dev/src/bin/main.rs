
#![allow(unused)]

use dev::*;

fn main() {
  let s = msg_line!("What is your name? ");
  println!("Hello {}", s);
}
