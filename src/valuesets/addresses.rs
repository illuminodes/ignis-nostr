use serde::{Deserialize, Serialize};
// https://www.hl7.org/fhir/valueset-address-type.html
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FhirAddressType {
    Postal,
    Physical,
    Both,
}
// https://www.hl7.org/fhir/valueset-address-use.html
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FhirAddressUse {
    Home,
    Work,
    Temporary,
    Old,
    Billing,
}
