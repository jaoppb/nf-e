use crate::enums::{CNPJ, Environment, Model};
use crate::models::NFe;
use crate::states::State;
use chrono::{DateTime, FixedOffset};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::services::namespaces;

/// NFe data wrapper for SOAP messages
#[derive(Debug, PartialEq)]
pub struct NfeDados<T> {
    pub content: T,
}

impl<T: Serialize> Serialize for NfeDados<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("nfeDadosMsg", 2)?;
        state.serialize_field("@xmlns", namespaces::NFE_DATA)?;
        state.serialize_field("$value", &self.content)?;
        state.end()
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for NfeDados<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct NfeDadosHelper<T> {
            #[serde(rename = "@xmlns")]
            xmlns: String,
            #[serde(rename = "$value")]
            content: T,
        }

        let helper = NfeDadosHelper::deserialize(deserializer)?;

        if helper.xmlns != namespaces::NFE_DATA {
            return Err(serde::de::Error::custom(format!(
                "Invalid xmlns: expected '{}', found '{}'",
                namespaces::NFE_DATA,
                helper.xmlns
            )));
        }

        Ok(NfeDados {
            content: helper.content,
        })
    }
}

/// Supported NFe version constant
const SUPPORTED_VERSION: &str = "4.00";

/// Request to send NFe batch for authorization
#[derive(Debug, PartialEq)]
pub struct EnviNFe {
    /// Batch identifier (up to 15 digits)
    pub id_lote: String,
    /// Synchronous processing indicator (0=No, 1=Yes)
    pub ind_sinc: u8,
    /// List of NFe documents (max 50)
    pub nfes: Vec<NFe>,
}

impl EnviNFe {
    /// Create a new synchronous NFe batch
    pub fn new_sync(id_lote: String, nfes: Vec<NFe>) -> Self {
        EnviNFe {
            id_lote,
            ind_sinc: 1,
            nfes,
        }
    }

    /// Create a new asynchronous NFe batch
    pub fn new_async(id_lote: String, nfes: Vec<NFe>) -> Self {
        EnviNFe {
            id_lote,
            ind_sinc: 0,
            nfes,
        }
    }
}

impl Serialize for EnviNFe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("enviNFe", 4)?;
        state.serialize_field("@xmlns", namespaces::NFE_DATA)?;
        state.serialize_field("@versao", SUPPORTED_VERSION)?;
        state.serialize_field("idLote", &self.id_lote)?;
        state.serialize_field("indSinc", &self.ind_sinc)?;
        state.serialize_field("NFe", &self.nfes)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for EnviNFe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct EnviNFeHelper {
            #[serde(rename = "@xmlns")]
            xmlns: String,
            #[serde(rename = "@versao")]
            versao: String,
            #[serde(rename = "idLote")]
            id_lote: String,
            #[serde(rename = "indSinc")]
            ind_sinc: u8,
            #[serde(rename = "NFe")]
            nfes: Vec<NFe>,
        }

        let helper = EnviNFeHelper::deserialize(deserializer)?;

        if helper.xmlns != namespaces::NFE_DATA {
            return Err(serde::de::Error::custom(format!(
                "Invalid xmlns: expected '{}', found '{}'",
                namespaces::NFE_DATA,
                helper.xmlns
            )));
        }

        if helper.versao != SUPPORTED_VERSION {
            return Err(serde::de::Error::custom(format!(
                "Unsupported version: expected '{}', found '{}'",
                SUPPORTED_VERSION, helper.versao
            )));
        }

        Ok(EnviNFe {
            id_lote: helper.id_lote,
            ind_sinc: helper.ind_sinc,
            nfes: helper.nfes,
        })
    }
}

/// Response from NFe authorization request
#[derive(Debug, PartialEq)]
pub struct RetEnviNFe {
    pub tp_amb: Environment,
    pub ver_aplic: String,
    pub c_stat: String,
    pub x_motivo: String,
    pub c_uf: State,
    pub dh_recbto: DateTime<FixedOffset>,
    pub inf_rec: Option<InfRec>,
    pub prot_nfe: Option<ProtNFe>,
}

