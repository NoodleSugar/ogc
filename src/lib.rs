use derive_more::From;
use reqwest::{Client, Response, Url};
use thiserror::Error;

pub mod ows;
pub mod wfs;
pub mod wms;
pub mod wmts;

trait OgcRequest {
	async fn get(&self, client: &Client, url: &Url) -> OgcResult<Response> {
		let request = client.get(url.clone()).query(&self.parameters());

		let response = request.send().await?.error_for_status()?;

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

#[derive(Clone, From)]
pub struct Bbox {
	pub min_lat: String,
	pub min_lon: String,
	pub max_lat: String,
	pub max_lon: String,
}

pub type OgcResult<T> = Result<T, OgcError>;
