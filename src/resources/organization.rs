use crate::{
    consts::{FHIR_RESOURCE_ORGANIZATION, NOSTR_KIND_ORGANIZATION},
    fhir_trait::{FhirResource, FhirText},
    metadata::FhirExtendedContactDetail,
    valuesets::FhirOrganizationType,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirOrganization {
    name: String,
    #[serde(rename = "type")]
    org_type: FhirOrganizationType,
    contact: Vec<FhirExtendedContactDetail>,
    description: String,
}
impl FhirOrganization {
    pub fn new(
        name: String,
        org_type: FhirOrganizationType,
        contact: Vec<FhirExtendedContactDetail>,
        description: String,
    ) -> Self {
        Self {
            name,
            org_type,
            contact,
            description,
        }
    }
}
impl FhirResource for FhirOrganization {
    fn nostr_kind(&self) -> u32 {
        NOSTR_KIND_ORGANIZATION
    }
    fn resource_type(&self) -> &str {
        FHIR_RESOURCE_ORGANIZATION
    }
    fn text(&self) -> FhirText {
        FhirText::new(self.name.clone())
    }
}
