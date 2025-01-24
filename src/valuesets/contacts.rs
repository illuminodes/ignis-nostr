use serde::{Deserialize, Serialize};
// https://www.hl7.org/fhir/valueset-contact-point-use.html
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum FhirContactPointUse {
    Home,
    Work,
    Temp,
    Old,
    Mobile,
}
// https://www.hl7.org/fhir/valueset-contact-point-system.html
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum FhirContactPointSystem {
    Phone,
    Fax,
    Email,
    Pager,
    Url,
    Sms,
    Other,
}
// https://terminology.hl7.org/5.1.0/ValueSet-contactentity-type.html
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FhirContactEntityType {
    Billing,
    Administrative,
    HumanResource,
    Payor,
    Patient,
    Press,
}
