use crate::enums::Environment;
use crate::models::NFe;
use crate::states::State;
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
            _xmlns: Option<String>,
            #[serde(rename = "$value")]
            content: T,
        }

        let helper = NfeDadosHelper::deserialize(deserializer)?;
        Ok(NfeDados {
            content: helper.content,
        })
    }
}

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

    fn version(&self) -> &'static str {
        "4.00"
    }
}

impl Serialize for EnviNFe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("enviNFe", 4)?;
        state.serialize_field("@xmlns", namespaces::NFE_DATA)?;
        state.serialize_field("@versao", &self.version())?;
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
            _xmlns: Option<String>,
            #[serde(rename = "@versao")]
            _versao: String,
            #[serde(rename = "idLote")]
            id_lote: String,
            #[serde(rename = "indSinc")]
            ind_sinc: u8,
            #[serde(rename = "NFe")]
            nfes: Vec<NFe>,
        }

        let helper = EnviNFeHelper::deserialize(deserializer)?;
        Ok(EnviNFe {
            id_lote: helper.id_lote,
            ind_sinc: helper.ind_sinc,
            nfes: helper.nfes,
        })
    }
}

/// Response from NFe authorization request
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "retEnviNFe")]
pub struct RetEnviNFe {
    #[serde(rename = "@versao")]
    pub versao: String,
    #[serde(rename = "tpAmb")]
    pub tp_amb: u8,
    #[serde(rename = "verAplic")]
    pub ver_aplic: String,
    #[serde(rename = "cStat")]
    pub c_stat: String,
    #[serde(rename = "xMotivo")]
    pub x_motivo: String,
    #[serde(rename = "cUF")]
    pub c_uf: u8,
    #[serde(rename = "dhRecbto")]
    pub dh_recbto: String,
    #[serde(rename = "infRec")]
    pub inf_rec: Option<InfRec>,
    #[serde(rename = "protNFe")]
    pub prot_nfe: Option<ProtNFe>,
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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct InfProt {
    #[serde(rename = "tpAmb")]
    pub tp_amb: u8,
    #[serde(rename = "verAplic")]
    pub ver_aplic: String,
    #[serde(rename = "chNFe")]
    pub ch_nfe: String,
    #[serde(rename = "dhRecbto")]
    pub dh_recbto: String,
    #[serde(rename = "nProt")]
    pub n_prot: Option<String>,
    #[serde(rename = "digVal")]
    pub dig_val: Option<String>,
    #[serde(rename = "cStat")]
    pub c_stat: String,
    #[serde(rename = "xMotivo")]
    pub x_motivo: String,
}

/// Request to query batch processing result
#[derive(Debug, PartialEq)]
pub struct ConsReciNFe {
    /// Environment (1=Production, 2=Homologation)
    pub tp_amb: u8,
    /// Receipt number
    pub n_rec: String,
}

impl ConsReciNFe {
    pub fn new(environment: &Environment, n_rec: String) -> Self {
        ConsReciNFe {
            tp_amb: environment.clone() as u8,
            n_rec,
        }
    }

    fn version(&self) -> &'static str {
        "4.00"
    }
}

impl Serialize for ConsReciNFe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("consReciNFe", 4)?;
        state.serialize_field("@xmlns", namespaces::NFE_DATA)?;
        state.serialize_field("@versao", &self.version())?;
        state.serialize_field("tpAmb", &self.tp_amb)?;
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
            _xmlns: Option<String>,
            #[serde(rename = "@versao")]
            _versao: String,
            #[serde(rename = "tpAmb")]
            tp_amb: u8,
            #[serde(rename = "nRec")]
            n_rec: String,
        }

        let helper = ConsReciNFeHelper::deserialize(deserializer)?;
        Ok(ConsReciNFe {
            tp_amb: helper.tp_amb,
            n_rec: helper.n_rec,
        })
    }
}

/// Response from batch processing query
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "retConsReciNFe")]
pub struct RetConsReciNFe {
    #[serde(rename = "@versao")]
    pub versao: String,
    #[serde(rename = "tpAmb")]
    pub tp_amb: u8,
    #[serde(rename = "verAplic")]
    pub ver_aplic: String,
    #[serde(rename = "nRec")]
    pub n_rec: String,
    #[serde(rename = "cStat")]
    pub c_stat: String,
    #[serde(rename = "xMotivo")]
    pub x_motivo: String,
    #[serde(rename = "cUF")]
    pub c_uf: u8,
    #[serde(rename = "dhRecbto")]
    pub dh_recbto: String,
    #[serde(rename = "protNFe")]
    pub prot_nfe: Option<Vec<ProtNFe>>,
}

