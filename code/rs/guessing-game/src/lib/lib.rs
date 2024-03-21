
#![allow(unused)]

pub mod prelude;
pub use prelude::*;

struct GameManager {
  max_number: u128,
  secret_number: u128,
  min_guess: u128,
  max_guess: u128,
  tries: u128,
}

impl GameManager {
  fn new(difficulty: u8) -> Self {
    let max_number = match difficulty {
      1 => u8::MAX.into(),
      2 => u16::MAX.into(),
      3 => u32::MAX.into(),
      4 => u64::MAX.into(),
      _ => u128::MAX,
    };

    let mut rng = rand::thread_rng();

    use rand::Rng;
    let secret_number = rng.gen_range(
      0..=max_number
    );

    println!("The number is decided.");
    println!("The range is 0 to {}.", max_number);

    Self {
      max_number,
      secret_number,
      min_guess: 0,
      max_guess: max_number,
      tries: 0
    }
  }

  fn run(&mut self) {
    'round_loop: loop {
      let current_guess = 'guess_loop: loop {
        println!();
        printfl!("What is your guess? ");
        println!("({} - {})", self.min_guess, self.max_guess);
        let guess_input = msg_line!("> ");
        if let Ok(parse) = guess_input.parse::<u128>() {
          if parse <= self.min_guess || parse >= self.max_guess {
            println!("This try is meaningless.");
          } else {
            self.tries += 1;
            break 'guess_loop parse;
          }
        } else {
          printfl!("Failed to parse.");
          get_line();
        };
      };

      if current_guess == self.secret_number {
        println!("Correct.");
        println!("The answer was {}.", self.secret_number);
        println!("You tried {} times.", self.tries);
        break 'round_loop;
      } else if current_guess < self.secret_number {
        println!("Incorrect.");
        self.min_guess = current_guess;
        println!("The range is now {} - {}.",
        self.min_guess, self.max_guess);
        println!("You tried {} times.", self.tries);
        continue 'round_loop;
      } else {
        println!("Incorrect.");
        self.max_guess = current_guess;
        println!("The range is now {} - {}.",
        self.min_guess, self.max_guess);
        println!("You tried {} times.", self.tries);
        continue 'round_loop;
      }
    }
  }
}

pub fn game_loop() {
  'game_loop: loop {
    let choice: u8 = get_choice();
    
    GameManager::new(choice).run();

    'decision_loop: loop {
      let decision = msg_line!("Continue?(Y/n) ");
      if decision.is_empty() {
        break 'decision_loop;
      }

      match decision.as_str() {
        "y" | "Y" => break 'decision_loop,
        "n" | "N" => break 'game_loop,
        _ => {
          println!("Failed to parse.");
        }
      }
    }
  }
}

fn get_choice() -> u8 {
  loop {
    println!();
    println!("Choose the difficulty.");
    println!("1. Easy (u8)");
    println!("2. Medium (u16)");
    println!("3. Hard (u32)");
    println!("4. Extreme (u64)");
    println!("5. Nightmare (u128)");

    let input = msg_line!("Your choice: ");
    if let Ok(parse) = input.parse::<u8>() {
      match parse {
        1..=5 => {
          return parse;
        }
        _ => {
          println!();
          printfl!("Range is out of 1-5.");
          get_line();
        }
      }
    } else {
      println!();
      printfl!("Failed to parse to u8.");
      get_line();
    }
  }
}