impl Serialize for RetEnviNFe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("retEnviNFe", 9)?;
        state.serialize_field("@versao", SUPPORTED_VERSION)?;
        state.serialize_field("tpAmb", &(self.tp_amb.clone() as u8))?;
        state.serialize_field("verAplic", &self.ver_aplic)?;
        state.serialize_field("cStat", &self.c_stat)?;
        state.serialize_field("xMotivo", &self.x_motivo)?;
        state.serialize_field("cUF", &self.c_uf.code())?;
        state.serialize_field("dhRecbto", &self.dh_recbto.to_rfc3339())?;
        if let Some(ref inf_rec) = self.inf_rec {
            state.serialize_field("infRec", inf_rec)?;
        }
        if let Some(ref prot_nfe) = self.prot_nfe {
            state.serialize_field("protNFe", prot_nfe)?;
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for RetEnviNFe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RetEnviNFeHelper {
            #[serde(rename = "@versao")]
            versao: String,
            #[serde(rename = "tpAmb")]
            tp_amb: u8,
            #[serde(rename = "verAplic")]
            ver_aplic: String,
            #[serde(rename = "cStat")]
            c_stat: String,
            #[serde(rename = "xMotivo")]
            x_motivo: String,
            #[serde(rename = "cUF")]
            c_uf: u8,
            #[serde(rename = "dhRecbto")]
            dh_recbto: String,
            #[serde(rename = "infRec")]
            inf_rec: Option<InfRec>,
            #[serde(rename = "protNFe")]
            prot_nfe: Option<ProtNFe>,
        }

        let helper = RetEnviNFeHelper::deserialize(deserializer)?;

        if helper.versao != SUPPORTED_VERSION {
            return Err(serde::de::Error::custom(format!(
                "Unsupported version: expected '{}', found '{}'",
                SUPPORTED_VERSION, helper.versao
            )));
        }

        let tp_amb = Environment::try_from(helper.tp_amb).map_err(serde::de::Error::custom)?;
        let c_uf = State::try_from(helper.c_uf).map_err(serde::de::Error::custom)?;
        let dh_recbto =
            DateTime::parse_from_rfc3339(&helper.dh_recbto).map_err(serde::de::Error::custom)?;

        Ok(RetEnviNFe {
            tp_amb,
            ver_aplic: helper.ver_aplic,
            c_stat: helper.c_stat,
            x_motivo: helper.x_motivo,
            c_uf,
            dh_recbto,
            inf_rec: helper.inf_rec,
            prot_nfe: helper.prot_nfe,
        })
    }
}

/// Receipt information for asynchronous processing
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct InfRec {
    #[serde(rename = "nRec")]
    pub n_rec: String,
    #[serde(rename = "tMed")]
    pub t_med: String,
}

/// NFe Protocol response
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "protNFe")]
pub struct ProtNFe {
    #[serde(rename = "@versao")]
    pub versao: String,
    #[serde(rename = "infProt")]
    pub inf_prot: InfProt,
}

/// Protocol information
#[derive(Debug, PartialEq)]
pub struct InfProt {
    pub tp_amb: Environment,
    pub ver_aplic: String,
    pub ch_nfe: String,
    pub dh_recbto: DateTime<FixedOffset>,
    pub n_prot: Option<String>,
    pub dig_val: Option<String>,
    pub c_stat: String,
    pub x_motivo: String,
}

impl Serialize for InfProt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("infProt", 8)?;
        state.serialize_field("tpAmb", &(self.tp_amb.clone() as u8))?;
        state.serialize_field("verAplic", &self.ver_aplic)?;
        state.serialize_field("chNFe", &self.ch_nfe)?;
        state.serialize_field("dhRecbto", &self.dh_recbto.to_rfc3339())?;
        if let Some(ref n_prot) = self.n_prot {
            state.serialize_field("nProt", n_prot)?;
        }
        if let Some(ref dig_val) = self.dig_val {
            state.serialize_field("digVal", dig_val)?;
        }
        state.serialize_field("cStat", &self.c_stat)?;
        state.serialize_field("xMotivo", &self.x_motivo)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for InfProt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct InfProtHelper {
            #[serde(rename = "tpAmb")]
            tp_amb: u8,
            #[serde(rename = "verAplic")]
            ver_aplic: String,
            #[serde(rename = "chNFe")]
            ch_nfe: String,
            #[serde(rename = "dhRecbto")]
            dh_recbto: String,
            #[serde(rename = "nProt")]
            n_prot: Option<String>,
            #[serde(rename = "digVal")]
            dig_val: Option<String>,
            #[serde(rename = "cStat")]
            c_stat: String,
            #[serde(rename = "xMotivo")]
            x_motivo: String,
        }

        let helper = InfProtHelper::deserialize(deserializer)?;
        let tp_amb = Environment::try_from(helper.tp_amb).map_err(serde::de::Error::custom)?;
        let dh_recbto =
            DateTime::parse_from_rfc3339(&helper.dh_recbto).map_err(serde::de::Error::custom)?;

        Ok(InfProt {
            tp_amb,
            ver_aplic: helper.ver_aplic,
            ch_nfe: helper.ch_nfe,
            dh_recbto,
            n_prot: helper.n_prot,
            dig_val: helper.dig_val,
            c_stat: helper.c_stat,
            x_motivo: helper.x_motivo,
        })
    }
}

