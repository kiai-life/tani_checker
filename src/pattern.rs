pub mod coins;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CreditsInfo {
  pub credits: f32,
  pub pattern: CreditsPattern,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CreditsPattern {
  Only(f32),
  Width(f32, Option<f32>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Credits {
  CreditsData(CreditsData),
  CreditsInfo(CreditsInfo),
}

#[derive(Clone, PartialEq, Debug)]
pub struct CreditsData {
  pub credits_list: Vec<Credits>,
  pub pattern: CreditsPattern,
}

pub fn msg_from_credits_info(credits_info: &CreditsInfo) -> String {
  String::new()
}

pub fn msg_from_credits_data(credits_data: &CreditsData) -> String {
  String::new()
}
