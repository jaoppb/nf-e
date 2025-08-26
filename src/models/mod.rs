use crate::enums::*;

use crate::states::Location;
use crate::LIBRARY_VERSION;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Info {
    pub id: String,
    pub identification: Identification,
}

impl Serialize for Info {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Info", 3)?;
        state.serialize_field("versao", &self.version())?;
        state.serialize_field("id", &self.id)?;
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
    location: Location,
    numeric_code: u32,
    operation_nature: String,
    model: Model,
    series: u8,
    number: u32,
    emission_date: chrono::DateTime<chrono::Local>,
    date: Option<chrono::DateTime<chrono::Local>>,
    r#type: Operation,
    destination: DestinationTarget,
    printing_type: Option<DanfeGeneration>,
    emission_type: EmissionType,
    verifier_digit: u8,
    environment: Environment,
    finality: Finality,
    consumer: bool,
    presence: Option<Presence>,
    intermediator: Option<Intermediator>,
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
        let mut state = serializer.serialize_struct("ide", 16)?;
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
