use serde::{Deserialize, Serialize};
use super::{rules::{Rules, RulesChecker}, types::{Downloads, Natives}};

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
  pub name: String,
  pub url: Option<String>,
  pub downloads: Option<Downloads>,
  pub natives: Option<Natives>,
  pub rules: Option<Rules>,
  #[serde(rename="downloadOnly")]
  pub download_only: Option<bool>
}

pub(crate) type Libraries = Vec<Library>;

pub trait LibrariesCollect {
  fn collect(&mut self) -> Vec<String>;
}

impl LibrariesCollect for Libraries {
  fn collect(&mut self) -> Vec<String> {
    let mut result = vec![];

    self.into_iter()
      .filter(|lib| lib.rules.as_ref().map_or(true, |rule| rule.is_followed()))
      .filter(|lib| !lib.download_only.unwrap_or_default())
      .for_each(|lib| { (!result.contains(&lib.name)).then(|| {result.push(lib.name.clone())}); });

    result
  }
}