/// Request to query batch processing result
#[derive(Debug, PartialEq)]
pub struct ConsReciNFe {
    /// Environment
    pub tp_amb: Environment,
    /// Receipt number
    pub n_rec: String,
}

impl ConsReciNFe {
    pub fn new(environment: Environment, n_rec: String) -> Self {
        ConsReciNFe {
            tp_amb: environment,
            n_rec,
        }
    }
}

impl Serialize for ConsReciNFe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("consReciNFe", 4)?;
        state.serialize_field("@xmlns", namespaces::NFE_DATA)?;
        state.serialize_field("@versao", SUPPORTED_VERSION)?;
        state.serialize_field("tpAmb", &(self.tp_amb.clone() as u8))?;
        state.serialize_field("nRec", &self.n_rec)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for ConsReciNFe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ConsReciNFeHelper {
            #[serde(rename = "@xmlns")]
            xmlns: String,
            #[serde(rename = "@versao")]
            versao: String,
            #[serde(rename = "tpAmb")]
            tp_amb: u8,
            #[serde(rename = "nRec")]
            n_rec: String,
        }

        let helper = ConsReciNFeHelper::deserialize(deserializer)?;

        if helper.xmlns != namespaces::NFE_DATA {
            return Err(serde::de::Error::custom(format!(
                "Invalid xmlns: expected '{}', found '{}'",
                namespaces::NFE_DATA,
                helper.xmlns
            )));
        }

        if helper.versao != SUPPORTED_VERSION {
            return Err(serde::de::Error::custom(format!(
                "Unsupported version: expected '{}', found '{}'",
                SUPPORTED_VERSION, helper.versao
            )));
        }

        let tp_amb = Environment::try_from(helper.tp_amb).map_err(serde::de::Error::custom)?;

        Ok(ConsReciNFe {
            tp_amb,
            n_rec: helper.n_rec,
        })
    }
}

/// Response from batch processing query
#[derive(Debug, PartialEq)]
pub struct RetConsReciNFe {
    pub tp_amb: Environment,
    pub ver_aplic: String,
    pub n_rec: String,
    pub c_stat: String,
    pub x_motivo: String,
    pub c_uf: State,
    pub dh_recbto: DateTime<FixedOffset>,
    pub prot_nfe: Option<Vec<ProtNFe>>,
}

impl Serialize for RetConsReciNFe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("retConsReciNFe", 9)?;
        state.serialize_field("@versao", SUPPORTED_VERSION)?;
        state.serialize_field("tpAmb", &(self.tp_amb.clone() as u8))?;
        state.serialize_field("verAplic", &self.ver_aplic)?;
        state.serialize_field("nRec", &self.n_rec)?;
        state.serialize_field("cStat", &self.c_stat)?;
        state.serialize_field("xMotivo", &self.x_motivo)?;
        state.serialize_field("cUF", &self.c_uf.code())?;
        state.serialize_field("dhRecbto", &self.dh_recbto.to_rfc3339())?;
        if let Some(ref prot_nfe) = self.prot_nfe {
            state.serialize_field("protNFe", prot_nfe)?;
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for RetConsReciNFe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RetConsReciNFeHelper {
            #[serde(rename = "@versao")]
            versao: String,
            #[serde(rename = "tpAmb")]
            tp_amb: u8,
            #[serde(rename = "verAplic")]
            ver_aplic: String,
            #[serde(rename = "nRec")]
            n_rec: String,
            #[serde(rename = "cStat")]
            c_stat: String,
            #[serde(rename = "xMotivo")]
            x_motivo: String,
            #[serde(rename = "cUF")]
            c_uf: u8,
            #[serde(rename = "dhRecbto")]
            dh_recbto: String,
            #[serde(rename = "protNFe")]
            prot_nfe: Option<Vec<ProtNFe>>,
        }

        let helper = RetConsReciNFeHelper::deserialize(deserializer)?;

        if helper.versao != SUPPORTED_VERSION {
            return Err(serde::de::Error::custom(format!(
                "Unsupported version: expected '{}', found '{}'",
                SUPPORTED_VERSION, helper.versao
            )));
        }

        let tp_amb = Environment::try_from(helper.tp_amb).map_err(serde::de::Error::custom)?;
        let c_uf = State::try_from(helper.c_uf).map_err(serde::de::Error::custom)?;
        let dh_recbto =
            DateTime::parse_from_rfc3339(&helper.dh_recbto).map_err(serde::de::Error::custom)?;

        Ok(RetConsReciNFe {
            tp_amb,
            ver_aplic: helper.ver_aplic,
            n_rec: helper.n_rec,
            c_stat: helper.c_stat,
            x_motivo: helper.x_motivo,
            c_uf,
            dh_recbto,
            prot_nfe: helper.prot_nfe,
        })
    }
}

