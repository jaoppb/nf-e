use serde::{Deserialize, Serialize};

/// NFe Web Service version
pub const NFE_VERSION: &str = "4.00";

/// NFe WSDL namespaces
pub mod namespaces {
    pub const NFE_AUTORIZACAO: &str = "http://www.portalfiscal.inf.br/nfe/wsdl/NFeAutorizacao4";
    pub const NFE_RET_AUTORIZACAO: &str =
        "http://www.portalfiscal.inf.br/nfe/wsdl/NFeRetAutorizacao4";
    pub const NFE_CONSULTA_PROTOCOLO: &str =
        "http://www.portalfiscal.inf.br/nfe/wsdl/NFeConsultaProtocolo4";
    pub const NFE_INUTILIZACAO: &str = "http://www.portalfiscal.inf.br/nfe/wsdl/NFeInutilizacao4";
    pub const NFE_STATUS_SERVICO: &str =
        "http://www.portalfiscal.inf.br/nfe/wsdl/NFeStatusServico4";
    pub const NFE_DATA: &str = "http://www.portalfiscal.inf.br/nfe";
}

/// Service type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceType {
    /// NFe Authorization (sending NFe batch)
    NfeAutorizacao,
    /// NFe Return Authorization (query batch result)
    NfeRetAutorizacao,
    /// NFe Protocol Query (query by access key)
    NfeConsultaProtocolo,
    /// NFe Inutilization (cancel number range)
    NfeInutilizacao,
    /// NFe Service Status
    NfeStatusServico,
}

impl ServiceType {
    /// Get the WSDL namespace for this service type
    pub fn namespace(&self) -> &'static str {
        match self {
            ServiceType::NfeAutorizacao => namespaces::NFE_AUTORIZACAO,
            ServiceType::NfeRetAutorizacao => namespaces::NFE_RET_AUTORIZACAO,
            ServiceType::NfeConsultaProtocolo => namespaces::NFE_CONSULTA_PROTOCOLO,
            ServiceType::NfeInutilizacao => namespaces::NFE_INUTILIZACAO,
            ServiceType::NfeStatusServico => namespaces::NFE_STATUS_SERVICO,
        }
    }

    /// Get the SOAP action for this service type
    pub fn soap_action(&self) -> &'static str {
        match self {
            ServiceType::NfeAutorizacao => "http://www.portalfiscal.inf.br/nfe/wsdl/NFeAutorizacao4/nfeAutorizacaoLote",
            ServiceType::NfeRetAutorizacao => "http://www.portalfiscal.inf.br/nfe/wsdl/NFeRetAutorizacao4/nfeRetAutorizacaoLote",
            ServiceType::NfeConsultaProtocolo => "http://www.portalfiscal.inf.br/nfe/wsdl/NFeConsultaProtocolo4/nfeConsultaNF",
            ServiceType::NfeInutilizacao => "http://www.portalfiscal.inf.br/nfe/wsdl/NFeInutilizacao4/nfeInutilizacaoNF",
            ServiceType::NfeStatusServico => "http://www.portalfiscal.inf.br/nfe/wsdl/NFeStatusServico4/nfeStatusServicoNF",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_type_namespace() {
        assert_eq!(
            ServiceType::NfeAutorizacao.namespace(),
            namespaces::NFE_AUTORIZACAO
        );
        assert_eq!(
            ServiceType::NfeStatusServico.namespace(),
            namespaces::NFE_STATUS_SERVICO
        );
    }
}
