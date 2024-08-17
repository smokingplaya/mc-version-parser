use super::rules::{Rules, RulesChecker};
use std::collections::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Argument {
  pub value: String,
  pub rules: Option<Rules>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Arguments {
  jvm: ArgumentList,
  game: ArgumentList
}

pub(crate) type ArgumentList = Vec<Argument>;

/// *CollectedArgument
/// example:
///   let data = parser::parse()?;
///   let arguments = data.arguments.collect();
///   let cmd = Command::new("javaw").args(arguments);

pub(crate) type CollectedArguments = Vec<String>;

pub trait Substitution {
  fn values(&mut self, values: Values) -> Vec<String>;
  fn append_arguments(&mut self, args: &  ArgumentList) -> Vec<String>;
}

// todo
impl Substitution for CollectedArguments {
  fn values(&mut self, values: Values) -> Vec<String> {
    let re = Regex::new(r"\$\{([^\}]+)\}").expect("Invalid regex");

    self.into_iter()
      .map(|str| {
        re.replace_all(&str, |caps: &regex::Captures| {
          let key = &caps[1];
          values.get(key).unwrap_or(&&caps[0].to_string()).to_string()
        })
        .to_string()
      })
      .collect()
  }

  // todo
  fn append_arguments(&mut self, args: &ArgumentList) -> Vec<String> {
    let mut result = self.clone();

    args.into_iter()
      .filter(|arg| arg.rules.as_ref().map_or(true, |r| r.is_followed()))
      .for_each(|arg| result.push(arg.value.clone()));

    result
  }
}

pub(crate) type Values = HashMap<String, String>;

/// Arguments type implementation

impl Arguments {
  pub fn collect(&mut self) -> CollectedArguments {
    let mut args: CollectedArguments = vec![];

    args.append_arguments(&self.jvm)
      .append_arguments(&self.game)
  }
}