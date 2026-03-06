use reqwest::{Client, Response, Url};
use thiserror::Error;

pub mod ows;
pub mod wms;
pub mod wmts;

trait OgcRequest {
	async fn get(&self, client: &Client, url: &Url) -> OgcResult<Response> {
		let response = client
			.get(url.clone())
			.query(&self.parameters())
			.send()
			.await?
			.error_for_status()?;

		Ok(response)
	}

	fn parameters(&self) -> Vec<(&'static str, String)>;
}

#[derive(Debug, Error)]
pub enum OgcError {
	#[error("Request error: {0}")]
	Request(#[from] reqwest::Error),
	#[error("Xml deserialization error: {0}")]
	DeserializeXml(#[from] quick_xml::DeError),
}

pub type OgcResult<T> = Result<T, OgcError>;
