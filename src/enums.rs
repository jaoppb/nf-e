use crate::models::ICMSSN102;
use crate::utils::left_pad;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum TransportType {
    CIF = 0,
    FOB = 1,
    ThirdParty = 2,
    Issuer = 3,
    Recipient = 4,
    None = 9,
}

impl Default for TransportType {
    fn default() -> Self {
        TransportType::None
    }
}

impl TryFrom<u8> for TransportType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TransportType::CIF),
            1 => Ok(TransportType::FOB),
            2 => Ok(TransportType::ThirdParty),
            3 => Ok(TransportType::Issuer),
            4 => Ok(TransportType::Recipient),
            9 => Ok(TransportType::None),
            _ => Err(format!("Invalid transport type value: {}", value)),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Model {
    NFe = 55,
    NFCe = 65,
}

impl Model {
    pub fn code(&self) -> u8 {
        self.clone() as u8
    }
}

impl TryFrom<u8> for Model {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            55 => Ok(Model::NFe),
            65 => Ok(Model::NFCe),
            _ => Err(format!("Invalid model value: {}", value)),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Operation {
    Incoming = 0,
    Outgoing = 1,
}

impl TryFrom<u8> for Operation {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Operation::Incoming),
            1 => Ok(Operation::Outgoing),
            _ => Err(format!("Invalid operation value: {}", value)),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum DestinationTarget {
    Internal = 1,
    Interstate = 2,
    External = 3,
}

impl TryFrom<u8> for DestinationTarget {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DestinationTarget::Internal),
            2 => Ok(DestinationTarget::Interstate),
            3 => Ok(DestinationTarget::External),
            _ => Err(format!("Invalid destination target value: {}", value)),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum DanfeGeneration {
    NormalPortrait = 1,
    NormalLandscape = 2,
    Simplified = 3,
    NFCe = 4,
    NFCeVirtual = 5,
}

impl TryFrom<u8> for DanfeGeneration {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DanfeGeneration::NormalPortrait),
            2 => Ok(DanfeGeneration::NormalLandscape),
            3 => Ok(DanfeGeneration::Simplified),
            4 => Ok(DanfeGeneration::NFCe),
            5 => Ok(DanfeGeneration::NFCeVirtual),
            _ => Err(format!("Invalid DANFE generation value: {}", value)),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum EmissionType {
    Normal = 1,
    FSIA = 2,
    EPEC = 4,
    FSDA = 5,
    SVCAN = 6,
    SVCRS = 7,
    Offline = 9,
}

impl EmissionType {
    pub fn code(&self) -> u8 {
        self.clone() as u8
    }
}

impl TryFrom<u8> for EmissionType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(EmissionType::Normal),
            2 => Ok(EmissionType::FSIA),
            4 => Ok(EmissionType::EPEC),
            5 => Ok(EmissionType::FSDA),
            6 => Ok(EmissionType::SVCAN),
            7 => Ok(EmissionType::SVCRS),
            9 => Ok(EmissionType::Offline),
            _ => Err(format!("Invalid emission type value: {}", value)),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Environment {
    Production = 1,
    Homologation = 2,
}

impl TryFrom<u8> for Environment {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Environment::Production),
            2 => Ok(Environment::Homologation),
            _ => Err(format!("Invalid environment value: {}", value)),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Finality {
    Normal = 1,
    Complementary = 2,
    Adjustment = 3,
    Cancellation = 4,
}

impl TryFrom<u8> for Finality {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Finality::Normal),
            2 => Ok(Finality::Complementary),
            3 => Ok(Finality::Adjustment),
            4 => Ok(Finality::Cancellation),
            _ => Err(format!("Invalid finality value: {}", value)),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Presence {
    InplaceIndoor = 1,
    InplaceOutdoor = 5,
    Internet = 2,
    Teleservice = 3,
    Delivery = 4,
    Other = 9,
}

impl TryFrom<u8> for Presence {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Presence::InplaceIndoor),
            2 => Ok(Presence::Internet),
            3 => Ok(Presence::Teleservice),
            4 => Ok(Presence::Delivery),
            5 => Ok(Presence::InplaceOutdoor),
            9 => Ok(Presence::Other),
            _ => Err(format!("Invalid presence value: {}", value)),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Intermediator {
    External = 1,
}

impl TryFrom<u8> for Intermediator {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Intermediator::External),
            _ => Err(format!("Invalid intermediator value: {}", value)),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Document {
    CNPJ(CNPJ),
    CPF(CPF),
    IE(IE),
}

impl Document {
    pub fn as_str(&self) -> &str {
        match self {
            Document::CNPJ(cnpj) => &cnpj.0,
            Document::CPF(cpf) => &cpf.0,
            Document::IE(ie) => &ie.0,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum PersonDocument {
    CNPJ(CNPJ),
    CPF(CPF),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CNPJ(pub String);

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CPF(pub String);

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(u8)]
#[serde(from = "u8", into = "u8")]
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

#[derive(PartialEq, Clone, Debug)]
pub enum PaymentType {
    Cash = 1,
    Check = 2,
    CreditCard = 3,
    DebitCard = 4,
    ShopCredit = 5,
    FoodVoucher = 6,
    MealVoucher = 7,
    GiftCard = 8,
    GasVoucher = 9,
    Boleto = 15,
    BankDeposit = 16,
    PIX = 17,
    Transfer = 18,
    Program = 19,
}

impl Serialize for PaymentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        left_pad(&self.code().to_string(), 2, '0').serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PaymentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let value = s.parse::<u8>().map_err(serde::de::Error::custom)?;
        PaymentType::try_from(value).map_err(serde::de::Error::custom)
    }
}

impl TryFrom<u8> for PaymentType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PaymentType::Cash),
            2 => Ok(PaymentType::Check),
            3 => Ok(PaymentType::CreditCard),
            4 => Ok(PaymentType::DebitCard),
            5 => Ok(PaymentType::ShopCredit),
            6 => Ok(PaymentType::FoodVoucher),
            7 => Ok(PaymentType::MealVoucher),
            8 => Ok(PaymentType::GiftCard),
            9 => Ok(PaymentType::GasVoucher),
            15 => Ok(PaymentType::Boleto),
            16 => Ok(PaymentType::BankDeposit),
            17 => Ok(PaymentType::PIX),
            18 => Ok(PaymentType::Transfer),
            19 => Ok(PaymentType::Program),
            _ => Err(format!("Invalid payment type value: {}", value)),
        }
    }
}

impl PaymentType {
    pub fn code(&self) -> u8 {
        self.clone() as u8
    }
}

#[cfg(test)]
mod test {
    use crate::utils::canonicalize_xml as canonicalize;
    use nf_e_macros::serialization_test;
    use quick_xml::{de::from_str as deserialize, se::to_string as serialize};

    use super::*;

    #[serialization_test(expected = "<CNPJ>12345678000195</CNPJ>")]
    fn setup_cnpj() -> CNPJ {
        CNPJ("12345678000195".to_string())
    }

    #[serialization_test(expected = "<CPF>12345678901</CPF>")]
    fn setup_cpf() -> CPF {
        CPF("12345678901".to_string())
    }

    #[serialization_test(expected = "<IE>123456789</IE>")]
    fn setup_ie() -> IE {
        IE("123456789".to_string())
    }

    #[serialization_test(fixture = "../tests/fixtures/enums/icms.xml")]
    fn setup_icms() -> ICMS {
        ICMS::ICMSSN102(ICMSSN102 {
            csosn: CSOSN::FinalConsumer,
            origin: Origin::National,
        })
    }
}
