/// https://schemas.opengis.net/wmts/1.0/wmtsGetTile_request.xsd
#[derive(Debug, Clone)]
pub struct GetTileRequest {
	pub layer: String,
	pub style: String,
	pub format: String,
	pub tile_matrix_set: String,
	pub tile_matrix: String,
	pub tile_row: u64,
	pub tile_col: u64,
}

#[derive(Debug, Clone)]
pub struct Tile {
	pub bytes: Vec<u8>,
}

impl GetTileRequest {
	pub fn parameters(&self) -> Vec<(&str, String)> {
		vec![
			("service", super::SERVICE.to_string()),
			("version", "1.0.0".to_string()),
			("request", "GetTile".to_string()),
			("layer", self.layer.to_string()),
			("style", self.style.to_string()),
			("format", self.format.to_string()),
			("tilematrixset", self.tile_matrix_set.to_string()),
			("tilematrix", self.tile_matrix.to_string()),
			("tilerow", self.tile_row.to_string()),
			("tilecol", self.tile_col.to_string()),
		]
	}
}