/// Request to query NFe by access key
#[derive(Debug, PartialEq)]
pub struct ConsSitNFe {
    /// Environment (1=Production, 2=Homologation)
    pub tp_amb: u8,
    /// Service type (always "1" for NFe query)
    pub x_serv: String,
    /// Access key (44 digits)
    pub ch_nfe: String,
}

impl ConsSitNFe {
    pub fn new(environment: &Environment, ch_nfe: String) -> Self {
        ConsSitNFe {
            tp_amb: environment.clone() as u8,
            x_serv: "CONSULTAR".to_string(),
            ch_nfe,
        }
    }

    fn version(&self) -> &'static str {
        "4.00"
    }
}

impl Serialize for ConsSitNFe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("consSitNFe", 5)?;
        state.serialize_field("@xmlns", namespaces::NFE_DATA)?;
        state.serialize_field("@versao", &self.version())?;
        state.serialize_field("tpAmb", &self.tp_amb)?;
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
            _xmlns: Option<String>,
            #[serde(rename = "@versao")]
            _versao: String,
            #[serde(rename = "tpAmb")]
            tp_amb: u8,
            #[serde(rename = "xServ")]
            x_serv: String,
            #[serde(rename = "chNFe")]
            ch_nfe: String,
        }

        let helper = ConsSitNFeHelper::deserialize(deserializer)?;
        Ok(ConsSitNFe {
            tp_amb: helper.tp_amb,
            x_serv: helper.x_serv,
            ch_nfe: helper.ch_nfe,
        })
    }
}

/// Response from NFe protocol query
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "retConsSitNFe")]
pub struct RetConsSitNFe {
    #[serde(rename = "@versao")]
    pub versao: String,
    #[serde(rename = "tpAmb")]
    pub tp_amb: u8,
    #[serde(rename = "verAplic")]
    pub ver_aplic: String,
    #[serde(rename = "cStat")]
    pub c_stat: String,
    #[serde(rename = "xMotivo")]
    pub x_motivo: String,
    #[serde(rename = "cUF")]
    pub c_uf: u8,
    #[serde(rename = "protNFe")]
    pub prot_nfe: Option<ProtNFe>,
}

/// Request to query service status
#[derive(Debug, PartialEq)]
pub struct ConsStatServ {
    /// Environment (1=Production, 2=Homologation)
    pub tp_amb: u8,
    /// State code (UF IBGE code)
    pub c_uf: u8,
    /// Service type (always "STATUS")
    pub x_serv: String,
}

impl ConsStatServ {
    pub fn new(state: &State, environment: &Environment) -> Self {
        ConsStatServ {
            tp_amb: environment.clone() as u8,
            c_uf: state.code(),
            x_serv: "STATUS".to_string(),
        }
    }

    fn version(&self) -> &'static str {
        "4.00"
    }
}

impl Serialize for ConsStatServ {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("consStatServ", 5)?;
        state.serialize_field("@xmlns", namespaces::NFE_DATA)?;
        state.serialize_field("@versao", &self.version())?;
        state.serialize_field("tpAmb", &self.tp_amb)?;
        state.serialize_field("cUF", &self.c_uf)?;
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
            _xmlns: Option<String>,
            #[serde(rename = "@versao")]
            _versao: String,
            #[serde(rename = "tpAmb")]
            tp_amb: u8,
            #[serde(rename = "cUF")]
            c_uf: u8,
            #[serde(rename = "xServ")]
            x_serv: String,
        }

        let helper = ConsStatServHelper::deserialize(deserializer)?;
        Ok(ConsStatServ {
            tp_amb: helper.tp_amb,
            c_uf: helper.c_uf,
            x_serv: helper.x_serv,
        })
    }
}

/// Response from service status query
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "retConsStatServ")]
pub struct RetConsStatServ {
    #[serde(rename = "@versao")]
    pub versao: String,
    #[serde(rename = "tpAmb")]
    pub tp_amb: u8,
    #[serde(rename = "verAplic")]
    pub ver_aplic: String,
    #[serde(rename = "cStat")]
    pub c_stat: String,
    #[serde(rename = "xMotivo")]
    pub x_motivo: String,
    #[serde(rename = "cUF")]
    pub c_uf: u8,
    #[serde(rename = "dhRecbto")]
    pub dh_recbto: String,
    #[serde(rename = "tMed")]
    pub t_med: Option<String>,
    #[serde(rename = "dhRetorno")]
    pub dh_retorno: Option<String>,
    #[serde(rename = "xObs")]
    pub x_obs: Option<String>,
}

/// Request to invalidate NFe number range
#[derive(Debug, PartialEq)]
pub struct InutNFe {
    /// Info about invalidation request
    pub inf_inut: InfInut,
}

impl InutNFe {
    fn version(&self) -> &'static str {
        "4.00"
    }
}

