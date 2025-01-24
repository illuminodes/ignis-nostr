#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IgnisResource<T>
where
    T: Sized + Clone,
{
    pub resource: T,
    pub signed_note: Option<nostro2::notes::NostrNote>,
    pub relay: Option<String>,
}
impl<'de, T> IgnisResource<T>
where
    T: Sized + Clone + fhir_rs::prelude::Serialize + fhir_rs::prelude::Deserialize<'de>,
{
    pub fn new(
        resource: T,
        signed_note: Option<nostro2::notes::NostrNote>,
        relay: Option<String>,
    ) -> Self {
        Self {
            resource,
            signed_note,
            relay,
        }
    }
    pub fn get_pubkey(&self) -> Option<String> {
        let note = self.signed_note.as_ref()?;
        Some(note.pubkey.clone())
    }
    pub fn ignis_id(&self) -> Option<String> {
        self.signed_note.as_ref()?.id.clone()
    }
    pub fn signed_resource(
        &mut self,
        nostr_keys: &nostro2::keypair::NostrKeypair,
    ) -> Result<nostro2::notes::NostrNote, Box<dyn std::error::Error>> {
        let resource_str = fhir_rs::prelude::to_json(&self.resource).map_err(|e| e.to_string())?;
        let mut new_note = nostro2::notes::NostrNote {
            pubkey: nostr_keys.public_key(),
            content: resource_str,
            kind: 82,
            ..Default::default()
        };
        nostr_keys.sign_nostr_event(&mut new_note);
        self.signed_note = Some(new_note.clone());
        Ok(new_note)
    }
}

impl<'de, T> TryFrom<&'de nostro2::notes::NostrNote> for IgnisResource<T>
where
    T: Sized + Clone + fhir_rs::prelude::Serialize + fhir_rs::prelude::Deserialize<'de>,
{
    type Error = Box<dyn std::error::Error>;
    fn try_from(signed_note: &'de nostro2::notes::NostrNote) -> Result<Self, Self::Error> {
        let borrowed: &'static str = Box::leak(signed_note.content.clone().into_boxed_str());
        let resource: Result<T, String> =
            fhir_rs::prelude::from_json(borrowed).map_err(|e| e.to_string());
        let resource = resource.clone()?;
        Ok(IgnisResource::new(
            resource,
            Some(signed_note.clone()),
            None,
        ))
    }
}
impl<'de, T> TryFrom<nostro2::notes::NostrNote> for IgnisResource<T>
where
    T: Sized + Clone + fhir_rs::prelude::Serialize + fhir_rs::prelude::Deserialize<'de>,
{
    type Error = Box<dyn std::error::Error>;
    fn try_from(signed_note: nostro2::notes::NostrNote) -> Result<Self, Self::Error> {
        let borrowed: &'static str = Box::leak(signed_note.content.clone().into_boxed_str());
        let resource: Result<T, String> =
            fhir_rs::prelude::from_json(borrowed).map_err(|e| e.to_string());
        let resource = resource.clone()?;
        Ok(IgnisResource::new(
            resource,
            Some(signed_note.clone()),
            None,
        ))
    }
}
