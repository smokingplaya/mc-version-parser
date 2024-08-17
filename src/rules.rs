use std::env;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
  pub action: String,
  pub os: Option<OSInfo>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OSInfo {
  pub name: Option<String>,
  pub version: Option<String>
}

pub(crate) type Rules = Vec<Rule>;

/// is_followed method

impl Rule {
  pub fn is_followed(&self) -> bool {
    let is_allowed = self.action == "allow".to_string();
    self.os.as_ref().map_or(is_allowed, |os| {os.name.as_ref().map_or(is_allowed, |name| {
      match self.action.as_str() {
        "allow" => name == env::consts::OS, // Не учитываем osx
        "disallow" => name != env::consts::OS,
        _ => false
      }
    })})
  }
}

pub(crate) trait RulesChecker {
  fn is_followed(&self) -> bool;
}

impl RulesChecker for Rules {
  fn is_followed(&self) -> bool {
    self.iter().all(|s| s.is_followed())
  }
}