use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    consts::{FHIR_RESOURCE_APPOINTMENT_RESPONSE, NOSTR_KIND_APPOINTMENT_RESPONSE},
    datatypes::FhirInstant,
    fhir_trait::{FhirReference, FhirResource, FhirText},
    valuesets::FhirAppointmentResponseStatus,
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
}

impl<A, P> FhirAppointmentResponse<A, P>
where
    A: Serialize + Deserialize<'static> + Clone,
    P: Serialize + Deserialize<'static> + Clone,
{
    pub fn new_accepted(
        appointment: FhirAppointment<A>,
        actor: FhirReference<P>,
        comment: Option<String>,
    ) -> Self {
        let start = appointment.get_start().clone();
        let end = appointment.get_end().clone();
        Self {
            appointment,
            start,
            end,
            actor,
            participant_status: FhirAppointmentResponseStatus::Accepted,
            proposed_new_time: false,
            comment,
        }
    }
    pub fn new_declined(
        appointment: FhirAppointment<A>,
        actor: FhirReference<P>,
        comment: Option<String>,
    ) -> Self {
        let start = appointment.get_start().clone();
        let end = appointment.get_end().clone();
        Self {
            appointment,
            start,
            end,
            actor,
            participant_status: FhirAppointmentResponseStatus::Declined,
            proposed_new_time: false,
            comment,
        }
    }
    pub fn propose_new_time(
        appointment: FhirAppointment<A>,
        actor: FhirReference<P>,
        comment: Option<String>,
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
        }
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
