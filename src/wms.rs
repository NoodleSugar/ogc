use reqwest::{Client, Url};
use thiserror::Error;

use crate::wms::map::{GetMapRequest, Map};

mod map;

#[derive(Debug, Clone)]
pub struct WmsClient {
	client: Client,
	url: Url,
}

impl WmsClient {
	pub fn new(url: Url) -> Self {
		let client = Client::new();

		Self { client, url }
	}

	pub async fn get_map(&self, request: &GetMapRequest) -> WmsResult<Map> {
		let response = self
			.client
			.get(self.url.clone())
			.query(&request.parameters())
			.send()
			.await?
			.error_for_status()?;

		let bytes = response.bytes().await?.to_vec();

		Ok(Map { bytes })
	}
}

#[derive(Debug, Error)]
pub enum WmsError {
	#[error("Request error: {0}")]
	Request(#[from] reqwest::Error),
	#[error("Deserialize error: {0}")]
	Deserialize(#[from] quick_xml::DeError),
}

pub type WmsResult<T> = std::result::Result<T, WmsError>;

const SERVICE: &str = "WMS";
