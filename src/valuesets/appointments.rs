use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FhirAppointmentResponseStatus {
    Accepted,       // The participant has accepted the appointment.
    Declined, // The participant has declined the appointment and will not participate in the appointment.
    Tentative, // The participant has tentatively accepted the appointment. There is no commitment that attendance will occur.
    NeedsAction, // The participant needs to indicate if they accept the appointment by changing this status to one of the other statuses.
    EnteredInError, // This instance should not have been part of this patient's medical record.
}
impl TryFrom<&str> for FhirAppointmentResponseStatus {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(FhirAppointmentResponseStatus::Accepted),
            "Declined" => Ok(FhirAppointmentResponseStatus::Declined),
            "Tentative" => Ok(FhirAppointmentResponseStatus::Tentative),
            "NeedsAction" => Ok(FhirAppointmentResponseStatus::NeedsAction),
            "EnteredInError" => Ok(FhirAppointmentResponseStatus::EnteredInError),
            _ => Err("Invalid FhirAppointmentResponseStatus"),
        }
    }
}
impl Into<String> for FhirAppointmentResponseStatus {
    fn into(self) -> String {
        match self {
            FhirAppointmentResponseStatus::Accepted => "Accepted".to_string(),
            FhirAppointmentResponseStatus::Declined => "Declined".to_string(),
            FhirAppointmentResponseStatus::Tentative => "Tentative".to_string(),
            FhirAppointmentResponseStatus::NeedsAction => "NeedsAction".to_string(),
            FhirAppointmentResponseStatus::EnteredInError => "EnteredInError".to_string(),
        }
    }
}
impl FhirAppointmentResponseStatus {
    pub fn to_str(&self) -> &str {
        match self {
            FhirAppointmentResponseStatus::Accepted => "Accepted",
            FhirAppointmentResponseStatus::Declined => "Declined",
            FhirAppointmentResponseStatus::Tentative => "Tentative",
            FhirAppointmentResponseStatus::NeedsAction => "NeedsAction",
            FhirAppointmentResponseStatus::EnteredInError => "EnteredInError",
        }
    }
    pub fn to_spanish_str(&self) -> &str {
        match self {
            FhirAppointmentResponseStatus::Accepted => "Aceptado",
            FhirAppointmentResponseStatus::Declined => "Rechazado",
            FhirAppointmentResponseStatus::Tentative => "Tentativo",
            FhirAppointmentResponseStatus::NeedsAction => "Necesita acción",
            FhirAppointmentResponseStatus::EnteredInError => "Ingresado por error",
        }
    }
}

// https://www.hl7.org/fhir/valueset-appointmentstatus.html
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum FhirAppointmentStatus {
    Proposed,
    Pending,
    Booked,
    Arrived,
    Fulfilled,
    Cancelled,
    Noshow,
    EnteredInError,
    CheckedIn,
    Waitlist,
}

impl FhirAppointmentStatus {
    pub fn to_str(&self) -> &str {
        match self {
            FhirAppointmentStatus::Proposed => "Proposed",
            FhirAppointmentStatus::Pending => "Pending",
            FhirAppointmentStatus::Booked => "Booked",
            FhirAppointmentStatus::Arrived => "Arrived",
            FhirAppointmentStatus::Fulfilled => "Fulfilled",
            FhirAppointmentStatus::Cancelled => "Cancelled",
            FhirAppointmentStatus::Noshow => "Noshow",
            FhirAppointmentStatus::EnteredInError => "EnteredInError",
            FhirAppointmentStatus::CheckedIn => "CheckedIn",
            FhirAppointmentStatus::Waitlist => "Waitlist",
        }
    }
    pub fn to_spanish_str(&self) -> &str {
        match self {
            FhirAppointmentStatus::Proposed => "Propuesto",
            FhirAppointmentStatus::Pending => "Pendiente",
            FhirAppointmentStatus::Booked => "Reservado",
            FhirAppointmentStatus::Arrived => "Llegado",
            FhirAppointmentStatus::Fulfilled => "Cumplido",
            FhirAppointmentStatus::Cancelled => "Cancelado",
            FhirAppointmentStatus::Noshow => "No presentado",
            FhirAppointmentStatus::EnteredInError => "Ingresado por error",
            FhirAppointmentStatus::CheckedIn => "Registrado",
            FhirAppointmentStatus::Waitlist => "Lista de espera",
        }
    }
}
