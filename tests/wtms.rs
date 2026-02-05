use ogc::{OgcClient, wmts::capabilities::GetCapabilitiesRequest};

const GEOPF_URL: &str = "https://data.geopf.fr/wmts";

#[tokio::test]
async fn get_capabilities_request_should_succeed() {
	let url = GEOPF_URL.parse().unwrap();
	let client = OgcClient::new(url);

	let result = GetCapabilitiesRequest.send(&client).await;

	assert!(result.is_ok());
}
