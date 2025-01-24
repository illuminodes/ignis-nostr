use crate::{
    consts::{FHIR_RESOURCE_PRACTITIONER, NOSTR_KIND_PRACTITIONER},
    datatypes::{FhirContactPoint, FhirHumanName},
    fhir_trait::{FhirResource, FhirText},
    valuesets::FhirContactPointSystem,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FhirPractitioner {
    name: FhirHumanName,
    #[serde(rename = "type")]
    contact: Vec<FhirContactPoint>,
    extension: Option<Value>,
}
impl FhirPractitioner {
    pub fn new(name: FhirHumanName, contact: Vec<FhirContactPoint>) -> Self {
        Self {
            name,
            contact,
            extension: None,
        }
    }
    pub fn text_name(&self) -> String {
        self.name.text.to_string()
    }
    pub fn get_email(&self) -> Option<&FhirContactPoint> {
        self.contact
            .iter()
            .find(|contact| contact.system == FhirContactPointSystem::Email)
    }
    pub fn get_phone(&self) -> Option<&FhirContactPoint> {
        self.contact
            .iter()
            .find(|contact| contact.system == FhirContactPointSystem::Phone)
    }
    pub fn get_extension<T>(&self) -> Option<T>
    where
        T: for<'de> DeserializeOwned,
    {
        let value = self.extension.as_ref().unwrap();
        Some(serde_json::from_value(value.clone()).unwrap())
    }
    pub fn extend(&mut self, extension: Value) {
        self.extension = Some(extension);
    }
}
impl FhirResource for FhirPractitioner {
    fn nostr_kind(&self) -> u32 {
        NOSTR_KIND_PRACTITIONER
    }
    fn resource_type(&self) -> &str {
        FHIR_RESOURCE_PRACTITIONER
    }
    fn text(&self) -> FhirText {
        FhirText::new(self.name.text.clone())
    }
}