impl Serialize for InutNFe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("inutNFe", 3)?;
        state.serialize_field("@xmlns", namespaces::NFE_DATA)?;
        state.serialize_field("@versao", &self.version())?;
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
            _xmlns: Option<String>,
            #[serde(rename = "@versao")]
            _versao: String,
            #[serde(rename = "infInut")]
            inf_inut: InfInut,
        }

        let helper = InutNFeHelper::deserialize(deserializer)?;
        Ok(InutNFe {
            inf_inut: helper.inf_inut,
        })
    }
}

/// Invalidation request information
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct InfInut {
    #[serde(rename = "@Id")]
    pub id: String,
    #[serde(rename = "tpAmb")]
    pub tp_amb: u8,
    #[serde(rename = "xServ")]
    pub x_serv: String,
    #[serde(rename = "cUF")]
    pub c_uf: u8,
    #[serde(rename = "ano")]
    pub ano: String,
    #[serde(rename = "CNPJ")]
    pub cnpj: String,
    #[serde(rename = "mod")]
    pub modelo: u8,
    #[serde(rename = "serie")]
    pub serie: u8,
    #[serde(rename = "nNFIni")]
    pub n_nf_ini: u32,
    #[serde(rename = "nNFFin")]
    pub n_nf_fin: u32,
    #[serde(rename = "xJust")]
    pub x_just: String,
}

impl InfInut {
    /// Generate the ID for the invalidation request
    pub fn generate_id(
        c_uf: u8,
        ano: &str,
        cnpj: &str,
        modelo: u8,
        serie: u8,
        n_nf_ini: u32,
        n_nf_fin: u32,
    ) -> String {
        format!(
            "ID{:02}{}{:0>14}{:02}{:03}{:09}{:09}",
            c_uf, ano, cnpj, modelo, serie, n_nf_ini, n_nf_fin
        )
    }
}
}

/// Response from invalidation request
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "retInutNFe")]
pub struct RetInutNFe {
    #[serde(rename = "@versao")]
    pub versao: String,
    #[serde(rename = "infInut")]
    pub inf_inut: RetInfInut,
}

/// Invalidation response information
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RetInfInut {
    #[serde(rename = "tpAmb")]
    pub tp_amb: u8,
    #[serde(rename = "verAplic")]
    pub ver_aplic: String,
    #[serde(rename = "cStat")]
    pub c_stat: String,
    #[serde(rename = "xMotivo")]
    pub x_motivo: String,
    #[serde(rename = "cUF")]
    pub c_uf: u8,
    #[serde(rename = "ano")]
    pub ano: Option<String>,
    #[serde(rename = "CNPJ")]
    pub cnpj: Option<String>,
    #[serde(rename = "mod")]
    pub modelo: Option<u8>,
    #[serde(rename = "serie")]
    pub serie: Option<u8>,
    #[serde(rename = "nNFIni")]
    pub n_nf_ini: Option<u32>,
    #[serde(rename = "nNFFin")]
    pub n_nf_fin: Option<u32>,
    #[serde(rename = "dhRecbto")]
    pub dh_recbto: Option<String>,
    #[serde(rename = "nProt")]
    pub n_prot: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::se::to_string as serialize;

    #[test]
    fn test_cons_stat_serv_serialization() {
        let request = ConsStatServ::new(&State::MinasGerais, &Environment::Homologation);
        let xml = serialize(&request).expect("Failed to serialize ConsStatServ");
        assert!(xml.contains("consStatServ"));
        assert!(xml.contains("tpAmb>2<"));
        assert!(xml.contains("cUF>31<"));
        assert!(xml.contains("xServ>STATUS<"));
        assert!(xml.contains("versao=\"4.00\""));
    }

    #[test]
    fn test_cons_sit_nfe_serialization() {
        let request = ConsSitNFe::new(
            &Environment::Homologation,
            "12345678901234567890123456789012345678901234".to_string(),
        );
        let xml = serialize(&request).expect("Failed to serialize ConsSitNFe");
        assert!(xml.contains("consSitNFe"));
        assert!(xml.contains("tpAmb>2<"));
        assert!(xml.contains("xServ>CONSULTAR<"));
        assert!(xml.contains("chNFe>12345678901234567890123456789012345678901234<"));
    }

    #[test]
    fn test_cons_reci_nfe_serialization() {
        let request = ConsReciNFe::new(&Environment::Homologation, "123456789012345".to_string());
        let xml = serialize(&request).expect("Failed to serialize ConsReciNFe");
        assert!(xml.contains("consReciNFe"));
        assert!(xml.contains("tpAmb>2<"));
        assert!(xml.contains("nRec>123456789012345<"));
    }

    #[test]
    fn test_inf_inut_generate_id() {
        let id = InfInut::generate_id(31, "23", "12345678000195", 55, 1, 1, 10);
        assert_eq!(id, "ID31231234567800019555001000000001000000010");
    }
}
