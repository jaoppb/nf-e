use crate::enums::*;

use crate::LIBRARY_VERSION;
use crate::config::ConfigError;
use crate::states::{City, Location, State};
use crate::utils::left_pad;
use chrono::Datelike;
use nf_e_macros::MethodAlgorithm;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct F64(pub f64);

impl Serialize for F64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:.2}", self.0))
    }
}

impl From<f64> for F64 {
    fn from(value: f64) -> Self {
        F64(value)
    }
}

impl AsRef<f64> for F64 {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename = "autXML")]
pub struct Authorized {
    #[serde(rename = "$value")]
    pub documents: Vec<PersonDocument>,
}

#[derive(Default, PartialEq, Debug)]
pub struct Transport {
    pub r#type: TransportType,
}

impl Serialize for Transport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("transp", 1)?;
        state.serialize_field("modFrete", &(self.r#type.clone() as u8))?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Transport {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TransportHelper {
            #[serde(rename = "modFrete")]
            mod_frete: u8,
        }

        let helper = TransportHelper::deserialize(deserializer)?;
        let r#type = TransportType::try_from(helper.mod_frete).map_err(serde::de::Error::custom)?;

        Ok(Transport { r#type })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NFe {
    pub info: Info,
    pub signature: Signature,
}

impl NFe {
    // TODO: Implement digital signature generation and verification and complete test
    pub fn new(info: Info) -> Self {
        let id = info.id();
        Self {
            info,
            signature: Signature {
                info: SignatureInfo {
                    canonicalization_method: CanonicalizationMethod,
                    signature_method: SignatureMethod,
                    reference: SignatureReference {
                        uri: format!("#{}", id),
                        transforms: SignatureTransforms,
                        digest_method: DigestMethod,
                        digest_value: "".to_string(),
                    },
                },
                key_info: KeyInfo {
                    data: X509Data {
                        certificate: "".to_string(),
                    },
                },
                value: Vec::new(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Signature {
    pub info: SignatureInfo,
    pub value: Vec<u8>,
    pub key_info: KeyInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SignatureInfo {
    pub canonicalization_method: CanonicalizationMethod,
    pub signature_method: SignatureMethod,
    pub reference: SignatureReference,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SignatureReference {
    #[serde(rename = "@URI")]
    pub uri: String,
    #[serde(rename = "Transforms")]
    pub transforms: SignatureTransforms,
    #[serde(rename = "DigestMethod")]
    pub digest_method: DigestMethod,
    #[serde(rename = "DigestValue")]
    pub digest_value: String,
}

#[derive(Debug, PartialEq)]
pub struct SignatureTransforms;

impl SignatureTransforms {
    fn transforms() -> Vec<SignatureTransform> {
        vec![
            SignatureTransform::SignatureEnvelopedTransform(SignatureEnvelopedTransform),
            SignatureTransform::SignatureCanonicalizedTransform(SignatureCanonicalizedTransform),
        ]
    }
}

impl Serialize for SignatureTransforms {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let transforms = Self::transforms();
        let mut seq = serializer.serialize_seq(Some(transforms.len()))?;
        for transform in transforms {
            seq.serialize_element(&transform)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for SignatureTransforms {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            #[serde(rename = "Transform")]
            transforms: Vec<SignatureTransform>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let expected_transforms = Self::transforms();

        if helper.transforms != expected_transforms {
            return Err(serde::de::Error::custom(
                "Transforms do not match expected values",
            ));
        }

        Ok(SignatureTransforms)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SignatureTransform {
    SignatureEnvelopedTransform(SignatureEnvelopedTransform),
    SignatureCanonicalizedTransform(SignatureCanonicalizedTransform),
}

#[derive(MethodAlgorithm, Debug, PartialEq)]
#[method_algorithm("http://www.w3.org/2000/09/xmldsig#enveloped-signature")]
pub struct SignatureEnvelopedTransform;

#[derive(MethodAlgorithm, Debug, PartialEq)]
#[method_algorithm("http://www.w3.org/TR/2001/REC-xml-c14n-20010315")]
pub struct SignatureCanonicalizedTransform;

#[derive(MethodAlgorithm, Debug, PartialEq)]
#[method_algorithm("http://www.w3.org/2000/09/xmldsig#sha1")]
pub struct DigestMethod;

#[derive(MethodAlgorithm, Debug, PartialEq)]
#[method_algorithm("http://www.w3.org/TR/2001/REC-xml-c14n-20010315")]
pub struct CanonicalizationMethod;

#[derive(MethodAlgorithm, Debug, PartialEq)]
#[method_algorithm("http://www.w3.org/2000/09/xmldsig#rsa-sha1")]
pub struct SignatureMethod;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct KeyInfo {
    #[serde(rename = "X509Data")]
    pub data: X509Data,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct X509Data {
    #[serde(rename = "X509Certificate")]
    pub certificate: String,
}

/// Main structure based on the XML structure of the NFe
///
/// The fields are public but use the `InfoBuilder` to create the structure.
///
/// Id: Identifier of the NFe (Id) - Format "NFe{chave}"
/// identification: Identification structure (ide)
/// issuer: Issuer structure (emit)
/// details: Details structure (det)
/// version: Fixed value "4.00" (@versao)
#[derive(Debug, PartialEq)]
pub struct Info {
    pub identification: Identification,
    pub issuer: Issuer,
    pub details: Vec<Detail>,
    pub authorized: Option<Authorized>,
    pub total: Total,
    pub transport: Transport,
    pub payments: Payments,
}

impl Info {
    pub fn version(&self) -> String {
        "4.00".to_string()
    }

    fn verifier_digit(&self, id: &str) -> u8 {
        let mut weight = 4;
        let remainder = id.chars().fold(0, |acc, d| {
            let d = d
                .to_digit(10)
                .unwrap_or_else(|| panic!("verifier_digit: failed to parse digit '{}'", d));
            let result = d * weight;
            weight = if weight <= 2 { 9 } else { weight - 1 };
            acc + result
        }) % 11;
        if remainder > 1 {
            11 - remainder as u8
        } else {
            0
        }
    }

    pub fn bare_id(&self) -> String {
        let mut id = String::new();
        id.push_str(&self.identification.location.state.code().to_string());
        id.push_str(&self.identification.emission_date.year().to_string()[2..]);
        id.push_str(&self.identification.emission_date.month().to_string());
        id.push_str(left_pad(self.issuer.document.as_str(), 14, '0').as_str());
        id.push_str(&self.identification.model.code().to_string());
        id.push_str(left_pad(&self.identification.series.to_string(), 3, '0').as_str());
        id.push_str(left_pad(&self.identification.number.to_string(), 9, '0').as_str());
        id.push_str(&self.identification.emission_type.code().to_string());
        id.push_str(left_pad(&self.identification.numeric_code.to_string(), 8, '0').as_str());
        assert_eq!(id.len(), 43);
        id
    }

    /// Generates the NFe key (chave) based on the identification and issuer information
    /// The key is composed of:
    /// - State code (cUF) - 2 digits
    /// - Year and month of emission (AA/MM) - 4 digits
    /// - CNPJ of the issuer - 14 digits (left-padded with zeros)
    /// - Model of the NFe (mod) - 2 digits
    /// - Series of the NFe (serie) - 3 digits (left-padded with zeros)
    /// - Number of the NFe (nNF) - 9 digits (left-padded with zeros)
    /// - Type of emission (tpEmis) - 1 digit
    /// - Numeric code (cNF) - 8 digits (left-padded with zeros)
    /// - Verifier digit (cDV) - 1 digit (calculated using a modulus 11 algorithm)
    ///   Returns the complete key in the format "NFe{chave}"
    pub fn id(&self) -> String {
        let id = self.bare_id();
        format!("NFe{}{}", id, self.verifier_digit(&id))
    }
}

impl Serialize for Info {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct IndexedDetail<'a> {
            #[serde(flatten)]
            detail: &'a Detail,
            #[serde(rename = "@nItem")]
            index: usize,
        }

        let len = 6 + self.authorized.is_some() as usize;

        let mut state = serializer.serialize_struct("infNFe", len)?;
        state.serialize_field("@versao", &self.version())?;
        state.serialize_field("@Id", &self.id())?;
        state.serialize_field("ide", &self.identification)?;
        state.serialize_field("emit", &self.issuer)?;
        if self.authorized.is_some() {
            state.serialize_field("autXML", &self.authorized)?;
        }
        state.serialize_field("total", &self.total)?;
        state.serialize_field("pag", &self.payments)?;
        state.serialize_field("transp", &self.transport)?;
        state.serialize_field(
            "det",
            &self
                .details
                .iter()
                .enumerate()
                .map(|(index, detail)| IndexedDetail {
                    detail,
                    index: index + 1,
                })
                .collect::<Vec<_>>(),
        )?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Info {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct InfoHelper {
            #[serde(rename = "@versao")]
            versao: String,
            #[serde(rename = "@Id")]
            id: String,
            #[serde(rename = "ide")]
            identification: Identification,
            #[serde(rename = "emit")]
            issuer: Issuer,
            #[serde(rename = "det")]
            details: Vec<Detail>,
            #[serde(rename = "autXML")]
            authorized: Option<Authorized>,
            total: Total,
            #[serde(rename = "transp")]
            transport: Transport,
            #[serde(rename = "pag")]
            payments: Payments,
        }

        let helper = InfoHelper::deserialize(deserializer)?;

        if helper.versao != "4.00" {
            return Err(serde::de::Error::custom(format!(
                "Unsupported version: {}",
                helper.versao
            )));
        }

        let info = Info {
            identification: helper.identification,
            issuer: helper.issuer,
            details: helper.details,
            authorized: helper.authorized,
            total: helper.total,
            transport: helper.transport,
            payments: helper.payments,
        };
        if info.id() != helper.id {
            return Err(serde::de::Error::custom(format!(
                "ID mismatch: expected {}, found {}",
                info.id(),
                helper.id
            )));
        }

        Ok(info)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoNotMatchTotal {
    expected: f64,
    total: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InfoBuilderError {
    PaymentsDoNotMatchTotal(DoNotMatchTotal),
    ConfigError(ConfigError),
}

pub struct InfoBuilder {
    identification: Identification,
    issuer: Issuer,
    payments: Payments,
    details: Vec<Detail>,
    authorized: Option<Authorized>,
    transport: Option<Transport>,
}

impl InfoBuilder {
    fn new(identification: Identification, payments: Payments) -> Result<Self, InfoBuilderError> {
        let issuer = crate::config::get_issuer().map_err(InfoBuilderError::ConfigError)?;
        Ok(Self {
            identification,
            issuer,
            payments,
            details: Vec::new(),
            authorized: None,
            transport: None,
        })
    }

    pub fn add_detail(mut self, detail: Detail) -> Self {
        self.details.push(detail);
        self
    }

    pub fn set_authorized(mut self, authorized: Authorized) -> Self {
        self.authorized = Some(authorized);
        self
    }

    pub fn set_transport(mut self, transport: Transport) -> Self {
        self.transport = Some(transport);
        self
    }

    fn check_paid(&self, total: &Total) -> Result<(), InfoBuilderError> {
        let paid = self
            .payments
            .payments
            .iter()
            .fold(0.0f64, |acc, p| acc + p.value.as_ref());
        let expected = total.icms.total.as_ref();
        if (paid - expected).abs() < f64::EPSILON {
            Ok(())
        } else {
            Err(InfoBuilderError::PaymentsDoNotMatchTotal(DoNotMatchTotal {
                expected: *expected,
                total: paid,
            }))
        }
    }

    pub fn build(self) -> Result<Info, InfoBuilderError> {
        let total = Total::calculate(&self);
        self.check_paid(&total)?;

        let mut info = Info {
            identification: self.identification,
            issuer: self.issuer,
            details: self.details,
            authorized: self.authorized,
            payments: self.payments,
            total,
            transport: self.transport.unwrap_or_default(),
        };
        info.identification.verifier_digit = info.verifier_digit(&info.bare_id());
        Ok(info)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Payments {
    #[serde(rename = "detPag")]
    pub payments: Vec<Payment>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Payment {
    #[serde(rename = "tPag")]
    pub r#type: PaymentType,
    #[serde(rename = "vPag")]
    pub value: F64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "total")]
pub struct Total {
    #[serde(rename = "ICMSTot")]
    pub icms: TotalICMS,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TotalICMS {
    #[serde(rename = "vBC")]
    pub base: F64,
    #[serde(rename = "vICMS")]
    pub value: F64,
    #[serde(rename = "vICMSDeson")]
    pub unburdened: F64,
    #[serde(rename = "vFCP")]
    pub fcp_value: F64,
    #[serde(rename = "vBCST")]
    pub base_tributary_substitution: F64,
    #[serde(rename = "vST")]
    pub total_tributary_substitution: F64,
    #[serde(rename = "vFCPST")]
    pub fcp_value_tributary_substitution: F64,
    #[serde(rename = "vFCPSTRet")]
    pub retained_fcp_value_tributary_substitution: F64,
    #[serde(rename = "vProd")]
    pub total_products: F64,
    #[serde(rename = "vFrete")]
    pub freight: F64,
    #[serde(rename = "vSeg")]
    pub insurance: F64,
    #[serde(rename = "vDesc")]
    pub discount: F64,
    #[serde(rename = "vII")]
    pub import_tax: F64,
    #[serde(rename = "vIPI")]
    pub industrial_tax: F64,
    #[serde(rename = "vIPIDevol")]
    pub refunded_industrial_tax: F64,
    #[serde(rename = "vPIS")]
    pub pis_value: F64,
    #[serde(rename = "vCOFINS")]
    pub cofins_value: F64,
    #[serde(rename = "vOutro")]
    pub other: F64,
    #[serde(rename = "vNF")]
    pub total: F64,
}

impl Total {
    pub(crate) fn calculate(builder: &InfoBuilder) -> Self {
        let total_products = builder
            .details
            .iter()
            .fold(0.0f64, |acc, d| acc + d.item.total_value);
        let discount = builder
            .details
            .iter()
            .fold(0.0f64, |acc, d| acc + d.item.discount_value.unwrap_or(0.0));
        let unburdened = 0.0;
        let freight = 0.0;
        let insurance = 0.0;
        let other = builder
            .details
            .iter()
            .fold(0.0f64, |acc, d| acc + d.item.other_value.unwrap_or(0.0));
        let import_tax = 0.0;
        let industrial_tax = 0.0;
        let refunded_industrial_tax = 0.0;

        let total_value = total_products - discount - unburdened
            + freight
            + insurance
            + other
            + import_tax
            + industrial_tax
            + refunded_industrial_tax;

        Total {
            icms: TotalICMS {
                base: F64(0.0),
                value: F64(0.0),
                unburdened: F64(unburdened),
                fcp_value: F64(0.0),
                base_tributary_substitution: F64(0.0),
                total_tributary_substitution: F64(0.0),
                fcp_value_tributary_substitution: F64(0.0),
                retained_fcp_value_tributary_substitution: F64(0.0),
                total_products: F64(total_products),
                freight: F64(freight),
                insurance: F64(insurance),
                discount: F64(discount),
                import_tax: F64(import_tax),
                industrial_tax: F64(industrial_tax),
                refunded_industrial_tax: F64(refunded_industrial_tax),
                pis_value: F64(0.0),
                cofins_value: F64(0.0),
                other: F64(other),
                total: F64(total_value),
            },
        }
    }
}

/// Identification structure based on the XML structure of the NFe
///
/// location: Location of the issuer (cUF, cMun)
/// numeric_code: Numeric code of the NFe (cNF)
/// operation_nature: Nature of the operation (natOp)
/// model: Model of the NFe (mod)
/// series: Series of the NFe (serie)
/// number: Number of the NFe (nNF)
/// emission_date: Date and time of emission (dhEmi)
/// date: Date and time of exit or entry (dhSaiEnt) - Optional
/// type: Type of operation (tpNF)
/// destination: Destination target (idDest)
/// printing_type: Type of DANFE printing (tpImp) - Optional
/// emission_type: Type of emission (tpEmis)
/// verifier_digit: Verifier digit (cDV)
/// environment: Environment type (tpAmb)
/// finality: Finality of the NFe (finNFe)
/// consumer: Indicates if the operation is for a final consumer (indFinal)
/// presence: Presence indicator (indPres) - Optional
/// intermediator: Intermediator information (intermed) - Optional
/// emission_process: Emission process (procEmi) - Fixed value "0"
/// emission_version: Emission version (verProc) - Library version
#[derive(Debug, PartialEq)]
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
        state.serialize_field("dhEmi", &self.emission_date.to_rfc3339())?;
        if let Some(date) = &self.date {
            state.serialize_field("dhSaiEnt", &date.to_utc())?;
        }
        state.serialize_field("tpNF", &(self.r#type.clone() as u8))?;
        state.serialize_field("idDest", &(self.destination.clone() as u8))?;
        state.serialize_field("cMunFG", &self.location.city.code)?;
        state.serialize_field("xMun", &self.location.city.name)?;
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

impl<'de> Deserialize<'de> for Identification {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct IdentificationHelper {
            #[serde(rename = "cUF")]
            c_uf: u8,
            #[serde(rename = "cNF")]
            c_nf: u32,
            #[serde(rename = "natOp")]
            nat_op: String,
            #[serde(rename = "mod")]
            model: u8,
            #[serde(rename = "serie")]
            serie: u8,
            #[serde(rename = "nNF")]
            n_nf: u32,
            #[serde(rename = "dhEmi")]
            dh_emi: String,
            #[serde(rename = "dhSaiEnt")]
            dh_sai_ent: Option<String>,
            #[serde(rename = "tpNF")]
            tp_nf: u8,
            #[serde(rename = "idDest")]
            id_dest: u8,
            #[serde(rename = "cMunFG")]
            c_mun_fg: u32,
            #[serde(rename = "xMun")]
            x_mun: String,
            #[serde(rename = "tpImp")]
            tp_imp: Option<u8>,
            #[serde(rename = "tpEmis")]
            tp_emis: u8,
            #[serde(rename = "cDV")]
            c_dv: u8,
            #[serde(rename = "tpAmb")]
            tp_amb: u8,
            #[serde(rename = "finNFe")]
            fin_nfe: u8,
            #[serde(rename = "indFinal")]
            ind_final: u8,
            #[serde(rename = "indPres")]
            ind_pres: u8,
            #[serde(rename = "intermed")]
            intermed: Option<Intermediator>,
        }

        let helper = IdentificationHelper::deserialize(deserializer)?;
        let state = State::try_from(helper.c_uf).map_err(serde::de::Error::custom)?;
        let model = Model::try_from(helper.model).map_err(serde::de::Error::custom)?;
        let r#type = Operation::try_from(helper.tp_nf).map_err(serde::de::Error::custom)?;
        let destination =
            DestinationTarget::try_from(helper.id_dest).map_err(serde::de::Error::custom)?;
        let printing_type = match helper.tp_imp {
            Some(v) => Some(DanfeGeneration::try_from(v).map_err(serde::de::Error::custom)?),
            None => None,
        };
        let emission_type =
            EmissionType::try_from(helper.tp_emis).map_err(serde::de::Error::custom)?;
        let environment = Environment::try_from(helper.tp_amb).map_err(serde::de::Error::custom)?;
        let finality = Finality::try_from(helper.fin_nfe).map_err(serde::de::Error::custom)?;
        let consumer = helper.ind_final == 1;
        let presence = match helper.ind_pres {
            0 => None,
            1..=6 => Some(Presence::try_from(helper.ind_pres).map_err(serde::de::Error::custom)?),
            _ => return Err(serde::de::Error::custom("Invalid ind_pres value")),
        };
        let emission_date = chrono::DateTime::parse_from_rfc3339(&helper.dh_emi)
            .map_err(serde::de::Error::custom)?
            .with_timezone(&chrono::Local);
        let date = match helper.dh_sai_ent {
            Some(v) => Some(
                chrono::DateTime::parse_from_rfc3339(&v)
                    .map_err(serde::de::Error::custom)?
                    .with_timezone(&chrono::Local),
            ),
            None => None,
        };
        Ok(Identification {
            location: Location {
                state,
                city: City {
                    code: helper.c_mun_fg,
                    name: helper.x_mun,
                },
            },
            numeric_code: helper.c_nf,
            operation_nature: helper.nat_op,
            model,
            series: helper.serie,
            number: helper.n_nf,
            emission_date,
            date,
            r#type,
            destination,
            printing_type,
            emission_type,
            verifier_digit: helper.c_dv,
            environment,
            finality,
            consumer,
            presence,
            intermediator: helper.intermed,
        })
    }
}

/// Address structure based on the XML structure of the NFe
///
/// line_1: Address line 1 (xLgr)
/// line_2: Address line 2 (xCpl) - Optional
/// number: Address number (nro)
/// neighborhood: Neighborhood (xBairro)
/// city: City (cMun, xMun)
/// state: State (UF)
/// zip_code: ZIP code (CEP) - Only numbers
/// telephone: Telephone number (fone) - Only numbers
/// country_name: Country name (xPais) - Fixed value "Brasil"
/// country_code: Country code (cPais) - Fixed value 1058
#[derive(Debug, PartialEq, Clone)]
pub struct Address {
    pub line_1: String,
    pub line_2: Option<String>,
    pub number: String,
    pub neighborhood: String,
    pub city: City,
    pub state: State,
    pub zip_code: String,
    pub telephone: String,
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let len = 7 + self.line_2.is_some() as usize;
        let mut state = serializer.serialize_struct("enderEmit", len)?;
        state.serialize_field("xLgr", &self.line_1)?;
        if let Some(line_2) = &self.line_2 {
            state.serialize_field("xCpl", line_2)?;
        }
        state.serialize_field("nro", &self.number)?;
        state.serialize_field("xBairro", &self.neighborhood)?;
        state.serialize_field("cMun", &self.city.code)?;
        state.serialize_field("xMun", &self.city.name)?;
        state.serialize_field("UF", self.state.acronym())?;
        state.serialize_field("CEP", &self.zip_code)?;
        state.serialize_field("fone", &self.telephone)?;
        state.serialize_field("xPais", &"Brasil".to_string())?;
        state.serialize_field("cPais", &1058)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct AddressHelper {
            #[serde(rename = "xLgr")]
            x_lgr: String,
            #[serde(rename = "xCpl")]
            x_cpl: Option<String>,
            #[serde(rename = "nro")]
            nro: String,
            #[serde(rename = "xBairro")]
            x_bairro: String,
            #[serde(rename = "cMun")]
            c_mun: u32,
            #[serde(rename = "xMun")]
            x_mun: String,
            #[serde(rename = "UF")]
            uf: String,
            #[serde(rename = "CEP")]
            cep: String,
            #[serde(rename = "fone")]
            fone: String,
        }

        let helper = AddressHelper::deserialize(deserializer)?;
        let state = State::from_acronym(&helper.uf).ok_or_else(|| {
            serde::de::Error::custom(format!("Invalid state acronym: {}", helper.uf))
        })?;

        Ok(Address {
            line_1: helper.x_lgr,
            line_2: helper.x_cpl,
            number: helper.nro,
            neighborhood: helper.x_bairro,
            city: City {
                code: helper.c_mun,
                name: helper.x_mun,
            },
            state,
            zip_code: helper.cep,
            telephone: helper.fone,
        })
    }
}

/// Taxable entity identifier
///
/// address: Address of the taxable entity
/// ie: State registration (IE) - Use "ISENTO" if exempt
#[derive(Debug, PartialEq, Clone)]
pub struct TaxableAddress {
    pub address: Address,
    pub ie: IE,
}

impl Serialize for TaxableAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("enderEmit", 8)?;
        state.serialize_field("xLgr", &self.address.line_1)?;
        if let Some(line_2) = &self.address.line_2 {
            state.serialize_field("xCpl", line_2)?;
        }
        state.serialize_field("nro", &self.address.number)?;
        state.serialize_field("xBairro", &self.address.neighborhood)?;
        state.serialize_field("cMun", &self.address.city.code)?;
        state.serialize_field("xMun", &self.address.city.name)?;
        state.serialize_field("UF", self.address.state.acronym())?;
        state.serialize_field("CEP", &self.address.zip_code)?;
        state.serialize_field("fone", &self.address.telephone)?;
        state.serialize_field("xPais", &"Brasil".to_string())?;
        state.serialize_field("cPais", &1058)?;
        state.serialize_field("IE", &self.ie.0)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for TaxableAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TaxableAddressHelper {
            #[serde(rename = "xLgr")]
            x_lgr: String,
            #[serde(rename = "xCpl")]
            x_cpl: Option<String>,
            #[serde(rename = "nro")]
            nro: String,
            #[serde(rename = "xBairro")]
            x_bairro: String,
            #[serde(rename = "cMun")]
            c_mun: u32,
            #[serde(rename = "xMun")]
            x_mun: String,
            #[serde(rename = "UF")]
            uf: String,
            #[serde(rename = "CEP")]
            cep: String,
            #[serde(rename = "fone")]
            fone: String,
            #[serde(rename = "IE")]
            ie: String,
        }

        let helper = TaxableAddressHelper::deserialize(deserializer)?;
        let state = State::from_acronym(&helper.uf).ok_or_else(|| {
            serde::de::Error::custom(format!("Invalid state acronym: {}", helper.uf))
        })?;

        Ok(TaxableAddress {
            address: Address {
                line_1: helper.x_lgr,
                line_2: helper.x_cpl,
                number: helper.nro,
                neighborhood: helper.x_bairro,
                city: City {
                    code: helper.c_mun,
                    name: helper.x_mun,
                },
                state,
                zip_code: helper.cep,
                telephone: helper.fone,
            },
            ie: IE(helper.ie),
        })
    }
}

/// Issuer structure based on the XML structure of the NFe
///
/// document: Document (CNPJ, CPF, or IE)
/// name: Legal name of the issuer (xNome)
/// trade_name: Trade name of the issuer (xFant) - Optional
/// address: Taxable address of the issuer (enderEmit)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "emit")]
pub struct Issuer {
    #[serde(rename = "$value")]
    pub document: PersonDocument,
    #[serde(rename = "xNome")]
    pub name: String,
    #[serde(rename = "xFant")]
    pub trade_name: Option<String>,
    #[serde(rename = "enderEmit")]
    pub address: TaxableAddress,
}

/// Item structure based on the XML structure of the NFe
///
/// code: Product code (cProd)
/// gtin: Global Trade Item Number (cEAN) - Optional
/// description: Product description (xProd)
/// ncm: NCM code (Nomenclatura Comum do Mercosul)
/// cfop: CFOP code (Código Fiscal de Operações e Prestações)
/// unit: Unit of measurement (uCom)
/// quantity: Quantity of the product (qCom)
/// total_value: Total value of the product (vProd)
/// tribute_unit: Unit of measurement for tax purposes (uTrib)
/// tribute_quantity: Quantity for tax purposes (qTrib)
/// tribute_unit_value: Unit value for tax purposes (vUnTrib)
/// discount_value: Discount value (vDesc) - Optional
/// other_value: Other additional costs (vOutro) - Optional
/// included: Indicates if the item is included in the total invoice value (indTot)
#[derive(Debug, PartialEq)]
pub struct Item {
    pub code: String,
    pub gtin: Option<String>,
    pub description: String,
    pub ncm: u32,
    pub cfop: u32,
    pub unit: String,
    pub quantity: f64,
    pub total_value: f64,
    pub tribute_unit: String,
    pub tribute_quantity: f64,
    pub tribute_unit_value: f64,
    pub discount_value: Option<f64>,
    pub other_value: Option<f64>,
    pub included: bool,
}

impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let len = 12
            + self.gtin.is_some() as usize
            + self.discount_value.is_some() as usize
            + self.other_value.is_some() as usize;

        let no_gtin = &"SEM GTIN".to_string();
        let gtin = self.gtin.as_ref().unwrap_or(no_gtin);
        let mut state = serializer.serialize_struct("prod", len)?;
        state.serialize_field("cProd", &self.code)?;
        state.serialize_field("cEAN", gtin)?;
        state.serialize_field("xProd", &self.description)?;
        state.serialize_field("NCM", &self.ncm)?;
        state.serialize_field("CFOP", &self.cfop)?;
        state.serialize_field("uCom", &self.unit)?;
        state.serialize_field("qCom", &format!("{:.4}", self.quantity))?;
        state.serialize_field(
            "vUnCom",
            &format!("{:.2}", self.total_value / self.quantity),
        )?;
        state.serialize_field("vProd", &format!("{:.2}", self.total_value))?;
        state.serialize_field("cEANTrib", gtin)?;
        state.serialize_field("uTrib", &self.tribute_unit)?;
        state.serialize_field("qTrib", &format!("{:.4}", self.tribute_quantity))?;
        state.serialize_field("vUnTrib", &format!("{:.2}", self.tribute_unit_value))?;
        if let Some(discount_value) = &self.discount_value {
            state.serialize_field("vDesc", &format!("{:.4}", discount_value))?;
        }
        if let Some(other_value) = &self.other_value {
            state.serialize_field("vOutro", &format!("{:.4}", other_value))?;
        }
        state.serialize_field("indTot", if self.included { &1 } else { &0 })?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Item {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ItemHelper {
            #[serde(rename = "cProd")]
            c_prod: String,
            #[serde(rename = "cEAN")]
            c_ean: Option<String>,
            #[serde(rename = "xProd")]
            x_prod: String,
            #[serde(rename = "NCM")]
            ncm: u32,
            #[serde(rename = "CFOP")]
            cfop: u32,
            #[serde(rename = "uCom")]
            u_com: String,
            #[serde(rename = "qCom")]
            q_com: String,
            #[serde(rename = "vProd")]
            v_prod: String,
            #[serde(rename = "uTrib")]
            u_trib: String,
            #[serde(rename = "qTrib")]
            q_trib: String,
            #[serde(rename = "vUnTrib")]
            v_un_trib: String,
            #[serde(rename = "vDesc")]
            v_desc: Option<String>,
            #[serde(rename = "vOutro")]
            v_outro: Option<String>,
            #[serde(rename = "indTot")]
            ind_tot: u8,
        }

        let helper = ItemHelper::deserialize(deserializer)?;

        let quantity = helper
            .q_com
            .parse::<f64>()
            .map_err(serde::de::Error::custom)?;
        let total_value = helper
            .v_prod
            .parse::<f64>()
            .map_err(serde::de::Error::custom)?;
        let tribute_quantity = helper
            .q_trib
            .parse::<f64>()
            .map_err(serde::de::Error::custom)?;
        let tribute_unit_value = helper
            .v_un_trib
            .parse::<f64>()
            .map_err(serde::de::Error::custom)?;
        let discount_value = match helper.v_desc {
            Some(v) => Some(v.parse::<f64>().map_err(serde::de::Error::custom)?),
            None => None,
        };
        let other_value = match helper.v_outro {
            Some(v) => Some(v.parse::<f64>().map_err(serde::de::Error::custom)?),
            None => None,
        };
        let included = match helper.ind_tot {
            0 => false,
            1 => true,
            _ => return Err(serde::de::Error::custom("Invalid ind_tot value")),
        };

        Ok(Item {
            code: helper.c_prod,
            gtin: helper.c_ean,
            description: helper.x_prod,
            ncm: helper.ncm,
            cfop: helper.cfop,
            unit: helper.u_com,
            quantity,
            total_value,
            tribute_unit: helper.u_trib,
            tribute_quantity,
            tribute_unit_value,
            discount_value,
            other_value,
            included,
        })
    }
}

/// ICMS structure for CSOSN 102
///
/// origin: Origin of the product (orig)
/// csosn: CSOSN code (CSOSN)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ICMSSN102 {
    #[serde(rename = "orig")]
    pub origin: Origin,
    #[serde(rename = "CSOSN")]
    pub csosn: CSOSN,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "imposto")]
pub struct Tax {
    #[serde(rename = "ICMS")]
    pub icms: ICMS,
}

/// Detail structure based on the XML structure of the NFe
///
/// item: Item structure (prod)
/// tax: Tax structure (imposto)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "det")]
pub struct Detail {
    #[serde(rename = "prod")]
    pub item: Item,
    #[serde(rename = "imposto")]
    pub tax: Tax,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::config::{Config, PKCS12Config, set_config};
    use crate::utils::canonicalize_xml as canonicalize;
    use chrono::TimeZone;
    use nf_e_macros::serialization_test;
    use quick_xml::{de::from_str as deserialize, se::to_string as serialize};

    #[serialization_test(fixture = "../tests/fixtures/tax.xml")]
    fn setup_tax() -> Tax {
        Tax {
            icms: ICMS::ICMSSN102(ICMSSN102 {
                origin: Origin::National,
                csosn: CSOSN::FinalConsumer,
            }),
        }
    }

    #[serialization_test(fixture = "../tests/fixtures/item.xml")]
    fn setup_item() -> Item {
        Item {
            cfop: 5403,
            code: "7896235354499".to_string(),
            description: "desodorante aerosol monange 200ML".to_string(),
            ncm: 33072010,
            gtin: Some("7896235354499".to_string()),
            included: true,
            quantity: 3.0f64,
            total_value: 18.99f64 * 3.0f64,
            unit: "UN".to_string(),
            tribute_unit: "UN".to_string(),
            tribute_quantity: 3.0f64,
            tribute_unit_value: 18.99f64,
            discount_value: None,
            other_value: None,
        }
    }

    #[serialization_test(fixture = "../tests/fixtures/detail.xml")]
    fn setup_detail() -> Detail {
        Detail {
            tax: Tax {
                icms: ICMS::ICMSSN102(ICMSSN102 {
                    csosn: CSOSN::FinalConsumer,
                    origin: Origin::National,
                }),
            },
            item: setup_item(),
        }
    }

    fn setup_payments() -> Payments {
        Payments {
            payments: vec![
                Payment {
                    r#type: PaymentType::Cash,
                    value: F64(40.00),
                },
                Payment {
                    r#type: PaymentType::CreditCard,
                    value: F64(73.94),
                },
            ],
        }
    }

    fn setup_config() {
        if crate::config::is_set() {
            return;
        }

        set_config(Config::new(
            setup_issuer(),
            PKCS12Config::new(
                "tests/certificates/cert.pfx".to_string(),
                "12345678".to_string(),
            ),
        ))
        .expect("Failed to set config");
    }

    fn setup_info_builder() -> InfoBuilder {
        setup_config();

        InfoBuilder::new(setup_identification(), setup_payments())
            .unwrap()
            .add_detail(setup_detail())
            .add_detail(setup_detail())
    }

    #[serialization_test(fixture = "../tests/fixtures/info_authorized.xml")]
    fn setup_info() -> Info {
        setup_info_builder()
            .set_authorized(setup_authorized())
            .build()
            .expect("Failed to build Info")
    }

    #[test]
    fn serialize_info_without_authorized() {
        let info = setup_info_builder().build().expect("Failed to build Info");
        let serialized = quick_xml::se::to_string(&info).expect("Failed to serialize info");
        let canonicalized = canonicalize(&serialized).expect("Failed to canonicalize XML");
        assert_eq!(
            canonicalized,
            canonicalize(include_str!("../tests/fixtures/info.xml")).unwrap()
        );
    }

    #[serialization_test(fixture = "../tests/fixtures/identification.xml")]
    fn setup_identification() -> Identification {
        Identification {
            location: Location {
                state: State::MinasGerais,
                city: City {
                    code: 3106200,
                    name: "Belo Horizonte".to_string(),
                },
            },
            numeric_code: 12345678,
            operation_nature: "Venda de mercadoria".to_string(),
            model: Model::NFCe,
            series: 1,
            number: 12345,
            emission_date: chrono::Local
                .with_ymd_and_hms(2023, 10, 5, 14, 30, 0)
                .unwrap(),
            date: None,
            r#type: Operation::Outgoing,
            destination: DestinationTarget::Internal,
            printing_type: Some(DanfeGeneration::NFCe),
            emission_type: EmissionType::Normal,
            verifier_digit: 5,
            environment: Environment::Production,
            finality: Finality::Normal,
            consumer: true,
            presence: Some(Presence::InplaceIndoor),
            intermediator: None,
        }
    }

    #[serialization_test(fixture = "../tests/fixtures/address.xml")]
    fn setup_address() -> Address {
        Address {
            line_1: "Rua Exemplo".to_string(),
            line_2: Some("Loja 1".to_string()),
            number: "123".to_string(),
            neighborhood: "Centro".to_string(),
            city: City {
                code: 3106200,
                name: "Belo Horizonte".to_string(),
            },
            state: State::MinasGerais,
            zip_code: "01001000".to_string(),
            telephone: "3132123456".to_string(),
        }
    }

    #[serialization_test(fixture = "../tests/fixtures/issuer.xml")]
    pub fn setup_issuer() -> Issuer {
        Issuer {
            document: PersonDocument::CNPJ(CNPJ("12345678000195".to_string())),
            name: "Empresa Exemplo LTDA".to_string(),
            trade_name: Some("Empresa Exemplo".to_string()),
            address: TaxableAddress {
                address: setup_address(),
                ie: IE("123456789".to_string()),
            },
        }
    }

    #[serialization_test(fixture = "../tests/fixtures/authorized.xml")]
    fn setup_authorized() -> Authorized {
        Authorized {
            documents: vec![
                PersonDocument::CNPJ(CNPJ("12345678000195".to_string())),
                PersonDocument::CPF(CPF("12345678901".to_string())),
            ],
        }
    }

    #[serialization_test(fixture = "../tests/fixtures/nfe.xml")]
    fn setup_nfe() -> NFe {
        NFe::new(setup_info())
    }

    #[serialization_test(fixture = "../tests/fixtures/total.xml")]
    fn setup_total() -> Total {
        Total::calculate(&setup_info_builder())
    }

    #[serialization_test(fixture = "../tests/fixtures/transport.xml")]
    fn setup_transport() -> Transport {
        Transport::default()
    }
}
