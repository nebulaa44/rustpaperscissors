use std::io::{Write, stdin, stdout};

pub fn game_loop() 
{
  let mut player_choice: String = String::new();

  loop 
  {
    println!("[r]rock\n[p]aper\n[scissors]");
    print!("Enter your guess: ");
    stdout()
      .flush()
      .expect("Could not flush output");

    player_choice.clear();
    stdin()
      .read_line(&mut player_choice)
      .expect("Could not read input");
  }
}