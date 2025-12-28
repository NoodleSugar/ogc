use ogc::wmts::WmtsClient;

const GEOPF_URL: &str = "https://data.geopf.fr/wmts";

#[tokio::test]
async fn get_capabilities_request_should_succeed() {
	let url = GEOPF_URL.parse().unwrap();
	let client = WmtsClient::new(url);

	let result = client.get_capabilities().await;

	assert!(result.is_ok());
}