/// Service query type enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServiceQueryType {
    #[serde(rename = "CONSULTAR")]
    Consultar,
}

impl Default for ServiceQueryType {
    fn default() -> Self {
        ServiceQueryType::Consultar
    }
}

/// Request to query NFe by access key
#[derive(Debug, PartialEq)]
pub struct ConsSitNFe {
    /// Environment
    pub tp_amb: Environment,
    /// Service type
    pub x_serv: ServiceQueryType,
    /// Access key (44 digits)
    pub ch_nfe: String,
}

impl ConsSitNFe {
    pub fn new(environment: Environment, ch_nfe: String) -> Self {
        ConsSitNFe {
            tp_amb: environment,
            x_serv: ServiceQueryType::default(),
            ch_nfe,
        }
    }
}

impl Serialize for ConsSitNFe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("consSitNFe", 5)?;
        state.serialize_field("@xmlns", namespaces::NFE_DATA)?;
        state.serialize_field("@versao", SUPPORTED_VERSION)?;
        state.serialize_field("tpAmb", &(self.tp_amb.clone() as u8))?;
        state.serialize_field("xServ", &self.x_serv)?;
        state.serialize_field("chNFe", &self.ch_nfe)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for ConsSitNFe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ConsSitNFeHelper {
            #[serde(rename = "@xmlns")]
            xmlns: String,
            #[serde(rename = "@versao")]
            versao: String,
            #[serde(rename = "tpAmb")]
            tp_amb: u8,
            #[serde(rename = "xServ")]
            x_serv: ServiceQueryType,
            #[serde(rename = "chNFe")]
            ch_nfe: String,
        }

        let helper = ConsSitNFeHelper::deserialize(deserializer)?;

        if helper.xmlns != namespaces::NFE_DATA {
            return Err(serde::de::Error::custom(format!(
                "Invalid xmlns: expected '{}', found '{}'",
                namespaces::NFE_DATA,
                helper.xmlns
            )));
        }

        if helper.versao != SUPPORTED_VERSION {
            return Err(serde::de::Error::custom(format!(
                "Unsupported version: expected '{}', found '{}'",
                SUPPORTED_VERSION, helper.versao
            )));
        }

        let tp_amb = Environment::try_from(helper.tp_amb).map_err(serde::de::Error::custom)?;

        Ok(ConsSitNFe {
            tp_amb,
            x_serv: helper.x_serv,
            ch_nfe: helper.ch_nfe,
        })
    }
}

/// Response from NFe protocol query
#[derive(Debug, PartialEq)]
pub struct RetConsSitNFe {
    pub tp_amb: Environment,
    pub ver_aplic: String,
    pub c_stat: String,
    pub x_motivo: String,
    pub c_uf: State,
    pub prot_nfe: Option<ProtNFe>,
}

