pub mod capabilities;

use reqwest::{Client, Url};
use thiserror::Error;

use crate::wmts::capabilities::{Capabilities, GetCapabilitiesRequest};

pub struct WmtsClient {
	client: Client,
	url: Url,
}

impl WmtsClient {
	pub fn new(url: Url) -> Self {
		let client = Client::new();

		Self { client, url }
	}

	pub async fn get_capabilities(&self) -> Result<Capabilities> {
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
}

#[derive(Debug, Error)]
pub enum Error {
	#[error("Request error: {0}")]
	Request(#[from] reqwest::Error),
	#[error("Deserialize error: {0}")]
	Deserialize(#[from] quick_xml::DeError),
}

pub type Result<T> = std::result::Result<T, Error>;
