use reqwest::{Client, Response, Url};

use super::{SERVICE, SrsOrCrs, map::GetMapRequest};

use crate::{OgcRequest, OgcResult, wms::WmsVersion};

pub struct GetFeatureInfoRequestBuilder {
	get_map_request: GetMapRequest,
	query_layers: Vec<String>,
	i: u32,
	j: u32,
	info_format: Option<String>,
	feature_count: Option<u32>,
}

impl GetFeatureInfoRequestBuilder {
	pub fn new(
		get_map_request: GetMapRequest,
		query_layers: impl IntoIterator<Item = String>,
		i: u32,
		j: u32,
	) -> Self {
		let query_layers = query_layers.into_iter().collect();

		Self {
			get_map_request,
			query_layers,
			i,
			j,
			info_format: None,
			feature_count: None,
		}
	}

	pub fn with_info_format(self, info_format: String) -> Self {
		Self {
			info_format: Some(info_format),
			..self
		}
	}

	pub fn with_feature_count(self, feature_count: u32) -> Self {
		Self {
			feature_count: Some(feature_count),
			..self
		}
	}

	pub fn build(self) -> GetFeatureInfoRequest {
		let GetMapRequest {
			version,
			layers,
			styles,
			rs,
			bbox,
			width,
			height,
			format: _,
		} = self.get_map_request;

		let ij = match version {
			super::WmsVersion::V1_0_0 | super::WmsVersion::V1_1_0 | super::WmsVersion::V1_1_1 => {
				XyOrIj::Xy(self.i, self.j)
			}
			super::WmsVersion::V1_3_0 => XyOrIj::Ij(self.i, self.j),
		};

		GetFeatureInfoRequest {
			version,
			layers,
			styles,
			rs,
			bbox,
			width,
			height,
			query_layers: self.query_layers,
			ij,
			info_format: self.info_format,
			feature_count: self.feature_count,
		}
	}
}

pub struct GetFeatureInfoRequest {
	version: WmsVersion,
	layers: Vec<String>,
	styles: Vec<String>,
	rs: SrsOrCrs,
	bbox: (String, String, String, String),
	width: u32,
	height: u32,
	query_layers: Vec<String>,
	ij: XyOrIj,
	info_format: Option<String>,
	feature_count: Option<u32>,
}

impl GetFeatureInfoRequest {
	pub async fn send(self, client: &Client, url: &Url) -> OgcResult<Response> {
		self.get(client, url).await
	}
}

impl OgcRequest for GetFeatureInfoRequest {
	fn parameters(&self) -> Vec<(&'static str, String)> {
		let layers = self.layers.join(",");
		let styles = self.styles.join(",");
		let query_layers = self.query_layers.join(",");
		let bbox = format!(
			"{},{},{},{}",
			self.bbox.0, self.bbox.1, self.bbox.2, self.bbox.3
		);

		let (key_i, key_j) = self.ij.parameter_keys();
		let (i, j) = self.ij.values();

		let mut parameters = vec![
			("SERVICE", SERVICE.to_string()),
			self.version.as_request_parameter(),
			("REQUEST", "GetFeatureInfo".to_string()),
			("LAYERS", layers),
			("STYLES", styles),
			("BBOX", bbox),
			self.rs.as_request_parameter(),
			("WIDTH", self.width.to_string()),
			("HEIGHT", self.height.to_string()),
			("QUERY_LAYERS", query_layers),
			(key_i, i.to_string()),
			(key_j, j.to_string()),
		];

		if let Some(info_format) = &self.info_format {
			parameters.push(("INFO_FORMAT", info_format.clone()));
		};

		if let Some(feature_count) = self.feature_count {
			parameters.push(("FEATURE_COUNT", feature_count.to_string()));
		};

		parameters
	}
}

pub struct FeatureInfo {}

#[derive(Clone, Copy)]
pub(super) enum XyOrIj {
	Xy(u32, u32),
	Ij(u32, u32),
}

impl XyOrIj {
	fn parameter_keys(self) -> (&'static str, &'static str) {
		match self {
			XyOrIj::Xy(_, _) => ("X", "Y"),
			XyOrIj::Ij(_, _) => ("I", "J"),
		}
	}

	fn values(self) -> (u32, u32) {
		match self {
			XyOrIj::Xy(x, y) => (x, y),
			XyOrIj::Ij(i, j) => (i, j),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{Bbox, wms::map::GetMapRequestBuilder};

	use super::*;

	#[tokio::test]
	async fn get_feature_info_for_infoterre() {
		let url = "https://infoterre.brgm.fr/services/gfi".parse().unwrap();

		let request = GetFeatureInfoRequestBuilder::new(
			GetMapRequestBuilder::new(
				"EPSG:4326".to_string(),
				Bbox {
					min_lat: "45.34114350490152".to_string(),
					min_lon: "5.5708309459012835".to_string(),
					max_lat: "45.359139049544474".to_string(),
					max_lon: "5.604859964098717".to_string(),
				},
				2000,
				1499,
				"image/png".to_string(),
			)
			.with_layers_and_styles([("SCAN_GEOL50".to_string(), "".to_string())])
			.build(WmsVersion::V1_1_1),
			["SCAN_GEOL50".to_string()],
			1000,
			750,
		)
		.with_feature_count(1)
		.with_info_format("application/vnd.ogc.gml".to_string())
		.build();

		let reponse = request.send(&Client::new(), &url).await.unwrap();
		println!("{}", reponse.text().await.unwrap());
	}
}
