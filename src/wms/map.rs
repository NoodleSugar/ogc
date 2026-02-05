use crate::{OgcClient, OgcResult};

pub struct GetMapRequest {
	layers: Vec<String>,
	styles: Vec<String>,
	crs: String,
	bbox: (f64, f64, f64, f64),
	width: u64,
	height: u64,
	format: String,
}

impl GetMapRequest {
	pub fn new(
		layer: String,
		style: String,
		crs: String,
		bbox: (f64, f64, f64, f64),
		width: u64,
		height: u64,
		format: String,
	) -> Self {
		Self {
			layers: vec![layer],
			styles: vec![style],
			crs,
			bbox,
			width,
			height,
			format,
		}
	}

	pub fn add_layer(mut self, layer: String, style: String) -> Self {
		self.layers.push(layer);
		self.styles.push(style);
		self
	}

	pub async fn send(self, client: &OgcClient) -> OgcResult<Map> {
		let response = client.get(&self.parameters()).await?;

		let bytes = response.bytes().await?.to_vec();

		Ok(Map { bytes })
	}

	fn parameters(&self) -> Vec<(&str, String)> {
		let layers = self.layers.join(",");
		let styles = self.styles.join(",");
		let (minx, miny, maxx, maxy) = self.bbox;
		let bbox = format!("{minx},{miny},{maxx},{maxy}");
		vec![
			("service", super::SERVICE.to_string()),
			("version", "1.3.0".to_string()),
			("request", "GetMap".to_string()),
			("layers", layers),
			("styles", styles),
			("crs", self.crs.clone()),
			("bbox", bbox),
			("width", self.width.to_string()),
			("height", self.height.to_string()),
			("format", self.format.clone()),
		]
	}
}

pub struct Map {
	pub bytes: Vec<u8>,
}
