pub mod capabilities;
pub mod tile;

use reqwest::{Client, Url};
use thiserror::Error;

use crate::wmts::{
	capabilities::{Capabilities, GetCapabilitiesRequest},
	tile::{GetTileRequest, Tile},
};

#[derive(Debug, Clone)]
pub struct WmtsClient {
	client: Client,
	url: Url,
}

impl WmtsClient {
	pub fn new(url: Url) -> Self {
		let client = Client::new();

		Self { client, url }
	}

	pub async fn get_capabilities(&self) -> WmtsResult<Capabilities> {
		let response = self
			.client
			.get(self.url.clone())
			.query(&GetCapabilitiesRequest.parameters())
			.send()
			.await?
			.error_for_status()?;

		let content = response.text().await?;

		let result = quick_xml::de::from_str::<Capabilities>(&content)?;

		Ok(result)
	}

	pub async fn get_tile(&self, request: GetTileRequest) -> WmtsResult<Tile> {
		let response = self
			.client
			.get(self.url.clone())
			.query(&request.parameters())
			.send()
			.await?
			.error_for_status()?;

		let bytes = response.bytes().await?.to_vec();

		Ok(Tile { bytes })
	}
}

#[derive(Debug, Error)]
pub enum WmtsError {
	#[error("Request error: {0}")]
	Request(#[from] reqwest::Error),
	#[error("Deserialize error: {0}")]
	Deserialize(#[from] quick_xml::DeError),
}

pub type WmtsResult<T> = std::result::Result<T, WmtsError>;

const SERVICE: &str = "WMTS";
