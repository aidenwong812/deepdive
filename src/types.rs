/// address of the birdeye public api server, and base of the request path
pub const BIRDEYE_PUBLIC_API_BASE: &str = "https://public-api.birdeye.so";

#[derive(Clone, Copy, Debug)]
pub enum RequestType {
    TokenOverview,
    WalletTokenList,
    HistoricalPrice,
    Price,
}
//     --url 'https://public-api.birdeye.so/public/price?address=So11111111111111111111111111111111111111112' \

pub enum RequestTypeOpts {
    TokenOverview {
        token: String,
    },
    WalletTokenList {
        wallet: String,
    },
    HistoricalPrice {
        token: String,
        time_from: u64,
        time_to: u64,
    },
    Price {
        token: String,
    },
}
impl RequestType {
    pub fn name(self) -> String {
        match self {
            Self::TokenOverview => "token_overview".to_string(),
            Self::WalletTokenList => "wallet/token_list".to_string(),
            Self::HistoricalPrice => "history_price".to_string(),
            Self::Price => "price".to_string(),
        }
    }
    pub fn to_string(&self, opts: RequestTypeOpts) -> Option<String> {
        match self {
            RequestType::TokenOverview => {
                if let RequestTypeOpts::TokenOverview { token } = opts {
                    Some(format!(
                        "{BIRDEYE_PUBLIC_API_BASE}/defi/token_overview?address={token}"
                    ))
                } else {
                    None
                }
            }
            RequestType::WalletTokenList => {
                if let RequestTypeOpts::WalletTokenList { wallet } = opts {
                    Some(format!(
                        "{BIRDEYE_PUBLIC_API_BASE}/v1/wallet/token_list?wallet={wallet}"
                    ))
                } else {
                    None
                }
            }
            RequestType::HistoricalPrice => {
                if let RequestTypeOpts::HistoricalPrice {
                    token,
                    time_from,
                    time_to,
                } = opts
                {
                    Some(format!("{BIRDEYE_PUBLIC_API_BASE}/public/history_price?address={token}&address_type=token&time_from={time_from}&time_to={time_to}"))
                } else {
                    None
                }
            }
            RequestType::Price => {
                if let RequestTypeOpts::Price { token } = opts {
                    Some(format!(
                        "{BIRDEYE_PUBLIC_API_BASE}/public/price?address={token}"
                    ))
                } else {
                    None
                }
            }
        }
    }
}

impl ToString for RequestType {
    fn to_string(&self) -> String {
        match self {
            RequestType::TokenOverview => {
                BIRDEYE_PUBLIC_API_BASE.to_owned() + "/defi/token_overview"
            }
            RequestType::WalletTokenList => {
                format!("{BIRDEYE_PUBLIC_API_BASE}/v1/wallet/token_list?wallet=")
            }
            RequestType::HistoricalPrice => {
                format!("{BIRDEYE_PUBLIC_API_BASE}/public/history_price?address=")
            }
            RequestType::Price => {
                format!("{BIRDEYE_PUBLIC_API_BASE}/public/price?address=")
            }
        }
    }
}
/*
curl --request GET \
     --url 'https://public-api.birdeye.so/v1/wallet/token_list?wallet=9cZyzZqPwqmXVN3W9jQizEPfCAEQr6GRQURuBrorym7g' \
     --header 'X-API-KEY: db0d6bb5b0664b9683e964fd1c805902' \
     --header 'x-chain: solana'
*/

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletPortfolioOverview {
    pub success: bool,
    pub data: WalletPortfolioOverviewData,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletPortfolioOverviewData {
    pub wallet: String,
    pub total_usd: f64,
    pub items: Vec<WalletPortfolioOverviewItem>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletPortfolioOverviewItem {
    pub address: String,
    pub decimals: i64,
    pub balance: i64,
    pub ui_amount: f64,
    pub chain_id: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    #[serde(rename = "logoURI")]
    pub logo_uri: Option<String>,
    pub price_usd: Option<f64>,
    pub value_usd: Option<f64>,
    pub icon: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalPriceOverview {
    pub data: HistoricalPriceOverviewData,
    pub success: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalPriceOverviewData {
    pub items: Vec<HistoricalPriceOverviewItem>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalPriceOverviewItem {
    pub unix_time: i64,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceOverview {
    pub data: PriceOverviewData,
    pub success: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceOverviewData {
    pub value: f64,
    pub update_unix_time: i64,
    pub update_human_time: String,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_request_type_url() {
        assert_eq!(
            RequestType::TokenOverview
                .to_string(RequestTypeOpts::TokenOverview{token: "So11111111111111111111111111111111111111112".to_string()})
                .unwrap(),
            "https://public-api.birdeye.so/defi/token_overview?address=So11111111111111111111111111111111111111112"
        )
    }
}
