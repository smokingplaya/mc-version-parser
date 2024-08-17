use super::{arguments::Arguments, libraries::Libraries};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct VersionData {
  pub id: String,
  pub jar: String,
  pub family: String,
  pub time: String,
  pub mainClass: String,
  pub releaseTime: String,
  pub r#type: String, // type: String
  pub libraries: Libraries, // biggest vector in this struct (mostly)
  pub downloads: Classifiers,
  pub assets: String,
  pub assetIndex: AssetIndex,
  pub arguments: Arguments
}

pub(crate) type Natives = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Downloads {
  pub artifact: Option<Artifact>,
  pub classifiers: Option<Classifiers>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artifact {
  pub url: String,
  pub sha1: String,
  pub size: u32
}

pub type Classifiers = HashMap<String, Artifact>;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AssetIndex {
  pub totalSize: u32,
  pub id: String,
  pub known: bool,
  pub url: String,
  pub sha1: String,
  pub size: u32
}