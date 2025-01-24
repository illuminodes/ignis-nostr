# Ignis Nostro

Mapping of HL7 FHIR resources [HL7 FHIR standard](https://www.hl7.org/fhir/) from and to Nostr based data models. 
The library is currently under construction and does not provide full coverage for resources.

An Ignis Resource can be used to find and map a Nostr Event into a FHIR resource and vice versa. 
This allows integration of FHIR resources into Nostr based systems, and facilitates the creation 
of FHIR resources from Nostr based data models.

## FHIR Data Models

The library contains a set of structs and enums representing FHIR datatypes, value sets, and resources.
Structs and Enums have helper implementations attached to them to facilitate the creation and manipulation of data.

## Ignis resources

The library also contains a definition for a `IgnisResource` struct that can be used with all FHIR resources. 
This protocol provides a set of functions to convert a FHIR resource to a Nostr based data model and vice versa.

