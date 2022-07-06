#[allow(dead_code)]

pub enum Action 
{
  Rock,
  Paper,
  Scissors
}

impl Action
{
  pub fn from_string(input: &String) -> Option<Self>
  {
    match input.trim()
    {
      "r" => Some(Action::Rock),
      "p" => Some(Action::Paper),
      "s" => Some(Action::Scissors),
      _ => None
    }
  }
}