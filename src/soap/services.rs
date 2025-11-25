use crate::enums::Environment;
use crate::states::State;
use serde::{Deserialize, Serialize};

/// NFe Web Service version
pub const NFE_VERSION: &str = "4.00";

/// Service URLs for NFe web services by environment and state
#[derive(Debug, Clone)]
pub struct ServiceUrls {
    pub nfe_autorizacao: &'static str,
    pub nfe_ret_autorizacao: &'static str,
    pub nfe_consulta_protocolo: &'static str,
    pub nfe_inutilizacao: &'static str,
    pub nfe_status_servico: &'static str,
}

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

/// Production URLs for SVRS (Virtual Environment of Rio Grande do Sul)
pub mod svrs_production {
    use super::ServiceUrls;

    pub const URLS: ServiceUrls = ServiceUrls {
        nfe_autorizacao: "https://nfe.svrs.rs.gov.br/ws/NfeAutorizacao/NFeAutorizacao4.asmx",
        nfe_ret_autorizacao:
            "https://nfe.svrs.rs.gov.br/ws/NfeRetAutorizacao/NFeRetAutorizacao4.asmx",
        nfe_consulta_protocolo:
            "https://nfe.svrs.rs.gov.br/ws/NfeConsulta/NfeConsulta4.asmx",
        nfe_inutilizacao:
            "https://nfe.svrs.rs.gov.br/ws/NfeInutilizacao/NfeInutilizacao4.asmx",
        nfe_status_servico:
            "https://nfe.svrs.rs.gov.br/ws/NfeStatusServico/NfeStatusServico4.asmx",
    };
}

/// Homologation URLs for SVRS (Virtual Environment of Rio Grande do Sul)
pub mod svrs_homologation {
    use super::ServiceUrls;

    pub const URLS: ServiceUrls = ServiceUrls {
        nfe_autorizacao:
            "https://nfe-homologacao.svrs.rs.gov.br/ws/NfeAutorizacao/NFeAutorizacao4.asmx",
        nfe_ret_autorizacao:
            "https://nfe-homologacao.svrs.rs.gov.br/ws/NfeRetAutorizacao/NFeRetAutorizacao4.asmx",
        nfe_consulta_protocolo:
            "https://nfe-homologacao.svrs.rs.gov.br/ws/NfeConsulta/NfeConsulta4.asmx",
        nfe_inutilizacao:
            "https://nfe-homologacao.svrs.rs.gov.br/ws/NfeInutilizacao/NfeInutilizacao4.asmx",
        nfe_status_servico:
            "https://nfe-homologacao.svrs.rs.gov.br/ws/NfeStatusServico/NfeStatusServico4.asmx",
    };
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

/// Get service URLs for a given state and environment
/// Note: This uses SVRS as the default for states that use the virtual environment
pub fn get_service_urls(_state: &State, environment: &Environment) -> ServiceUrls {
    // Most states use SVRS, some use their own SEFAZ
    // For simplicity, we return SVRS URLs here. This can be extended
    // to support state-specific URLs.
    match environment {
        Environment::Production => svrs_production::URLS.clone(),
        Environment::Homologation => svrs_homologation::URLS.clone(),
    }
}

/// Get the URL for a specific service
pub fn get_service_url(
    state: &State,
    environment: &Environment,
    service_type: ServiceType,
) -> &'static str {
    let urls = get_service_urls(state, environment);
    match service_type {
        ServiceType::NfeAutorizacao => urls.nfe_autorizacao,
        ServiceType::NfeRetAutorizacao => urls.nfe_ret_autorizacao,
        ServiceType::NfeConsultaProtocolo => urls.nfe_consulta_protocolo,
        ServiceType::NfeInutilizacao => urls.nfe_inutilizacao,
        ServiceType::NfeStatusServico => urls.nfe_status_servico,
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

    #[test]
    fn test_get_service_urls_production() {
        let urls = get_service_urls(&State::MinasGerais, &Environment::Production);
        assert!(urls.nfe_autorizacao.contains("nfe.svrs.rs.gov.br"));
    }

    #[test]
    fn test_get_service_urls_homologation() {
        let urls = get_service_urls(&State::MinasGerais, &Environment::Homologation);
        assert!(urls.nfe_autorizacao.contains("nfe-homologacao.svrs.rs.gov.br"));
    }
}
