use types::VersionData;
use std::{fs::File, io::{self, BufReader}};

pub mod types;
pub(crate) mod rules;
pub mod arguments;
pub mod libraries;

pub fn parse(file: File) -> io::Result<VersionData> {
  let reader = BufReader::new(file);
  let data = serde_json::from_reader(reader)?;

  Ok(data)
}