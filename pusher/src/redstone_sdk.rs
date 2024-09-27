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
    data_feed_id: FeedId,
    value: f64,
    decimals: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SignedDataPackage {
    data_points: Vec<DataPoint>,
    timestamp_milliseconds: u64,
    signature: String,
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
            .flatten()
            .cloned()
            .collect::<Vec<_>>();

        let payload = RedstonePayload::new(signed_data_packages);
        Ok(payload.to_bytes())
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
            let query_params =
                format!("dataFeedIds={}", params.data_packages_ids.join(","));
            let request_url = format!("{}?{}", fetch_url, query_params);

            let mut response =
                Fetch::Url(request_url.parse()?).send().await?;

            if response.status_code() == 200 {
                return response.json().await;
            }
        }

        Err(Error::from(
            "Failed to fetch data packages from all gateways",
        ))
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
        // Implement the logic to convert the payload to bytes
        // This is a simplified version and may need to be adjusted based on the exact protocol
        let mut bytes = Vec::new();
        for package in &self.signed_data_packages {
            bytes.extend_from_slice(
                &package.timestamp_milliseconds.to_be_bytes(),
            );
            for point in &package.data_points {
                bytes.extend_from_slice(point.data_feed_id.as_bytes());
                bytes.extend_from_slice(&(point.value as u64).to_be_bytes());
            }
            bytes
                .extend_from_slice(&hex::decode(&package.signature).unwrap());
        }
        bytes
    }
}
