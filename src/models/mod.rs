use crate::enums::*;

use crate::states::{City, Location, State};
use crate::LIBRARY_VERSION;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Info {
    pub id: String,
    pub identification: Identification,
    pub issuer: Issuer,
}

impl Serialize for Info {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Info", 3)?;
        state.serialize_field("@versao", &self.version())?;
        state.serialize_field("@id", &self.id)?;
        state.serialize_field("ide", &self.identification)?;
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
