//! https://schemas.opengis.net/ows/1.1.0/owsServiceIdentification.xsd

use serde::Deserialize;

use super::iso_19115::{KeywordsType, LanguageStringType};

/// https://schemas.opengis.net/ows/1.1.0/owsServiceIdentification.xsd

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceIdentification {
	#[serde(default, rename = "Title")]
	pub titles: Vec<LanguageStringType>,
	#[serde(default, rename = "Abstract")]
	pub abstracts: Vec<LanguageStringType>,
	#[serde(rename = "Keywords")]
	pub keywords: KeywordsType,
}
