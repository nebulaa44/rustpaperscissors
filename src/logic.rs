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

pub enum Verdict
{
  Win,
  Draw,
  Loss
}

impl Verdict
{
  // todo: actually implement this
  pub fn from_actions(player: &Action, computer: &Action) -> Self
  {
    Verdict::Draw 
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