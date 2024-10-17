use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, )]
pub struct TokenDenonimation {
    #[serde(rename = "createTimestamp")]
    pub create_time_stamp: Option<i64>,
    pub fdv: Option<f64>,
    pub supply: Option<f64>,
    #[serde(rename = "circulatingSupply")]
    pub circulating_supply: Option<f64>,
    
}