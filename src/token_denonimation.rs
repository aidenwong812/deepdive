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

//     "coinType": "0xe152cf4590affa8e187f98a4d31bba396e7922ae45c2c7c59b4575742dfde196::sui::SUI",
//     "objectId": "0xd08b502eee8da2c1ddcee6fdbfbff53f835a6933049d0ef45aa879658c2589a1",
//     "coinName": "CristianoRonaldoSpeedSmurf7Siu",
//     "coinDenom": "SUI",
//     "decimals": 6,
//     "coinSymbol": "SUI",
//     "imgUrl": "https://api.movepump.com/uploads/1000029604_bc62421431.webp",
//     "description": "Siuuuuuuuuu!",
//     "supply": 10000000000.0000000000,
//     "supplyInUsd": 0E-11,
//     "price": 0.0,
//     "dominance": null,
//     "circulatingSupply": null,
//     "marketCap": null,
//     "totalVolume": null,
//     "maxSupply": null,
//     "fdv": 0.0,
//     "holdersCount": 800,
//     "packageId": "0xe152cf4590affa8e187f98a4d31bba396e7922ae45c2c7c59b4575742dfde196",
//     "creatorAddress": "0x07051cff3b47c7a1b01c1c25009076e08768093ca0cd7bb00ff7faf74bb4945e",
//     "creatorName": null,
//     "creatorImg": null,
//     "creatorSecurityMessage": null,
//     "createTimestamp": 1728284072062,
//     "isVerified": false,
//     "isBridged": false,
//     "securityMessage": null