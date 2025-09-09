use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct City {
    pub code: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum State {
    Rondonia = 11,
    Acre = 12,
    Amazonas = 13,
    Roraima = 14,
    Para = 15,
    Amapa = 16,
    Tocantins = 17,
    Maranhao = 21,
    Piaui = 22,
    Ceara = 23,
    RioGrandeDoNorte = 24,
    Paraiba = 25,
    Pernambuco = 26,
    Alagoas = 27,
    Sergipe = 28,
    Bahia = 29,
    MinasGerais = 31,
    EspiritoSanto = 32,
    RioDeJaneiro = 33,
    SaoPaulo = 35,
    Parana = 41,
    SantaCatarina = 42,
    RioGrandeDoSul = 43,
    MatoGrossoDoSul = 50,
    MatoGrosso = 51,
    Goias = 52,
    DistritoFederal = 53,
}

impl State {
    pub fn from_acronym(acronym: &str) -> Option<Self> {
        match acronym {
            "RO" => Some(State::Rondonia),
            "AC" => Some(State::Acre),
            "AM" => Some(State::Amazonas),
            "RR" => Some(State::Roraima),
            "PA" => Some(State::Para),
            "AP" => Some(State::Amapa),
            "TO" => Some(State::Tocantins),
            "MA" => Some(State::Maranhao),
            "PI" => Some(State::Piaui),
            "CE" => Some(State::Ceara),
            "RN" => Some(State::RioGrandeDoNorte),
            "PB" => Some(State::Paraiba),
            "PE" => Some(State::Pernambuco),
            "AL" => Some(State::Alagoas),
            "SE" => Some(State::Sergipe),
            "BA" => Some(State::Bahia),
            "MG" => Some(State::MinasGerais),
            "ES" => Some(State::EspiritoSanto),
            "RJ" => Some(State::RioDeJaneiro),
            "SP" => Some(State::SaoPaulo),
            "PR" => Some(State::Parana),
            "SC" => Some(State::SantaCatarina),
            "RS" => Some(State::RioGrandeDoSul),
            "MS" => Some(State::MatoGrossoDoSul),
            "MT" => Some(State::MatoGrosso),
            "GO" => Some(State::Goias),
            "DF" => Some(State::DistritoFederal),
            _ => None,
        }
    }
}

impl TryFrom<u8> for State {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            11 => Ok(State::Rondonia),
            12 => Ok(State::Acre),
            13 => Ok(State::Amazonas),
            14 => Ok(State::Roraima),
            15 => Ok(State::Para),
            16 => Ok(State::Amapa),
            17 => Ok(State::Tocantins),
            21 => Ok(State::Maranhao),
            22 => Ok(State::Piaui),
            23 => Ok(State::Ceara),
            24 => Ok(State::RioGrandeDoNorte),
            25 => Ok(State::Paraiba),
            26 => Ok(State::Pernambuco),
            27 => Ok(State::Alagoas),
            28 => Ok(State::Sergipe),
            29 => Ok(State::Bahia),
            31 => Ok(State::MinasGerais),
            32 => Ok(State::EspiritoSanto),
            33 => Ok(State::RioDeJaneiro),
            35 => Ok(State::SaoPaulo),
            41 => Ok(State::Parana),
            42 => Ok(State::SantaCatarina),
            43 => Ok(State::RioGrandeDoSul),
            50 => Ok(State::MatoGrossoDoSul),
            51 => Ok(State::MatoGrosso),
            52 => Ok(State::Goias),
            53 => Ok(State::DistritoFederal),
            _ => Err(format!("Invalid state code: {}", value)),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Location {
    pub state: State,
    pub city: City,
}

impl State {
    pub fn name(&self) -> &str {
        match self {
            State::Rondonia => "Rondônia",
            State::Acre => "Acre",
            State::Amazonas => "Amazonas",
            State::Roraima => "Roraima",
            State::Para => "Pará",
            State::Amapa => "Amapá",
            State::Tocantins => "Tocantins",
            State::Maranhao => "Maranhão",
            State::Piaui => "Piauí",
            State::Ceara => "Ceará",
            State::RioGrandeDoNorte => "Rio Grande do Norte",
            State::Paraiba => "Paraíba",
            State::Pernambuco => "Pernambuco",
            State::Alagoas => "Alagoas",
            State::Sergipe => "Sergipe",
            State::Bahia => "Bahia",
            State::MinasGerais => "Minas Gerais",
            State::EspiritoSanto => "Espírito Santo",
            State::RioDeJaneiro => "Rio de Janeiro",
            State::SaoPaulo => "São Paulo",
            State::Parana => "Paraná",
            State::SantaCatarina => "Santa Catarina",
            State::RioGrandeDoSul => "Rio Grande do Sul",
            State::MatoGrossoDoSul => "Mato Grosso do Sul",
            State::MatoGrosso => "Mato Grosso",
            State::Goias => "Goiás",
            State::DistritoFederal => "Distrito Federal",
        }
    }

    pub fn acronym(&self) -> &str {
        match self {
            State::Rondonia => "RO",
            State::Acre => "AC",
            State::Amazonas => "AM",
            State::Roraima => "RR",
            State::Para => "PA",
            State::Amapa => "AP",
            State::Tocantins => "TO",
            State::Maranhao => "MA",
            State::Piaui => "PI",
            State::Ceara => "CE",
            State::RioGrandeDoNorte => "RN",
            State::Paraiba => "PB",
            State::Pernambuco => "PE",
            State::Alagoas => "AL",
            State::Sergipe => "SE",
            State::Bahia => "BA",
            State::MinasGerais => "MG",
            State::EspiritoSanto => "ES",
            State::RioDeJaneiro => "RJ",
            State::SaoPaulo => "SP",
            State::Parana => "PR",
            State::SantaCatarina => "SC",
            State::RioGrandeDoSul => "RS",
            State::MatoGrossoDoSul => "MS",
            State::MatoGrosso => "MT",
            State::Goias => "GO",
            State::DistritoFederal => "DF",
        }
    }

    pub fn code(&self) -> u8 {
        self.clone() as u8
    }
}
