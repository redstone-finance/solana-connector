use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use worker::*;

#[derive(Default)]
pub struct RedstoneClient {}

pub type FeedId = String;

#[derive(Debug, Serialize, Deserialize)]
struct DataPackagesRequestParams {
    data_service_id: String,
    data_packages_ids: Vec<FeedId>,
    unique_signers_count: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DataPoint {
    #[serde(rename = "dataFeedId")]
    data_feed_id: FeedId,
    value: f64,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SignedDataPackage {
    #[serde(rename = "timestampMilliseconds")]
    timestamp_milliseconds: u64,
    #[serde(rename = "dataPoints")]
    data_points: Vec<DataPoint>,
    #[serde(rename = "signerAddress")]
    signer_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GatewayResponse {
    #[serde(flatten)]
    data_packages: HashMap<FeedId, Vec<SignedDataPackage>>,
}

impl RedstoneClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn request_redstone_payload(
        &self,
        feed_ids: &[FeedId],
        data_service_id: String,
        unique_signers_count: usize,
    ) -> Result<Vec<u8>> {
        let params = DataPackagesRequestParams {
            data_service_id: data_service_id.clone(),
            data_packages_ids: feed_ids.to_vec(),
            unique_signers_count,
        };

        let response = self.request_data_packages(&params).await?;
        let signed_data_packages = response
            .data_packages
            .values()
            .flat_map(|v| v.iter().cloned())
            .collect::<Vec<_>>();

        let payload = RedstonePayload::new(signed_data_packages);
        let res = payload.to_bytes();
        console_log!("Payload: {} bytes", res.len());
        Ok(res)
    }

    async fn request_data_packages(
        &self,
        params: &DataPackagesRequestParams,
    ) -> Result<GatewayResponse> {
        let urls = self.resolve_data_service_urls();

        for url in urls {
            let fetch_url = format!(
                "{}/data-packages/latest/{}",
                url, params.data_service_id
            );
            let query_params = format!(
                "dataFeedIds={}&dataPackagesIds={}&minimalSignerCount={}",
                params.data_packages_ids.join(","),
                params.data_packages_ids.join(","),
                params.unique_signers_count
            );
            let request_url = format!("{}?{}", fetch_url, query_params);
            console_log!("Requesting data packages from: {}", request_url);

            let mut response =
                Fetch::Url(request_url.parse()?).send().await?;

            console_log!("Redstone gateway: {}", response.status_code());

            if response.status_code() == 200 {
                let response_text = response.text().await?;
                let res = serde_json::from_str(&response_text).map_err(|e| {
                    Error::from(format!("Failed to parse response: {:?}", e))
                });
                return res;
            }
        }

        Err(Error::from("Failed to fetch data packages from gateways"))
    }

    fn resolve_data_service_urls(&self) -> Vec<String> {
        vec![
            "https://oracle-gateway-1.a.redstone.finance".to_string(),
            "https://oracle-gateway-2.a.redstone.finance".to_string(),
        ]
    }
}

struct RedstonePayload {
    signed_data_packages: Vec<SignedDataPackage>,
}

impl RedstonePayload {
    fn new(signed_data_packages: Vec<SignedDataPackage>) -> Self {
        Self {
            signed_data_packages,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for package in &self.signed_data_packages {
            bytes.extend_from_slice(
                &package.timestamp_milliseconds.to_be_bytes(),
            );
            for point in &package.data_points {
                bytes.extend_from_slice(point.data_feed_id.as_bytes());
                bytes.extend_from_slice(&(point.value as u64).to_be_bytes());
            }

            bytes.extend_from_slice(package.signature.as_bytes());
        }
        bytes
    }
}
