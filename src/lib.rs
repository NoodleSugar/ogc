use reqwest::{Client, Response, Url};
use serde::Serialize;
use thiserror::Error;

pub mod ows;
pub mod wms;
pub mod wmts;

#[derive(Debug, Clone)]
pub struct OgcClient {
	client: Client,
	url: Url,
}

impl OgcClient {
	pub fn new(url: Url) -> Self {
		let client = reqwest::Client::new();

		Self { client, url }
	}

	pub async fn get<T: Serialize + ?Sized>(&self, parameters: &T) -> OgcResult<Response> {
		let response = self
			.client
			.get(self.url.clone())
			.query(parameters)
			.send()
			.await?
			.error_for_status()?;

		Ok(response)
	}
}

#[derive(Debug, Error)]
pub enum OgcError {
	#[error("Request error: {0}")]
	Request(#[from] reqwest::Error),
	#[error("Deserialize error: {0}")]
	Deserialize(#[from] quick_xml::DeError),
}

pub type OgcResult<T> = Result<T, OgcError>;
