use derive_more::From;

pub mod feature_info;
pub mod map;

#[derive(Clone, Copy)]
pub enum WmsVersion {
	V1_0_0,
	V1_1_0,
	V1_1_1,
	V1_3_0,
}

impl WmsVersion {
	fn as_request_parameter(&self) -> (&'static str, String) {
		match self {
			WmsVersion::V1_0_0 => ("VERSION", "1.0.0".to_string()),
			WmsVersion::V1_1_0 => ("VERSION", "1.1.0".to_string()),
			WmsVersion::V1_1_1 => ("VERSION", "1.1.1".to_string()),
			WmsVersion::V1_3_0 => ("VERSION", "1.3.0".to_string()),
		}
	}
}

#[derive(Clone, From)]
pub struct Bbox {
	pub min_lat: String,
	pub min_lon: String,
	pub max_lat: String,
	pub max_lon: String,
}

#[derive(Clone)]
enum SrsOrCrs {
	Srs(String),
	Crs(String),
}

impl SrsOrCrs {
	fn as_request_parameter(&self) -> (&'static str, String) {
		match self {
			SrsOrCrs::Crs(crs) => ("CRS", crs.clone()),
			SrsOrCrs::Srs(srs) => ("SRS", srs.clone()),
		}
	}
}

const SERVICE: &str = "WMS";
