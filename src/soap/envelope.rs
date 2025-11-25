use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};

/// SOAP Envelope namespace
pub const SOAP_ENVELOPE_NS: &str = "http://www.w3.org/2003/05/soap-envelope";

/// NFe namespace
pub const NFE_NS: &str = "http://www.portalfiscal.inf.br/nfe/wsdl/NFeAutorizacao4";

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
            xmlns_soap: Option<String>,
            #[serde(rename = "soap:Body", alias = "Body")]
            body: SoapBody<T>,
        }

        let helper = SoapEnvelopeHelper::deserialize(deserializer)?;
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
    use quick_xml::{de::from_str as deserialize, se::to_string as serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestContent {
        #[serde(rename = "value")]
        value: String,
    }

    #[test]
    fn test_soap_envelope_serialization() {
        let envelope = SoapEnvelope::new(TestContent {
            value: "test".to_string(),
        });
        let xml = serialize(&envelope).expect("Failed to serialize SOAP envelope");
        assert!(xml.contains("soap:Envelope"));
        assert!(xml.contains("soap:Body"));
        assert!(xml.contains("<value>test</value>"));
    }

    #[test]
    fn test_soap_envelope_deserialization() {
        let xml = r#"<soap:Envelope xmlns:soap="http://www.w3.org/2003/05/soap-envelope"><soap:Body><value>test</value></soap:Body></soap:Envelope>"#;
        let envelope: SoapEnvelope<TestContent> =
            deserialize(xml).expect("Failed to deserialize SOAP envelope");
        assert_eq!(envelope.body.content.value, "test");
    }
}