impl Serialize for RetConsSitNFe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("retConsSitNFe", 7)?;
        state.serialize_field("@versao", SUPPORTED_VERSION)?;
        state.serialize_field("tpAmb", &(self.tp_amb.clone() as u8))?;
        state.serialize_field("verAplic", &self.ver_aplic)?;
        state.serialize_field("cStat", &self.c_stat)?;
        state.serialize_field("xMotivo", &self.x_motivo)?;
        state.serialize_field("cUF", &self.c_uf.code())?;
        if let Some(ref prot_nfe) = self.prot_nfe {
            state.serialize_field("protNFe", prot_nfe)?;
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for RetConsSitNFe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RetConsSitNFeHelper {
            #[serde(rename = "@versao")]
            versao: String,
            #[serde(rename = "tpAmb")]
            tp_amb: u8,
            #[serde(rename = "verAplic")]
            ver_aplic: String,
            #[serde(rename = "cStat")]
            c_stat: String,
            #[serde(rename = "xMotivo")]
            x_motivo: String,
            #[serde(rename = "cUF")]
            c_uf: u8,
            #[serde(rename = "protNFe")]
            prot_nfe: Option<ProtNFe>,
        }

        let helper = RetConsSitNFeHelper::deserialize(deserializer)?;

        if helper.versao != SUPPORTED_VERSION {
            return Err(serde::de::Error::custom(format!(
                "Unsupported version: expected '{}', found '{}'",
                SUPPORTED_VERSION, helper.versao
            )));
        }

        let tp_amb = Environment::try_from(helper.tp_amb).map_err(serde::de::Error::custom)?;
        let c_uf = State::try_from(helper.c_uf).map_err(serde::de::Error::custom)?;

        Ok(RetConsSitNFe {
            tp_amb,
            ver_aplic: helper.ver_aplic,
            c_stat: helper.c_stat,
            x_motivo: helper.x_motivo,
            c_uf,
            prot_nfe: helper.prot_nfe,
        })
    }
}

/// Service status query type enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusServiceType {
    #[serde(rename = "STATUS")]
    Status,
}

impl Default for StatusServiceType {
    fn default() -> Self {
        StatusServiceType::Status
    }
}

/// Request to query service status
#[derive(Debug, PartialEq)]
pub struct ConsStatServ {
    /// Environment
    pub tp_amb: Environment,
    /// State code
    pub c_uf: State,
    /// Service type
    pub x_serv: StatusServiceType,
}

impl ConsStatServ {
    pub fn new(state: State, environment: Environment) -> Self {
        ConsStatServ {
            tp_amb: environment,
            c_uf: state,
            x_serv: StatusServiceType::default(),
        }
    }
}

impl Serialize for ConsStatServ {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("consStatServ", 5)?;
        state.serialize_field("@xmlns", namespaces::NFE_DATA)?;
        state.serialize_field("@versao", SUPPORTED_VERSION)?;
        state.serialize_field("tpAmb", &(self.tp_amb.clone() as u8))?;
        state.serialize_field("cUF", &self.c_uf.code())?;
        state.serialize_field("xServ", &self.x_serv)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for ConsStatServ {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ConsStatServHelper {
            #[serde(rename = "@xmlns")]
            xmlns: String,
            #[serde(rename = "@versao")]
            versao: String,
            #[serde(rename = "tpAmb")]
            tp_amb: u8,
            #[serde(rename = "cUF")]
            c_uf: u8,
            #[serde(rename = "xServ")]
            x_serv: StatusServiceType,
        }

        let helper = ConsStatServHelper::deserialize(deserializer)?;

        if helper.xmlns != namespaces::NFE_DATA {
            return Err(serde::de::Error::custom(format!(
                "Invalid xmlns: expected '{}', found '{}'",
                namespaces::NFE_DATA,
                helper.xmlns
            )));
        }

        if helper.versao != SUPPORTED_VERSION {
            return Err(serde::de::Error::custom(format!(
                "Unsupported version: expected '{}', found '{}'",
                SUPPORTED_VERSION, helper.versao
            )));
        }

        let tp_amb = Environment::try_from(helper.tp_amb).map_err(serde::de::Error::custom)?;
        let c_uf = State::try_from(helper.c_uf).map_err(serde::de::Error::custom)?;

        Ok(ConsStatServ {
            tp_amb,
            c_uf,
            x_serv: helper.x_serv,
        })
    }
}

/// Response from service status query
#[derive(Debug, PartialEq)]
pub struct RetConsStatServ {
    pub tp_amb: Environment,
    pub ver_aplic: String,
    pub c_stat: String,
    pub x_motivo: String,
    pub c_uf: State,
    pub dh_recbto: DateTime<FixedOffset>,
    pub t_med: Option<String>,
    pub dh_retorno: Option<DateTime<FixedOffset>>,
    pub x_obs: Option<String>,
}

