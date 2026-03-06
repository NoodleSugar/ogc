use ogc::wmts::capabilities::GetCapabilitiesRequest;
use reqwest::Client;

const GEOPF_URL: &str = "https://data.geopf.fr/wmts";

#[tokio::test]
async fn get_capabilities_request_should_succeed() {
	let url = GEOPF_URL.parse().unwrap();
	let client = Client::new();

	let result = GetCapabilitiesRequest.send(&client, &url).await;

	assert!(result.is_ok());
}
