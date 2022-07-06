#[allow(dead_code)]

pub enum Action 
{
  Rock,
  Paper,
  Scissors,
  Other
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