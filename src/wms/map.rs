use reqwest::{Client, Url};

use super::{SERVICE, SrsOrCrs, WmsVersion};

use crate::{Bbox, OgcRequest, OgcResult};

pub struct GetMapRequestBuilder {
	layers: Vec<String>,
	styles: Vec<String>,
	crs: String,
	bbox: Bbox,
	width: u32,
	height: u32,
	format: String,
}

impl GetMapRequestBuilder {
	pub fn new(crs: String, bbox: Bbox, width: u32, height: u32, format: String) -> Self {
		Self {
			layers: Vec::new(),
			styles: Vec::new(),
			crs,
			bbox,
			width,
			height,
			format,
		}
	}

	pub fn with_layers_and_styles(
		mut self,
		layers_and_styles: impl IntoIterator<Item = (String, String)>,
	) -> Self {
		let (layers, styles) = layers_and_styles.into_iter().unzip();
		self.layers = layers;
		self.styles = styles;

		self
	}

	pub fn build(self, version: WmsVersion) -> GetMapRequest {
		let Bbox {
			min_lat,
			min_lon,
			max_lat,
			max_lon,
		} = self.bbox;

		let (rs, bbox) = match version {
			WmsVersion::V1_0_0 | WmsVersion::V1_1_0 | WmsVersion::V1_1_1 => (
				SrsOrCrs::Srs(self.crs),
				(min_lon, min_lat, max_lon, max_lat),
			),
			WmsVersion::V1_3_0 => (
				SrsOrCrs::Crs(self.crs),
				(min_lat, min_lon, max_lat, max_lon),
			),
		};

		GetMapRequest {
			version,
			layers: self.layers,
			styles: self.styles,
			rs,
			bbox,
			width: self.width,
			height: self.height,
			format: self.format,
		}
	}
}

pub struct GetMapRequest {
	pub(super) version: WmsVersion,
	pub(super) layers: Vec<String>,
	pub(super) styles: Vec<String>,
	pub(super) rs: SrsOrCrs,
	pub(super) bbox: (String, String, String, String),
	pub(super) width: u32,
	pub(super) height: u32,
	pub(super) format: String,
}

impl GetMapRequest {
	pub async fn send(self, client: &Client, url: &Url) -> OgcResult<Map> {
		let response = self.get(client, url).await?;

		let bytes = response.bytes().await?.to_vec();

		Ok(Map { bytes })
	}
}

impl OgcRequest for GetMapRequest {
	fn parameters(&self) -> Vec<(&'static str, String)> {
		let layers = self.layers.join(",");
		let styles = self.styles.join(",");
		let bbox = format!(
			"{},{},{},{}",
			self.bbox.0, self.bbox.1, self.bbox.2, self.bbox.3
		);

		vec![
			("SERVICE", SERVICE.to_string()),
			self.version.as_request_parameter(),
			("REQUEST", "GetMap".to_string()),
			("LAYERS", layers),
			("STYLES", styles),
			("BBOX", bbox),
			self.rs.as_request_parameter(),
			("WIDTH", self.width.to_string()),
			("HEIGHT", self.height.to_string()),
			("FORMAT", self.format.clone()),
		]
	}
}

pub struct Map {
	pub bytes: Vec<u8>,
}
