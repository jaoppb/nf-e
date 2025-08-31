use crate::enums::*;

use crate::states::{City, Location, State};
use crate::LIBRARY_VERSION;
use serde::ser::SerializeSeq;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Info {
    pub id: String,
    pub identification: Identification,
    pub issuer: Issuer,
    pub details: Details,
}

impl Serialize for Info {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Info", 5)?;
        state.serialize_field("@versao", &self.version())?;
        state.serialize_field("@id", &self.id)?;
        state.serialize_field("ide", &self.identification)?;
        state.serialize_field("emit", &self.issuer)?;
        state.serialize_field("details", &self.details)?;
        state.end()
    }
}

impl Info {
    pub fn version(&self) -> String {
        "4.00".to_string()
    }
}

#[derive(Deserialize, Debug)]
pub struct Identification {
    pub location: Location,
    pub numeric_code: u32,
    pub operation_nature: String,
    pub model: Model,
    pub series: u8,
    pub number: u32,
    pub emission_date: chrono::DateTime<chrono::Local>,
    pub date: Option<chrono::DateTime<chrono::Local>>,
    pub r#type: Operation,
    pub destination: DestinationTarget,
    pub printing_type: Option<DanfeGeneration>,
    pub emission_type: EmissionType,
    pub verifier_digit: u8,
    pub environment: Environment,
    pub finality: Finality,
    pub consumer: bool,
    pub presence: Option<Presence>,
    pub intermediator: Option<Intermediator>,
}

impl Identification {
    fn emission_process(&self) -> u8 {
        0
    }

    fn emission_version(&self) -> &str {
        LIBRARY_VERSION
    }
}

impl Serialize for Identification {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let len = 17
            + self.date.is_some() as usize
            + self.printing_type.is_some() as usize
            + self.intermediator.is_some() as usize;

        let mut state = serializer.serialize_struct("ide", len)?;
        state.serialize_field("cUF", &(self.location.state.clone() as u8))?;
        state.serialize_field("cNF", &self.numeric_code)?;
        state.serialize_field("natOp", &self.operation_nature)?;
        state.serialize_field("mod", &(self.model.clone() as u8))?;
        state.serialize_field("serie", &self.series)?;
        state.serialize_field("nNF", &self.number)?;
        state.serialize_field("dhEmi", &self.emission_date.to_utc())?;
        if let Some(date) = &self.date {
            state.serialize_field("dhSaiEnt", &date.to_utc())?;
        }
        state.serialize_field("tpNF", &(self.r#type.clone() as u8))?;
        state.serialize_field("idDest", &(self.destination.clone() as u8))?;
        if let Some(printing_type) = &self.printing_type {
            state.serialize_field("tpImp", &(printing_type.clone() as u8))?;
        }
        state.serialize_field("tpEmis", &(self.emission_type.clone() as u8))?;
        state.serialize_field("cDV", &self.verifier_digit)?;
        state.serialize_field("tpAmb", &(self.environment.clone() as u8))?;
        state.serialize_field("finNFe", &(self.finality.clone() as u8))?;
        state.serialize_field("indFinal", if self.consumer { &1 } else { &0 })?;
        state.serialize_field(
            "indPres",
            &(self.presence.as_ref().map_or(0, |p| (*p).clone() as u8)),
        )?;
        if let Some(intermediator) = &self.intermediator {
            state.serialize_field("intermed", intermediator)?;
        }
        state.serialize_field("procEmi", &self.emission_process())?;
        state.serialize_field("verProc", &self.emission_version())?;
        state.end()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub line_1: String,
    pub line_2: Option<String>,
    pub number: String,
    pub neighborhood: String,
    pub city: City,
    pub state: State,
    pub zip_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaxableAddress {
    pub address: Address,
    pub ie: IE,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Issuer {
    pub document: Document,
    pub name: String,
    pub trade_name: Option<String>,
    pub address: TaxableAddress,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub code: String,
    pub gtin: Option<String>,
    pub description: String,
    pub ncm: u32,
    pub cest: u32,
    pub tribute: u16,
    pub cfop: u32,
    pub unit: String,
    pub quantity: f64,
    pub total_value: f64,
    pub tribute_unit: String,
    pub tribute_quantity: f64,
    pub tribute_unit_value: f64,
    pub freight_value: Option<f64>,
    pub insurance_value: Option<f64>,
    pub discount_value: Option<f64>,
    pub other_value: Option<f64>,
    pub included: bool,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub enum CSOSN {
    FinalConsumer = 102,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ICMSSN102 {
    pub origin: Origin,
    pub csosn: CSOSN,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ICMS {
    ICMSSN102(ICMSSN102),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Detail {
    pub item: Item,
    pub icms: ICMS,
}

#[derive(Deserialize, Debug)]
pub struct Details {
    pub details: Vec<Detail>,
}

impl Serialize for Details {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct DetailWrapper<'a> {
            #[serde(flatten)]
            detail: &'a Detail,
            #[serde(rename = "@nItem")]
            n_item: usize,
        }

        let mut sequence = serializer.serialize_seq(Some(self.details.len()))?;
        for (index, detail) in self.details.iter().enumerate() {
            let element = DetailWrapper {
                detail,
                n_item: index + 1,
            };

            sequence.serialize_element(&element)?;
        }
        sequence.end()
    }
}
