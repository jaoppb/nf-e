use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};

/// SOAP Envelope namespace
pub const SOAP_ENVELOPE_NS: &str = "http://www.w3.org/2003/05/soap-envelope";

/// SOAP Envelope structure for NFe requests
#[derive(Debug, PartialEq)]
pub struct SoapEnvelope<T> {
    pub body: SoapBody<T>,
}

impl<T: Serialize> Serialize for SoapEnvelope<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("soap:Envelope", 2)?;
        state.serialize_field("@xmlns:soap", SOAP_ENVELOPE_NS)?;
        state.serialize_field("soap:Body", &self.body)?;
        state.end()
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for SoapEnvelope<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct SoapEnvelopeHelper<T> {
            #[serde(rename = "@xmlns:soap")]
            xmlns_soap: String,
            #[serde(rename = "soap:Body", alias = "Body")]
            body: SoapBody<T>,
        }

        let helper = SoapEnvelopeHelper::deserialize(deserializer)?;

        if helper.xmlns_soap != SOAP_ENVELOPE_NS {
            return Err(serde::de::Error::custom(format!(
                "Invalid SOAP namespace: expected '{}', found '{}'",
                SOAP_ENVELOPE_NS, helper.xmlns_soap
            )));
        }

        Ok(SoapEnvelope { body: helper.body })
    }
}

impl<T> SoapEnvelope<T> {
    pub fn new(content: T) -> Self {
        SoapEnvelope {
            body: SoapBody { content },
        }
    }
}

/// SOAP Body structure
#[derive(Debug, PartialEq)]
pub struct SoapBody<T> {
    pub content: T,
}

impl<T: Serialize> Serialize for SoapBody<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("soap:Body", 1)?;
        state.serialize_field("$value", &self.content)?;
        state.end()
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for SoapBody<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct SoapBodyHelper<T> {
            #[serde(rename = "$value")]
            content: T,
        }

        let helper = SoapBodyHelper::deserialize(deserializer)?;
        Ok(SoapBody {
            content: helper.content,
        })
    }
}

/// SOAP Fault structure for error responses
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SoapFault {
    #[serde(rename = "faultcode")]
    pub fault_code: String,
    #[serde(rename = "faultstring")]
    pub fault_string: String,
    #[serde(rename = "detail")]
    pub detail: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::canonicalize_xml as canonicalize;
    use nf_e_macros::serialization_test;
    use quick_xml::{de::from_str as deserialize, se::to_string as serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct TestContent {
        #[serde(rename = "value")]
        value: String,
    }

    #[serialization_test(fixture = "../../tests/fixtures/soap/envelope.xml")]
    fn setup_soap_envelope() -> SoapEnvelope<TestContent> {
        SoapEnvelope::new(TestContent {
            value: "test".to_string(),
        })
    }

    #[test]
    fn test_soap_envelope_invalid_namespace() {
        let xml = r#"<soap:Envelope xmlns:soap="http://invalid.namespace"><soap:Body><value>test</value></soap:Body></soap:Envelope>"#;
        let result: Result<SoapEnvelope<TestContent>, _> = deserialize(xml);
        assert!(result.is_err());
    }
}
