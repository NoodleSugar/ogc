use reqwest::{Client, Response, Url};

use crate::{Bbox, OgcRequest, OgcResult};

use super::SERVICE;

pub struct GetFeatureRequest {
	pub type_name: String,
	pub property_names: Option<Vec<String>>,
	pub bbox: Option<Bbox>,
}

impl GetFeatureRequest {
	pub async fn send(self, client: &Client, url: &Url) -> OgcResult<Response> {
		self.get(client, url).await
	}
}

impl OgcRequest for GetFeatureRequest {
	fn parameters(&self) -> Vec<(&'static str, String)> {
		let mut parameters = vec![
			("SERVICE", SERVICE.to_string()),
			("VERSION", "2.0.0".to_string()),
			("REQUEST", "GetFeature".to_string()),
			("TYPENAME", self.type_name.clone()),
		];

		if let Some(property_names) = &self.property_names {
			parameters.push(("PROPERTYNAME", property_names.join(",")));
		}

		if let Some(Bbox {
			min_lat,
			min_lon,
			max_lat,
			max_lon,
		}) = &self.bbox
		{
			parameters.push(("BBOX", format!("{min_lat},{min_lon},{max_lat},{max_lon}",)));
		}

		parameters
	}
}

// BBOX=45.349475,5.587109,45.350953,5.58986