impl Serialize for RetConsStatServ {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("retConsStatServ", 10)?;
        state.serialize_field("@versao", SUPPORTED_VERSION)?;
        state.serialize_field("tpAmb", &(self.tp_amb.clone() as u8))?;
        state.serialize_field("verAplic", &self.ver_aplic)?;
        state.serialize_field("cStat", &self.c_stat)?;
        state.serialize_field("xMotivo", &self.x_motivo)?;
        state.serialize_field("cUF", &self.c_uf.code())?;
        state.serialize_field("dhRecbto", &self.dh_recbto.to_rfc3339())?;
        if let Some(ref t_med) = self.t_med {
            state.serialize_field("tMed", t_med)?;
        }
        if let Some(ref dh_retorno) = self.dh_retorno {
            state.serialize_field("dhRetorno", &dh_retorno.to_rfc3339())?;
        }
        if let Some(ref x_obs) = self.x_obs {
            state.serialize_field("xObs", x_obs)?;
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for RetConsStatServ {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RetConsStatServHelper {
            #[serde(rename = "@versao")]
            versao: String,
            #[serde(rename = "tpAmb")]
            tp_amb: u8,
            #[serde(rename = "verAplic")]
            ver_aplic: String,
            #[serde(rename = "cStat")]
            c_stat: String,
            #[serde(rename = "xMotivo")]
            x_motivo: String,
            #[serde(rename = "cUF")]
            c_uf: u8,
            #[serde(rename = "dhRecbto")]
            dh_recbto: String,
            #[serde(rename = "tMed")]
            t_med: Option<String>,
            #[serde(rename = "dhRetorno")]
            dh_retorno: Option<String>,
            #[serde(rename = "xObs")]
            x_obs: Option<String>,
        }

        let helper = RetConsStatServHelper::deserialize(deserializer)?;

        if helper.versao != SUPPORTED_VERSION {
            return Err(serde::de::Error::custom(format!(
                "Unsupported version: expected '{}', found '{}'",
                SUPPORTED_VERSION, helper.versao
            )));
        }

        let tp_amb = Environment::try_from(helper.tp_amb).map_err(serde::de::Error::custom)?;
        let c_uf = State::try_from(helper.c_uf).map_err(serde::de::Error::custom)?;
        let dh_recbto =
            DateTime::parse_from_rfc3339(&helper.dh_recbto).map_err(serde::de::Error::custom)?;
        let dh_retorno = match helper.dh_retorno {
            Some(s) => Some(DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?),
            None => None,
        };

        Ok(RetConsStatServ {
            tp_amb,
            ver_aplic: helper.ver_aplic,
            c_stat: helper.c_stat,
            x_motivo: helper.x_motivo,
            c_uf,
            dh_recbto,
            t_med: helper.t_med,
            dh_retorno,
            x_obs: helper.x_obs,
        })
    }
}

/// Inutilization service type enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InutServiceType {
    #[serde(rename = "INUTILIZAR")]
    Inutilizar,
}

impl Default for InutServiceType {
    fn default() -> Self {
        InutServiceType::Inutilizar
    }
}

/// Request to invalidate NFe number range
#[derive(Debug, PartialEq)]
pub struct InutNFe {
    /// Info about invalidation request
    pub inf_inut: InfInut,
}

impl Serialize for InutNFe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("inutNFe", 3)?;
        state.serialize_field("@xmlns", namespaces::NFE_DATA)?;
        state.serialize_field("@versao", SUPPORTED_VERSION)?;
        state.serialize_field("infInut", &self.inf_inut)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for InutNFe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct InutNFeHelper {
            #[serde(rename = "@xmlns")]
            xmlns: String,
            #[serde(rename = "@versao")]
            versao: String,
            #[serde(rename = "infInut")]
            inf_inut: InfInut,
        }

        let helper = InutNFeHelper::deserialize(deserializer)?;

        if helper.xmlns != namespaces::NFE_DATA {
            return Err(serde::de::Error::custom(format!(
                "Invalid xmlns: expected '{}', found '{}'",
                namespaces::NFE_DATA,
                helper.xmlns
            )));
        }

        if helper.versao != SUPPORTED_VERSION {
            return Err(serde::de::Error::custom(format!(
                "Unsupported version: expected '{}', found '{}'",
                SUPPORTED_VERSION, helper.versao
            )));
        }

        Ok(InutNFe {
            inf_inut: helper.inf_inut,
        })
    }
}

/// Invalidation request information
#[derive(Debug, PartialEq)]
pub struct InfInut {
    pub id: String,
    pub tp_amb: Environment,
    pub x_serv: InutServiceType,
    pub c_uf: State,
    pub ano: String,
    pub cnpj: CNPJ,
    pub modelo: Model,
    pub serie: u8,
    pub n_nf_ini: u32,
    pub n_nf_fin: u32,
    pub x_just: String,
}

