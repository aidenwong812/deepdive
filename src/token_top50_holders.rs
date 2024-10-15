use serde::{Deserialize, Serialize};

// #[derive(Default, Debug, Clone, PartialEq,  Serialize, Deserialize)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TokenTopHolders {
    pub content: Vec<HolderInfo>,
    #[serde(rename = "totalElements")]
    pub holders_count: i32,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// // #[derive(Default, Debug, Clone, Serialize, Deserialize)]
// pub struct TokenContents {
//     pub holders: Vec<HolderInfo>,

// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct HolderInfo {
    #[serde(rename = "holderAddress")]
    pub holder_address: String,
    #[serde(rename = "coinType")]
    pub coin_type: String,
    #[serde(rename = "coinDenom")]
    pub coin_denom: String,
    pub amount: f64,
    #[serde(rename = "usdAmount")]
    pub usd_amount: Option<f64>,
    pub percentage: f64,
    #[serde(rename = "objectsCount")]
    pub objects_count: i32,
}
