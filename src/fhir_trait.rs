use nostro2::{
    notes::{Note, SignedNote},
    userkeys::UserKeys,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FhirReference<T> {
    reference: String,
    identifier: Vec<FhirIdentifier>,
    display: String,
    #[serde(rename = "type")]
    reference_type: Option<T>,
}
impl<'a, T> FhirReference<T>
where
    T: FhirResource + Serialize +  Sized + Clone + DeserializeOwned,
{
    pub fn new(new_reference: T, reference_id: String, identifier: Vec<FhirIdentifier>) -> Self {
        let reference = format!("{}/{}", new_reference.resource_type(), reference_id);
        let display = new_reference.text().div.clone();
        let reference_type = Some(new_reference);
        Self {
            reference,
            identifier,
            display,
            reference_type,
        }
    }
    pub fn get_reference(&self) -> Option<T> {
        self.reference_type.clone()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FhirIdentifier {
    system: String,
    value: String,
}
impl FhirIdentifier {
    pub fn new(system: String, value: String) -> Self {
        Self { system, value }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FhirText {
    status: String,
    div: String,
}
impl FhirText {
    pub fn new(div_text: String) -> Self {
        Self {
            status: "generated".to_string(),
            div: format!(
                "<div xmlns=\"http://www.w3.org/1999/xhtml\">{}</div>",
                div_text
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IgnisResource<T> {
    resource: T,
    signed_note: Option<SignedNote>,
    relay: Option<String>,
}
impl<T> IgnisResource<T>
where
    T: FhirResource + Serialize + Sized + Clone + DeserializeOwned,
{
    pub fn new(resource: T, signed_note: Option<SignedNote>, relay: Option<String>) -> Self {
        Self {
            resource,
            signed_note,
            relay,
        }
    }
    pub fn get_resource(&self) -> &T {
        &self.resource
    }
    pub fn get_pubkey(&self) -> Option<String> {
        if let Some(note) = &self.signed_note {
            Some(note.get_pubkey().to_string())
        } else {
            None
        }
    }
    pub fn get_signed_note(&self) -> Option<&SignedNote> {
        self.signed_note.as_ref()
    }
    pub fn get_resource_type(&self) -> &str {
        self.resource.resource_type()
    }
    pub fn get_resource_id(&self) -> Option<String> {
        if let Some(note) = &self.signed_note {
            Some(note.get_id().to_string())
        } else {
            None
        }
    }
    pub fn get_resource_identifier(&self) -> FhirIdentifier {
        FhirIdentifier::new(
            format!("nostr:{}", self.relay.as_ref().unwrap()),
            self.get_resource_id().unwrap().to_string(),
        )
    }
    pub fn get_resource_text(&self) -> FhirText {
        self.resource.text()
    }
    pub fn get_resource_reference(&self) -> Result<FhirReference<T>, String> {
        if let Some(id) = self.get_resource_id() {
            Ok(FhirReference::new(
                self.resource.clone(),
                id.to_string(),
                vec![self.get_resource_identifier()],
            ))
        } else {
            Err("Resource has not been signed yet.".to_string())
        }
    }
    pub fn get_fhir_json(&self) -> Result<Value, String> {
        let resource_map = serde_json::to_value(&self.resource).map_err(|e| e.to_string())?;
        if let Some(id) = self.get_resource_id() {
            let mut resource_json = json!({
                "resourceType": self.resource.resource_type(),
                "id": id,
                "identifier": self.get_resource_identifier(),
                "text": self.get_resource_text(),
            });
            resource_json
                .as_object_mut()
                .unwrap()
                .extend(resource_map.as_object().unwrap().clone());
            Ok(resource_json)
        } else {
            Err("Resource has not been signed yet.".to_string())
        }
    }
    pub fn sign_data(&mut self, user_keys: &UserKeys) -> SignedNote {
        self.signed_note = Some(self.resource.sign_data(user_keys));
        self.signed_note.clone().unwrap()
    }
    pub fn sign_encrypted_data(
        &mut self,
        user_keys: &UserKeys,
        pubkey: String,
    ) -> Result<SignedNote, String> {
        self.signed_note = Some(self.resource.sign_encrypted_data(user_keys, pubkey)?);
        Ok(self.signed_note.clone().unwrap())
    }
    pub fn from_signed_note(
        signed_note: &SignedNote,
        relay: Option<String>,
    ) -> Result<Self, String> {
        let resource = T::from_signed_note(signed_note)?;
        Ok(Self::new(resource, Some(signed_note.clone()), relay))
    }
    pub fn from_encrypted_note(
        signed_note: &SignedNote,
        user_keys: &UserKeys,
        relay: Option<String>,
    ) -> Result<Self, String> {
        let resource = T::from_encrypted_note(signed_note, user_keys)?;
        Ok(Self::new(resource, Some(signed_note.clone()), relay))
    }
}

pub trait FhirResource
where
    Self: Serialize  + Sized + Clone+ Sized + DeserializeOwned,
{
    fn nostr_kind(&self) -> u32;
    fn resource_type(&self) -> &str;
    fn text(&self) -> FhirText;
    fn sign_data(&self, user_keys: &UserKeys) -> SignedNote {
        let new_note = Note::new(
            &user_keys.get_public_key(),
            self.nostr_kind(),
            &serde_json::to_string(self).unwrap(),
        );
        user_keys.sign_nostr_event(new_note)
    }
    fn sign_encrypted_data(
        &self,
        user_keys: &UserKeys,
        pubkey: String,
    ) -> Result<SignedNote, String> {
        let new_note = Note::new(
            &user_keys.get_public_key(),
            self.nostr_kind(),
            &serde_json::to_string(self).unwrap(),
        );
        user_keys
            .sign_nip_44_encrypted(new_note, pubkey)
            .map_err(|e| e.to_string())
    }
    fn from_signed_note(
        signed_note: &SignedNote,
    ) -> Result<Self, String> {
        let resource: Self = serde_json::from_str(&signed_note.get_content()).map_err(|e| e.to_string())?;
        Ok(resource)
    }
    fn from_encrypted_note(
        signed_note: &SignedNote,
        user_keys: &UserKeys,
    ) -> Result<Self, String> {
        let plaintext = user_keys
            .decrypt_nip_44_content(signed_note)
            .map_err(|e| e.to_string())?;
        let resource: Self = serde_json::from_str(&plaintext).map_err(|e| e.to_string())?;
        Ok(resource)
    }
}
