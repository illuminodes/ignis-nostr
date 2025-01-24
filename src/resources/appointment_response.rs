use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::{
    consts::{FHIR_RESOURCE_APPOINTMENT_RESPONSE, NOSTR_KIND_APPOINTMENT_RESPONSE},
    datatypes::FhirInstant,
    fhir_trait::{FhirReference, FhirResource, FhirText},
    valuesets::{FhirAppointmentResponseStatus, FhirAppointmentStatus},
};

use super::appointment::FhirAppointment;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FhirAppointmentResponse<A, P> {
    appointment: FhirAppointment<A>,
    start: FhirInstant,
    end: FhirInstant,
    actor: FhirReference<P>,
    #[serde(rename = "participantStatus")]
    participant_status: FhirAppointmentResponseStatus,
    #[serde(rename = "proposedNewTime")]
    proposed_new_time: bool,
    comment: Option<String>,
    extension: Option<Value>,
}

impl<A, P> FhirAppointmentResponse<A, P>
where
    A: Serialize + Deserialize<'static> + Clone,
    P: Serialize + Deserialize<'static> + Clone,
{
    pub fn new(
        appointment: FhirAppointment<A>,
        actor: FhirReference<P>,
        comment: Option<String>,
        extension: Option<Value>,
    ) -> Self {
        let start = appointment.get_start().clone();
        let end = appointment.get_end().clone();
        let status = match appointment.get_status() {
            FhirAppointmentStatus::Booked => FhirAppointmentResponseStatus::Accepted,
            FhirAppointmentStatus::Cancelled => FhirAppointmentResponseStatus::Declined,
            _ => FhirAppointmentResponseStatus::NeedsAction,
        };
        Self {
            appointment,
            start,
            end,
            actor,
            participant_status: status,
            proposed_new_time: false,
            comment,
            extension,
        }
    }
    pub fn propose_new_time(
        appointment: FhirAppointment<A>,
        actor: FhirReference<P>,
        comment: Option<String>,
        extension: Option<Value>,
    ) -> Self {
        let start = appointment.get_start().clone();
        let end = appointment.get_end().clone();
        Self {
            appointment,
            start,
            end,
            actor,
            participant_status: FhirAppointmentResponseStatus::Tentative,
            proposed_new_time: true,
            comment,
            extension,
        }
    }
    pub fn get_extension(&self) -> &Option<Value> {
        &self.extension
    }
    pub fn get_fhir_appointment(&self) -> &FhirAppointment<A> {
        &self.appointment
    }
    pub fn get_fhir_actor(&self) -> &FhirReference<P> {
        &self.actor
    }
    pub fn get_start(&self) -> &FhirInstant {
        &self.start
    }
    pub fn get_end(&self) -> &FhirInstant {
        &self.end
    }
    pub fn status(&self) -> FhirAppointmentResponseStatus {
        self.participant_status
    }
    pub fn get_comment(&self) -> Option<String> {
        self.comment.clone()
    }
}

impl<A, P> FhirResource for FhirAppointmentResponse<A, P>
where
    A: Serialize + DeserializeOwned + Clone,
    P: Serialize + DeserializeOwned + Clone,
{
    fn nostr_kind(&self) -> u32 {
        NOSTR_KIND_APPOINTMENT_RESPONSE
    }
    fn resource_type(&self) -> &str {
        FHIR_RESOURCE_APPOINTMENT_RESPONSE
    }
    fn text(&self) -> FhirText {
        FhirText::new(self.comment.clone().unwrap_or_default())
    }
}
