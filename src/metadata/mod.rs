
use serde::{Deserialize, Serialize};

use super::{datatypes::{FhirContactPoint, FhirAddress, FhirPeriod}, valuesets::FhirContactEntityType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirExtendedContactDetail {
    pub purpose: FhirContactEntityType,
    pub name: Option<String>,
    pub telecom: Vec<FhirContactPoint>,
    pub address: Option<FhirAddress>,
    pub period: Option<FhirPeriod>,
}