impl InfInut {
    /// Generate the ID for the invalidation request
    pub fn generate_id(
        c_uf: &State,
        ano: &str,
        cnpj: &CNPJ,
        modelo: &Model,
        serie: u8,
        n_nf_ini: u32,
        n_nf_fin: u32,
    ) -> String {
        format!(
            "ID{:02}{}{:0>14}{:02}{:03}{:09}{:09}",
            c_uf.code(),
            ano,
            cnpj.0,
            modelo.code(),
            serie,
            n_nf_ini,
            n_nf_fin
        )
    }
}

impl Serialize for InfInut {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("infInut", 11)?;
        state.serialize_field("@Id", &self.id)?;
        state.serialize_field("tpAmb", &(self.tp_amb.clone() as u8))?;
        state.serialize_field("xServ", &self.x_serv)?;
        state.serialize_field("cUF", &self.c_uf.code())?;
        state.serialize_field("ano", &self.ano)?;
        state.serialize_field("CNPJ", &self.cnpj.0)?;
        state.serialize_field("mod", &self.modelo.code())?;
        state.serialize_field("serie", &self.serie)?;
        state.serialize_field("nNFIni", &self.n_nf_ini)?;
        state.serialize_field("nNFFin", &self.n_nf_fin)?;
        state.serialize_field("xJust", &self.x_just)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for InfInut {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct InfInutHelper {
            #[serde(rename = "@Id")]
            id: String,
            #[serde(rename = "tpAmb")]
            tp_amb: u8,
            #[serde(rename = "xServ")]
            x_serv: InutServiceType,
            #[serde(rename = "cUF")]
            c_uf: u8,
            #[serde(rename = "ano")]
            ano: String,
            #[serde(rename = "CNPJ")]
            cnpj: String,
            #[serde(rename = "mod")]
            modelo: u8,
            #[serde(rename = "serie")]
            serie: u8,
            #[serde(rename = "nNFIni")]
            n_nf_ini: u32,
            #[serde(rename = "nNFFin")]
            n_nf_fin: u32,
            #[serde(rename = "xJust")]
            x_just: String,
        }

        let helper = InfInutHelper::deserialize(deserializer)?;

        let tp_amb = Environment::try_from(helper.tp_amb).map_err(serde::de::Error::custom)?;
        let c_uf = State::try_from(helper.c_uf).map_err(serde::de::Error::custom)?;
        let modelo = Model::try_from(helper.modelo).map_err(serde::de::Error::custom)?;

        Ok(InfInut {
            id: helper.id,
            tp_amb,
            x_serv: helper.x_serv,
            c_uf,
            ano: helper.ano,
            cnpj: CNPJ(helper.cnpj),
            modelo,
            serie: helper.serie,
            n_nf_ini: helper.n_nf_ini,
            n_nf_fin: helper.n_nf_fin,
            x_just: helper.x_just,
        })
    }
}

/// Response from invalidation request
#[derive(Debug, PartialEq)]
pub struct RetInutNFe {
    pub inf_inut: RetInfInut,
}

impl Serialize for RetInutNFe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("retInutNFe", 2)?;
        state.serialize_field("@versao", SUPPORTED_VERSION)?;
        state.serialize_field("infInut", &self.inf_inut)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for RetInutNFe {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RetInutNFeHelper {
            #[serde(rename = "@versao")]
            versao: String,
            #[serde(rename = "infInut")]
            inf_inut: RetInfInut,
        }

        let helper = RetInutNFeHelper::deserialize(deserializer)?;

        if helper.versao != SUPPORTED_VERSION {
            return Err(serde::de::Error::custom(format!(
                "Unsupported version: expected '{}', found '{}'",
                SUPPORTED_VERSION, helper.versao
            )));
        }

        Ok(RetInutNFe {
            inf_inut: helper.inf_inut,
        })
    }
}

/// Invalidation response information
#[derive(Debug, PartialEq)]
pub struct RetInfInut {
    pub tp_amb: Environment,
    pub ver_aplic: String,
    pub c_stat: String,
    pub x_motivo: String,
    pub c_uf: State,
    pub ano: Option<String>,
    pub cnpj: Option<CNPJ>,
    pub modelo: Option<Model>,
    pub serie: Option<u8>,
    pub n_nf_ini: Option<u32>,
    pub n_nf_fin: Option<u32>,
    pub dh_recbto: Option<DateTime<FixedOffset>>,
    pub n_prot: Option<String>,
}

