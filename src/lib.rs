// pub mod consts;
// pub mod datatypes;
mod fhir_trait;
// pub mod metadata;
// pub mod resources;
mod valuesets;

pub use fhir_rs::*;
pub use fhir_trait::IgnisResource;
pub use valuesets::*;

#[cfg(test)]
mod tests {

    use crate::fhir_trait::IgnisResource;
    use fhir_rs::{
        prelude::{Convert, HumanName},
        resource::Practitioner,
    };
    use nostro2::userkeys::UserKeys;
    #[test]
    fn test_practitioner() {
        let human_name = HumanName::default()
            .set_given(vec!["John".into()])
            .set_family("Doe");
        let practitioner = Practitioner::default().set_name(vec![human_name]);
        let ignis_resource = IgnisResource::new(practitioner, None, None);
        println!("{:?}", ignis_resource);
        let name = ignis_resource.resource.name.clone().unwrap();
        let name = name.first().clone().unwrap();
        println!("{:?}", name);

        let given: String = name
            .given
            .clone()
            .unwrap()
            .first()
            .clone()
            .unwrap()
            .to_strings()
            .unwrap();
        println!("{:?}", given);
        let family: String = name.family.clone().unwrap().to_string();
        assert_eq!(given, "John".to_string());
        assert_eq!(family, "Doe".to_string());
    }

    #[test]
    fn test_ignis_resource() {
        let human_name = HumanName::default()
            .set_given(vec!["John".into()])
            .set_family("Doe");
        let practitioner = Practitioner::default().set_name(vec![human_name]);
        let mut ignis_resource = IgnisResource::new(practitioner, None, None);
        let keys = UserKeys::generate();
        let signed_note = ignis_resource.signed_resource(&keys).unwrap();
        let ignis_resource = IgnisResource::<Practitioner>::try_from(signed_note).unwrap();
        println!("{:?}", ignis_resource);
        let name = ignis_resource.resource.name.clone().unwrap();
        let name = name.first().clone().unwrap();
        println!("{:?}", name);
        let given: String = name
            .given
            .clone()
            .unwrap()
            .first()
            .clone()
            .unwrap()
            .to_strings()
            .unwrap();
        println!("{:?}", given);
        let family: String = name.family.clone().unwrap().to_string();
        assert_eq!(given, "John".to_string());
        assert_eq!(family, "Doe".to_string());
    }
}
