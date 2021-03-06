use crate::logic::*;

use std::io::{Write, stdin, stdout};

pub fn game_loop() 
{
  let mut player_choice: String = String::new();

  loop 
  {
    // prompt user for their choice
    println!("[r]rock\n[p]aper\n[s]cissors\n[q]uit");
    print!("Enter your guess: ");
    stdout()
      .flush()
      .expect("Could not flush output");

    // actually read the guess
    player_choice.clear();
    stdin()
      .read_line(&mut player_choice)
      .expect("Could not read input");

    if player_choice.trim() == "q" {return}

    let player_action = Action::from_string(&player_choice);
    if player_action == Action::Other {continue}
    
    let computer_action: Action = rand::random();

    println!("{player_action:?} vs {computer_action:?}");
    
    let verdict = Verdict::from_actions(&player_action, &computer_action);
    println!("{verdict:?}\n");
    
  }
}