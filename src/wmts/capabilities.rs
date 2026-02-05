use serde::Deserialize;

use crate::{
	OgcClient, OgcResult,
	ows::v1_1_0::{
		iso_19115::{CodeType, LanguageStringType},
		service_identification::ServiceIdentification,
	},
};

/// https://schemas.opengis.net/wmts/1.0/wmtsGetCapabilities_request.xsd
pub struct GetCapabilitiesRequest;

impl GetCapabilitiesRequest {
	const PARAMETERS: &[(&str, &str)] =
		&[("service", super::SERVICE), ("request", "GetCapabilities")];

	pub async fn send(self, client: &OgcClient) -> OgcResult<Capabilities> {
		let response = client.get(Self::PARAMETERS).await?;
		let content = response.text().await?;

		let result = quick_xml::de::from_str::<Capabilities>(&content)?;

		Ok(result)
	}
}

/// https://schemas.opengis.net/wmts/1.0/wmtsGetCapabilities_response.xsd
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Capabilities {
	pub service_identification: ServiceIdentification,
	pub contents: ContentsType,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContentsType {
	#[serde(rename = "Layer")]
	pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Layer {
	pub identifier: CodeType,
	#[serde(default, rename = "Title")]
	pub titles: Vec<LanguageStringType>,
	#[serde(rename = "Format")]
	pub formats: Vec<String>,
	#[serde(rename = "Style")]
	pub styles: Vec<Style>,
	pub tile_matrix_set_link: Vec<TileMatrixSetLink>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Style {
	pub identifier: String,
	#[serde(rename = "@isDefault")]
	pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TileMatrixSetLink {
	pub tile_matrix_set: String,
	pub tile_matrix_set_limits: Option<TileMatrixSetLimits>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TileMatrixSetLimits {
	pub tile_matrix_limits: Vec<TileMatrixLimits>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TileMatrixLimits {
	pub tile_matrix: String,
	pub min_tile_row: u64,
	pub max_tile_row: u64,
	pub min_tile_col: u64,
	pub max_tile_col: u64,
}

#[cfg(test)]
mod tests {
	use super::*;

	const RESOURCE_PATH: &str = concat!(
		env!("CARGO_MANIFEST_DIR"),
		"/resources/test/wmts/capabilities"
	);

	#[test]
	fn deserialization_should_succeed() {
		let file_path = format!("{RESOURCE_PATH}/geopf.xml");
		let content = std::fs::read_to_string(file_path).unwrap();

		let result = quick_xml::de::from_str::<Capabilities>(&content);

		assert!(result.is_ok())
	}
}
