use rand::{
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

// TODO: implement display
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
      // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
      match rng.gen_range(0..=2) { // rand 0.8
          0 => Action::Rock,
          1 => Action::Paper,
          _ => Action::Scissors,
      }
  }
}