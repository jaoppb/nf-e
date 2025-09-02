use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct City {
    pub code: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub(crate) fn acronym(&self) -> &str {
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
}
