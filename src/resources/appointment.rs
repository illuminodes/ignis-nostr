use crate::{
    consts::{FHIR_RESOURCE_APPOINTMENT, NOSTR_KIND_APPOINTMENT},
    datatypes::FhirInstant,
    fhir_trait::{FhirResource, FhirText},
    valuesets::{FhirAppointmentStatus, FhirSpecialty},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FhirAppointment<T> {
    status: FhirAppointmentStatus,
    #[serde(rename = "serviceCategory")]
    service_category: T,
    specialty: FhirSpecialty,
    start: FhirInstant,
    end: FhirInstant,
    description: Option<String>,
    extension: Option<Value>,
}
impl<T> FhirAppointment<T>
where
    T: Serialize + Deserialize<'static> + Clone,
{
    pub fn new(
        status: FhirAppointmentStatus,
        service_category: T,
        specialty: FhirSpecialty,
        start: FhirInstant,
        end: FhirInstant,
        description: Option<String>,
        extension: Option<Value>,
    ) -> Self {
        Self {
            status,
            service_category,
            specialty,
            start,
            end,
            description,
            extension,
        }
    }
    pub fn get_extension(&self) -> &Option<Value> {
        &self.extension
    }
    pub fn get_start(&self) -> &FhirInstant {
        &self.start
    }
    pub fn get_end(&self) -> &FhirInstant {
        &self.end
    }
    pub fn get_specialty(&self) -> &FhirSpecialty {
        &self.specialty
    }
    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
    }
    pub fn change_status(&mut self, status: FhirAppointmentStatus) {
        self.status = status;
    }
    pub fn get_status(&self) -> &FhirAppointmentStatus {
        &self.status
    }
    pub fn get_service_category(&self) -> &T {
        &self.service_category
    }
}
impl<T> FhirResource for FhirAppointment<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn nostr_kind(&self) -> u32 {
        NOSTR_KIND_APPOINTMENT
    }
    fn resource_type(&self) -> &str {
        FHIR_RESOURCE_APPOINTMENT
    }
    fn text(&self) -> FhirText {
        FhirText::new(self.description.clone().unwrap_or_default())
    }
}
