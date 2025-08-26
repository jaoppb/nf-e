use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Model {
    NFe = 55,
    NFCe = 65,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Operation {
    Incoming = 0,
    Outgoing = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DestinationTarget {
    Internal = 1,
    Interstate = 2,
    External = 3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DanfeGeneration {
    NormalPortrait = 1,
    NormalLandscape = 2,
    Simplified = 3,
    NFCe = 4,
    NFCeVirtual = 5,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EmissionType {
    Normal = 1,
    FSIA = 2,
    EPEC = 4,
    FSDA = 5,
    SVCAN = 6,
    SVCRS = 7,
    Offline = 9,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Environment {
    Production = 1,
    Homologation = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Finality {
    Normal = 1,
    Complementary = 2,
    Adjustment = 3,
    Cancellation = 4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Presence {
    InplaceIndoor = 1,
    InplaceOutdoor = 5,
    Internet = 2,
    Teleservice = 3,
    Delivery = 4,
    Other = 9,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Intermediator {
    External = 1,
}
