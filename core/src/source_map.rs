use serde::Serialize;

use crate::magic_string::DecodedMap;
use crate::mapping::Mapping;
use crate::result::Result;

// current specification version
static VERSION: u8 = 3;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceMap {
  pub version: u8,
  pub file: Option<String>,
  pub mappings: String,
  pub sources_content: Vec<Option<String>>,
  pub source_root: Option<String>,
  pub names: Vec<String>,
  pub sources: Vec<Option<String>>,
}

impl SourceMap {
  pub fn new(
    mappings: &str,
    file: Option<String>,
    names: Vec<String>,
    sources_content: Vec<Option<String>>,
    source_root: Option<String>,
    sources: Vec<Option<String>>,
  ) -> Self {
    Self {
      version: VERSION,
      mappings: String::from(mappings),
      file,
      names,
      sources_content,
      source_root,
      sources,
    }
  }

  pub fn new_from_decoded(decoded_map: DecodedMap) -> Result<Self> {
    Ok(Self {
      version: VERSION,
      file: decoded_map.file,
      mappings: Mapping::generate_encoded_mappings(&decoded_map.mappings)?,
      names: decoded_map.names,
      source_root: decoded_map.source_root,
      sources_content: decoded_map.sources_content,
      sources: decoded_map.sources,
    })
  }

  pub fn to_json(&self) -> Result<String> {
    Ok(serde_json::to_string(self)?)
  }

  pub fn to_url(&self) -> Result<String> {
    let str = Self::to_json(self)?;

    Ok(format!(
      "data:application/json;charset=utf-8;base64,{}",
      base64::encode(str)
    ))
  }
}
