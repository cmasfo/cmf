
pub fn flush() {
  std::io::Write::flush(
    &mut std::io::stdout()
  ).unwrap();
}

#[macro_export]
macro_rules! printfl {
  () => {{
    $crate::flush();
  }};
  ($($arg:tt)*) => {{
    print!($($arg)*);
    flush();
  }};
}

pub fn get_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  s.trim().to_string()
}

#[macro_export]
macro_rules! msg_line {
  () => {{
    get_line()
  }};
  ($($arg:tt)*) => {{
    printfl!($($arg)*);
    get_line()
  }};
}
