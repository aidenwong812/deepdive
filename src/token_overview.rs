use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenOverview {
    pub data: TokenOverviewData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenOverviewData {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub liquidity: f64,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
    #[serde(rename = "buy1hChangePercent")]
    pub buy_1h_change_percent: Option<f64>,
    #[serde(rename = "buy2hChangePercent")]
    pub buy_2h_change_percent: Option<f64>,
    #[serde(rename = "buy4hChangePercent")]
    pub buy_4h_change_percent: Option<f64>,
    #[serde(rename = "buy6hChangePercent")]
    pub buy_6h_change_percent: Option<f64>,
    #[serde(rename = "buy8hChangePercent")]
    pub buy_8h_change_percent: Option<f64>,
    #[serde(rename = "buy12hChangePercent")]
    pub buy_12h_change_percent: Option<f64>,
    #[serde(rename = "buy24hChangePercent")]
    pub buy_24h_change_percent: Option<f64>,
    #[serde(rename = "priceChange30mPercent")]
    pub price_change_30m_percent: Option<f64>,
    #[serde(rename = "priceChange1hPercent")]
    pub price_change_1h_percent: Option<f64>,
    #[serde(rename = "priceChange2hPercent")]
    pub price_change_2h_percent: Option<f64>,
    #[serde(rename = "priceChange4hPercent")]
    pub price_change_4h_percent: Option<f64>,
    #[serde(rename = "priceChange6hPercent")]
    pub price_change_6h_percent: Option<f64>,
    #[serde(rename = "priceChange8hPercent")]
    pub price_change_8h_percent: Option<f64>,
    #[serde(rename = "priceChange12hPercent")]
    pub price_change_12h_percent: Option<f64>,
    #[serde(rename = "priceChange24hPercent")]
    pub price_change_24h_percent: Option<f64>,
    #[serde(rename = "history30mPrice")]
    pub history_30m_price: f64,
    #[serde(rename = "history1hPrice")]
    pub history_1h_price: f64,
    #[serde(rename = "history24hPrice")]
    pub history_2h_price: f64,
    #[serde(rename = "history4hPrice")]
    pub history_4h_price: f64,
    #[serde(rename = "history6hPrice")]
    pub history_6h_price: f64,
    #[serde(rename = "history8hPrice")]
    pub history_8h_price: f64,
    #[serde(rename = "history12hPrice")]
    pub history_12h_price: f64,
    #[serde(rename = "history2hPrice")]
    pub history_24h_price: f64,
    #[serde(rename = "numberMarkets")]
    pub number_markets: i64,

   
}
