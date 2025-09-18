pub mod coins22;

pub trait Credits {
  fn msg(&self) -> String;
  fn credits(&self) -> usize;
  fn all_credits(&self) -> usize;
  fn is_ok(&self) -> bool;
}

pub trait DynCredits: Credits {
  fn clone_boxed(&self) -> Box<dyn DynCredits>;
}

impl<T> DynCredits for T
where
  T: Credits + Clone + 'static,
{
  fn clone_boxed(&self) -> Box<dyn DynCredits> {
    Box::new(Clone::clone(self))
  }
}

impl Clone for Box<dyn DynCredits> {
  fn clone(&self) -> Self {
    self.clone_boxed()
  }
}

#[derive(Clone, Debug)]
pub struct CreditsInfo {
  pub name: String,
  pub credits: usize,
  pub pattern: CreditsPattern,
}

impl CreditsInfo {
  pub fn new(name: &str, pat: CreditsPattern) -> Self {
    CreditsInfo {
      name: name.to_string(),
      credits: 0,
      pattern: pat,
    }
  }

  pub fn add(&mut self, n: usize) {
    self.credits += n
  }
}

impl Credits for CreditsInfo {
  fn all_credits(&self) -> usize {
    self.credits
  }
  fn credits(&self) -> usize {
    match self.pattern {
      CreditsPattern::Range(_, n) => {
        if n < self.credits {
          n
        } else {
          self.credits
        }
      }
      CreditsPattern::Top(n) => {
        if n < self.credits {
          n
        } else {
          self.credits
        }
      }
      CreditsPattern::Only(n) => {
        if n < self.credits {
          n
        } else {
          self.credits
        }
      }
      CreditsPattern::Bottom(_) => self.credits,
    }
  }
  fn is_ok(&self) -> bool {
    match self.pattern {
      CreditsPattern::Range(n, _) => n <= self.credits,
      CreditsPattern::Top(_) => true,
      CreditsPattern::Only(n) => n <= self.credits,
      CreditsPattern::Bottom(n) => n <= self.credits,
    }
  }
  fn msg(&self) -> String {
    format!(
      "{} {}： {}({})/{}",
      if self.is_ok() { '✅' } else { '❌' },
      self.name,
      self.credits(),
      self.all_credits(),
      self.pattern.to_credits_pattern_string()
    )
  }
}

/// 単位のパターン
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CreditsPattern {
  /// 単位数の指定
  Only(usize),
  /// 単位数の幅がある
  Range(usize, usize),
  /// 単位数の上限
  Top(usize),
  /// 単位数の下限
  Bottom(usize),
}

impl CreditsPattern {
  fn to_credits_pattern_string(self) -> String {
    match self {
      CreditsPattern::Only(n) => n.to_string(),
      CreditsPattern::Range(n, m) => {
        format!("{n}～{m}")
      }
      CreditsPattern::Top(n) => {
        format!("～{n}")
      }
      CreditsPattern::Bottom(n) => {
        format!("{n}～")
      }
    }
  }
}

/// 体育の単位
#[derive(Clone, Debug)]
pub struct CreditsPE {
  pub name: String,
  pub credits: usize,
}

impl CreditsPE {
  fn new(name: &str) -> Self {
    CreditsPE {
      name: name.to_string(),
      credits: 0,
    }
  }
  fn get(&mut self) {
    self.credits += 1;
  }
}

impl Credits for CreditsPE {
  fn all_credits(&self) -> usize {
    self.credits()
  }
  fn credits(&self) -> usize {
    if self.is_ok() {
      1
    } else {
      0
    }
  }
  fn is_ok(&self) -> bool {
    self.credits == 2
  }
  fn msg(&self) -> String {
    format!(
      "{} {}：{}/1",
      if self.is_ok() { '✅' } else { '❌' },
      self.name,
      self.credits()
    )
  }
}

#[derive(Clone)]
pub struct CreditsData {
  pub msg_prefix: String,
  pub name: String,
  pub credits_list: Vec<Box<dyn DynCredits>>,
  pub pattern: CreditsPattern,
}

impl Credits for CreditsData {
  fn all_credits(&self) -> usize {
    self.credits_list.iter().map(|t| t.all_credits()).sum()
  }
  fn credits(&self) -> usize {
    let credits = self.credits_list.iter().map(|t| t.credits()).sum();
    match self.pattern {
      CreditsPattern::Range(_, n) => {
        if n < credits {
          n
        } else {
          credits
        }
      }
      CreditsPattern::Top(n) => {
        if n < credits {
          n
        } else {
          credits
        }
      }
      CreditsPattern::Only(n) => {
        if n < credits {
          n
        } else {
          credits
        }
      }
      CreditsPattern::Bottom(_) => credits,
    }
  }

  fn is_ok(&self) -> bool {
    let credits = self.credits_list.iter().map(|t| t.credits()).sum();
    let is_ok = self.credits_list.iter().all(|t| t.is_ok());
    let b = match self.pattern {
      CreditsPattern::Range(n, _) => n <= credits,
      CreditsPattern::Top(_) => true,
      CreditsPattern::Only(n) => n <= credits,
      CreditsPattern::Bottom(n) => n <= credits,
    };
    is_ok && b
  }

  fn msg(&self) -> String {
    format!(
      "{}{} {}： {}({})/{}",
      self.msg_prefix,
      if self.is_ok() { '✅' } else { '❌' },
      self.name,
      self.credits(),
      self.all_credits(),
      self.pattern.to_credits_pattern_string()
    )
  }
}
