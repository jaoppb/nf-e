use crate::models::ICMSSN102;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Model {
    NFe = 55,
    NFCe = 65,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Operation {
    Incoming = 0,
    Outgoing = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DestinationTarget {
    Internal = 1,
    Interstate = 2,
    External = 3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DanfeGeneration {
    NormalPortrait = 1,
    NormalLandscape = 2,
    Simplified = 3,
    NFCe = 4,
    NFCeVirtual = 5,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EmissionType {
    Normal = 1,
    FSIA = 2,
    EPEC = 4,
    FSDA = 5,
    SVCAN = 6,
    SVCRS = 7,
    Offline = 9,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Environment {
    Production = 1,
    Homologation = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Finality {
    Normal = 1,
    Complementary = 2,
    Adjustment = 3,
    Cancellation = 4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Presence {
    InplaceIndoor = 1,
    InplaceOutdoor = 5,
    Internet = 2,
    Teleservice = 3,
    Delivery = 4,
    Other = 9,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Intermediator {
    External = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Document {
    CNPJ(CNPJ),
    CPF(CPF),
    IE(IE),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CNPJ(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CPF(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IE(pub String);

#[derive(Debug, PartialEq)]
pub enum ICMS {
    ICMSSN102(ICMSSN102),
}

impl Serialize for ICMS {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ICMS::ICMSSN102(data) => {
                let mut state = serializer.serialize_struct("ICMS", 1)?;
                state.serialize_field("ICMSSN102", data)?;
                state.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for ICMS {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ICMSHelper {
            #[serde(rename = "ICMSSN102")]
            icmssn102: Option<ICMSSN102>,
        }

        let helper = ICMSHelper::deserialize(deserializer)?;
        if let Some(data) = helper.icmssn102 {
            Ok(ICMS::ICMSSN102(data))
        } else {
            Err(serde::de::Error::custom("Unknown ICMS variant"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[repr(u8)]
#[serde(from = "u8", into = "u8")]
pub enum CSOSN {
    FinalConsumer = 102,
}

impl From<u8> for CSOSN {
    fn from(value: u8) -> Self {
        match value {
            102 => CSOSN::FinalConsumer,
            _ => panic!("Invalid CSOSN value: {}", value),
        }
    }
}

impl From<CSOSN> for u8 {
    fn from(value: CSOSN) -> Self {
        value as u8
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[repr(u8)]
#[serde(from = "u8", into = "u8")]
#[derive(PartialEq)]
pub enum Origin {
    National = 0,
    NationalInConformity = 4,
    NationalContentBelow40 = 5,
    NationalContentBetween40And70 = 3,
    NationalContentAbove70 = 8,
    Foreign = 1,
    ForeignInternalMarket = 2,
    ForeignNoSimilar = 6,
    ForeignInternalMarketNoSimilar = 7,
}

impl From<u8> for Origin {
    fn from(value: u8) -> Self {
        match value {
            0 => Origin::National,
            1 => Origin::Foreign,
            2 => Origin::ForeignInternalMarket,
            3 => Origin::NationalContentBetween40And70,
            4 => Origin::NationalInConformity,
            5 => Origin::NationalContentBelow40,
            6 => Origin::ForeignNoSimilar,
            7 => Origin::ForeignInternalMarketNoSimilar,
            8 => Origin::NationalContentAbove70,
            _ => panic!("Invalid origin value: {}", value),
        }
    }
}

impl From<Origin> for u8 {
    fn from(value: Origin) -> Self {
        value as u8
    }
}
