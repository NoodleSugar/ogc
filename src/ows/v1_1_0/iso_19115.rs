//! https://schemas.opengis.net/ows/1.1.0/ows19115subset.xsd

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LanguageStringType {
	#[serde(rename = "@xml:lang")]
	pub lang: Option<String>,
	#[serde(rename = "$text")]
	pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct KeywordsType {
	#[serde(rename = "Keyword")]
	pub keywords: Vec<LanguageStringType>,
}

#[derive(Debug, Deserialize)]
pub struct CodeType {
	#[serde(rename = "codeSpace")]
	pub code_space: Option<String>,
	#[serde(rename = "$text")]
	pub value: String,
}