impl Serialize for RetInfInut {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("infInut", 13)?;
        state.serialize_field("tpAmb", &(self.tp_amb.clone() as u8))?;
        state.serialize_field("verAplic", &self.ver_aplic)?;
        state.serialize_field("cStat", &self.c_stat)?;
        state.serialize_field("xMotivo", &self.x_motivo)?;
        state.serialize_field("cUF", &self.c_uf.code())?;
        if let Some(ref ano) = self.ano {
            state.serialize_field("ano", ano)?;
        }
        if let Some(ref cnpj) = self.cnpj {
            state.serialize_field("CNPJ", &cnpj.0)?;
        }
        if let Some(ref modelo) = self.modelo {
            state.serialize_field("mod", &modelo.code())?;
        }
        if let Some(ref serie) = self.serie {
            state.serialize_field("serie", serie)?;
        }
        if let Some(ref n_nf_ini) = self.n_nf_ini {
            state.serialize_field("nNFIni", n_nf_ini)?;
        }
        if let Some(ref n_nf_fin) = self.n_nf_fin {
            state.serialize_field("nNFFin", n_nf_fin)?;
        }
        if let Some(ref dh_recbto) = self.dh_recbto {
            state.serialize_field("dhRecbto", &dh_recbto.to_rfc3339())?;
        }
        if let Some(ref n_prot) = self.n_prot {
            state.serialize_field("nProt", n_prot)?;
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for RetInfInut {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RetInfInutHelper {
            #[serde(rename = "tpAmb")]
            tp_amb: u8,
            #[serde(rename = "verAplic")]
            ver_aplic: String,
            #[serde(rename = "cStat")]
            c_stat: String,
            #[serde(rename = "xMotivo")]
            x_motivo: String,
            #[serde(rename = "cUF")]
            c_uf: u8,
            #[serde(rename = "ano")]
            ano: Option<String>,
            #[serde(rename = "CNPJ")]
            cnpj: Option<String>,
            #[serde(rename = "mod")]
            modelo: Option<u8>,
            #[serde(rename = "serie")]
            serie: Option<u8>,
            #[serde(rename = "nNFIni")]
            n_nf_ini: Option<u32>,
            #[serde(rename = "nNFFin")]
            n_nf_fin: Option<u32>,
            #[serde(rename = "dhRecbto")]
            dh_recbto: Option<String>,
            #[serde(rename = "nProt")]
            n_prot: Option<String>,
        }

        let helper = RetInfInutHelper::deserialize(deserializer)?;

        let tp_amb = Environment::try_from(helper.tp_amb).map_err(serde::de::Error::custom)?;
        let c_uf = State::try_from(helper.c_uf).map_err(serde::de::Error::custom)?;
        let modelo = match helper.modelo {
            Some(m) => Some(Model::try_from(m).map_err(serde::de::Error::custom)?),
            None => None,
        };
        let dh_recbto = match helper.dh_recbto {
            Some(s) => Some(DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?),
            None => None,
        };

        Ok(RetInfInut {
            tp_amb,
            ver_aplic: helper.ver_aplic,
            c_stat: helper.c_stat,
            x_motivo: helper.x_motivo,
            c_uf,
            ano: helper.ano,
            cnpj: helper.cnpj.map(CNPJ),
            modelo,
            serie: helper.serie,
            n_nf_ini: helper.n_nf_ini,
            n_nf_fin: helper.n_nf_fin,
            dh_recbto,
            n_prot: helper.n_prot,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::canonicalize_xml as canonicalize;
    use nf_e_macros::serialization_test;
    use quick_xml::{de::from_str as deserialize, se::to_string as serialize};

    #[serialization_test(fixture = "../../tests/fixtures/soap/cons_stat_serv.xml")]
    fn setup_cons_stat_serv() -> ConsStatServ {
        ConsStatServ::new(State::MinasGerais, Environment::Homologation)
    }

    #[serialization_test(fixture = "../../tests/fixtures/soap/cons_sit_nfe.xml")]
    fn setup_cons_sit_nfe() -> ConsSitNFe {
        ConsSitNFe::new(
            Environment::Homologation,
            "12345678901234567890123456789012345678901234".to_string(),
        )
    }

    #[serialization_test(fixture = "../../tests/fixtures/soap/cons_reci_nfe.xml")]
    fn setup_cons_reci_nfe() -> ConsReciNFe {
        ConsReciNFe::new(Environment::Homologation, "123456789012345".to_string())
    }

    #[test]
    fn test_inf_inut_generate_id() {
        let id = InfInut::generate_id(
            &State::MinasGerais,
            "23",
            &CNPJ("12345678000195".to_string()),
            &Model::NFe,
            1,
            1,
            10,
        );
        assert_eq!(id, "ID31231234567800019555001000000001000000010");
    }
}
