use crate::api;
use crate::api::{ApiResponseError, MarketType, MARKET_TYPE_QUERY_KEY};
use std::collections::HashMap;

const PATH : &'static str = "/v1/markets";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetMarketResponse {
    Error { errors: Vec<String> },
    Response(Vec<MarketInfo>)
}

#[derive(Deserialize, Debug)]
pub struct MarketInfo {
    pub product_code: String,
    pub market_type: String,
    pub alias: Option<String>,
}

pub async fn get_markets(market_type: MarketType) -> Result<Vec<MarketInfo>, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(MARKET_TYPE_QUERY_KEY.to_string(), market_type.to_string());
    let response = api::get_with_params::<GetMarketResponse>(&PATH, &params).await?;
    
    match response {
        GetMarketResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetMarketResponse::Response(vec) => Ok(vec),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_markets::get_markets;
    use crate::api::MarketType;

    #[tokio::test]
    async fn get_markets_test() {
        test_api!(get_markets(MarketType::Spot));
    }
}
