use rand::
{
  distributions::{Distribution, Standard},
  Rng,
};

#[derive(Debug, PartialEq)]
pub enum Action 
{
  Rock,
  Paper,
  Scissors,
  Other
}

#[derive(Debug)]
pub enum Verdict
{
  Win,
  Draw,
  Loss
}

impl Verdict
{
  // todo: learn how to not be yanderedev
  pub fn from_actions(player: &Action, computer: &Action) -> Self
  {
    if player == &Action::Rock && computer == &Action::Paper {return Verdict::Loss}
    else if player == &Action::Rock && computer == &Action::Scissors {return Verdict::Win}

    else if player == &Action::Paper && computer == &Action::Rock {return Verdict::Win}
    else if player == &Action::Paper && computer == &Action::Scissors {return Verdict::Loss}

    else if player == &Action::Scissors && computer == &Action::Rock {return Verdict::Loss}
    else if player == &Action::Scissors && computer == &Action::Paper {return Verdict::Win}

    else {return Verdict::Draw}
  }
}

impl Action
{
  pub fn from_string(input: &String) -> Self
  {
    match input.trim()
    {
      "r" => Action::Rock,
      "p" => Action::Paper,
      "s" => Action::Scissors,
      _ => Action::Other
    }
  }
}

impl Distribution<Action> for Standard 
{
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Action 
  {
      match rng.gen_range(0..=2) 
      {
          0 => Action::Rock,
          1 => Action::Paper,
          _ => Action::Scissors,
      }
  }
}