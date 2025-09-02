use crate::enums::*;

use crate::states::{City, Location, State};
use crate::LIBRARY_VERSION;
use serde::ser::SerializeSeq;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

/// Main structure based on the XML structure of the NFe
///
/// id: Identifier of the NFe (id) - Format "NFe{chave}"
/// identification: Identification structure (ide)
/// issuer: Issuer structure (emit)
/// details: Details structure (det)
/// version: Fixed value "4.00" (@versao)
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

/// Address structure based on the XML structure of the NFe
///
/// line_1: Address line 1 (xLgr)
/// line_2: Address line 2 (xCpl) - Optional
/// number: Address number (nro)
/// neighborhood: Neighborhood (xBairro)
/// city: City (cMun, xMun)
/// state: State (UF)
/// zip_code: ZIP code (CEP) - Only numbers
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

/// Taxable entity identifier
///
/// address: Address of the taxable entity
/// ie: State registration (IE) - Use "ISENTO" if exempt
#[derive(Serialize, Deserialize, Debug)]
pub struct TaxableAddress {
    pub address: Address,
    pub ie: IE,
}

/// Issuer structure based on the XML structure of the NFe
///
/// document: Document (CNPJ, CPF, or IE)
/// name: Legal name of the issuer (xNome)
/// trade_name: Trade name of the issuer (xFant) - Optional
/// address: Taxable address of the issuer (enderEmit)
#[derive(Serialize, Deserialize, Debug)]
pub struct Issuer {
    pub document: Document,
    pub name: String,
    pub trade_name: Option<String>,
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
#[derive(Deserialize, Debug)]
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

/// ICMS structure for CSOSN 102
///
/// origin: Origin of the product (orig)
/// csosn: CSOSN code (CSOSN)
#[derive(Serialize, Deserialize, Debug)]
pub struct ICMSSN102 {
    #[serde(rename = "orig")]
    pub origin: Origin,
    #[serde(rename = "CSOSN")]
    pub csosn: CSOSN,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tax {
    #[serde(rename = "ICMS")]
    pub icms: ICMS,
}

/// Detail structure based on the XML structure of the NFe
///
/// item: Item structure (prod)
/// icms: ICMS structure (imposto->ICMS)
#[derive(Deserialize, Debug)]
pub struct Detail {
    pub item: Item,
    pub tax: Tax,
}

impl Serialize for Detail {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("det", 2)?;
        state.serialize_field("prod", &self.item)?;
        state.serialize_field("imposto", &self.tax)?;
        state.end()
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::canonicalize_str;
    use chrono::TimeZone;

    #[test]
    fn serialize_icms() {
        let icms = ICMS::ICMSSN102(ICMSSN102 {
            origin: Origin::National,
            csosn: CSOSN::FinalConsumer,
        });

        let serialized = quick_xml::se::to_string(&icms);

        match serialized {
            Ok(xml) => assert_eq!(
                canonicalize_str(&xml).unwrap(),
                "<ICMS><ICMSSN102><orig>0</orig><CSOSN>102</CSOSN></ICMSSN102></ICMS>"
            ),
            Err(e) => panic!("Failed to serialize ICMS {}", e.to_string()),
        }
    }

    #[test]
    fn serialize_detail() {
        let detail = Detail {
            tax: Tax {
                icms: ICMS::ICMSSN102(ICMSSN102 {
                    csosn: CSOSN::FinalConsumer,
                    origin: Origin::National,
                }),
            },
            item: Item {
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
            },
        };

        let serialized = quick_xml::se::to_string(&detail);

        match serialized {
            Ok(xml) => {
                let canonicalized = canonicalize_str(&xml).unwrap();
                assert_eq!(
                    canonicalized,
                    include_str!("../../tests/fixtures/detail.xml")
                );
            }
            Err(e) => panic!("Failed to serialize detail {}", e.to_string()),
        }
    }

    #[test]
    fn serialize_identification() {
        let identification = Identification {
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
            emission_date: chrono::Local.with_ymd_and_hms(2023, 10, 5, 14, 30, 0).unwrap(),
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
        };

        let serialized = quick_xml::se::to_string(&identification);

        match serialized {
            Ok(xml) => {
                let canonicalized = canonicalize_str(&xml).unwrap();
                assert_eq!(
                    canonicalized,
                    include_str!("../../tests/fixtures/identification.xml")
                );
            }
            Err(e) => panic!("Failed to serialize identification {}", e.to_string()),
        }
    }
}
