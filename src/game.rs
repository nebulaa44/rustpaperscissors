use crate::logic::Action;

use std::io::{Write, stdin, stdout};

pub fn game_loop() 
{
  let mut player_choice: String = String::new();

  loop 
  {
    // prompt user for their choice
    println!("[r]rock\n[p]aper\n[scissors]");
    print!("Enter your guess: ");
    stdout()
      .flush()
      .expect("Could not flush output");

    // actually read the guess
    player_choice.clear();
    stdin()
      .read_line(&mut player_choice)
      .expect("Could not read input");

    let player_action = Action::from_string(&player_choice);
    if player_action == Action::Other {continue}
    
    println!("You chose {player_action:?}");

  }
}