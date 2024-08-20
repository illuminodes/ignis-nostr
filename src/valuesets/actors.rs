use serde::{Deserialize, Serialize};

// https://www.hl7.org/fhir/valueset-name-use.html
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum FhirNameUse {
    Usual,
    Official,
    Temp,
    Nickname,
    Anonymous,
    Old,
    Maiden,
}
// https://www.hl7.org/fhir/valueset-organization-type.html
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FhirOrganizationType {
    Provider,
    Department,
    Team,
    Government,
    InsuranceCompany,
    Payer,
    EducationalInstitute,
    ReligiousInstitution,
    ClinicalResearchSponsor,
    CommunityGroup,
    NonHealthcareBusiness,
    Other,
}
