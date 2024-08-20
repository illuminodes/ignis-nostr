use super::valuesets::{
    FhirAddressType, FhirAddressUse, FhirContactPointSystem, FhirContactPointUse, FhirNameUse,
};
use serde::{Deserialize, Serialize};

// https://www.hl7.org/fhir/datatypes.html#Period
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FhirPeriod {
    pub start: FhirDateTime,
    pub end: FhirDateTime,
}
// https://www.hl7.org/fhir/datatypes.html#instant
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FhirInstant(pub String);
// https://www.hl7.org/fhir/datatypes.html#dateTime
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FhirDateTime(String);
// https://www.hl7.org/fhir/datatypes.html#HumanName
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FhirHumanName {
    #[serde(rename = "use")]
    pub name_use: FhirNameUse,
    pub text: String,
    pub family: String,
    pub given: Vec<String>,
    pub prefix: Vec<String>,
    pub suffix: Vec<String>,
    pub period: Option<FhirPeriod>,
}
impl FhirHumanName {
    pub fn new_simple(text: String) -> Self {
        let family = text.clone().split(' ').collect();
        Self {
            name_use: FhirNameUse::Official,
            text,
            family,
            given: vec![],
            prefix: vec![],
            suffix: vec![],
            period: None,
        }
    }
}
// https://www.hl7.org/fhir/datatypes.html#ContactPoint
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FhirContactPoint {
    pub system: FhirContactPointSystem,
    pub value: String,
    #[serde(rename = "use")]
    pub contact_use: FhirContactPointUse,
    pub period: Option<FhirPeriod>,
    pub rank: Option<u32>,
}
impl FhirContactPoint {
    pub fn new_email(value: String) -> Self {
        Self {
            system: FhirContactPointSystem::Email,
            value,
            contact_use: FhirContactPointUse::Mobile,
            period: None,
            rank: None,
        }
    }
    pub fn new_phone(value: String) -> Self {
        Self {
            system: FhirContactPointSystem::Phone,
            value,
            contact_use: FhirContactPointUse::Mobile,
            period: None,
            rank: None,
        }
    }
}
// https://www.hl7.org/fhir/datatypes.html#Address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirAddress {
    #[serde(rename = "use")]
    pub address_use: FhirAddressUse,
    #[serde(rename = "type")]
    pub address_type: FhirAddressType,
    pub text: String,
    pub line: Vec<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub period: Option<FhirPeriod>,
}
