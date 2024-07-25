//! Resource definitions.

#[rustfmt::skip] // Too much for rustfmt
#[allow(clippy::doc_lazy_continuation)] // Comments from FHIR spec.
mod generated;

pub use generated::*;

use super::types::{FieldExtension, Identifier};

/// Trait for all resources with multiple identifiers in the `identifier` field.
/// Simplifies access to identifiers.
pub trait IdentifiableResource {
	/// Get the identifier field.
	fn identifier(&self) -> &Vec<Option<Identifier>>;
	/// Get the identifier field mutably.
	fn identifier_mut(&mut self) -> &mut Vec<Option<Identifier>>;
	/// Set the identifier field.
	fn set_identifier(&mut self, value: Vec<Option<Identifier>>);

	/// Get the identifier extension field.
	fn identifier_ext(&self) -> &Vec<Option<FieldExtension>>;
	/// Get the identifier extension field mutably.
	fn identifier_ext_mut(&mut self) -> &mut Vec<Option<FieldExtension>>;
	/// Set the identifier extension field.
	fn set_identifier_ext(&mut self, value: Vec<Option<FieldExtension>>);

	/// Append or replace an identifier. If there is already an identifier with
	/// the same system or type (exact full match), it is replaced, otherwise
	/// appended.
	///
	/// Returns whether it was created (true) or replaced another identifier
	/// (false).
	fn place_identifier(&mut self, identifier: Identifier) -> bool {
		if let Some(ident) = self.identifier_mut().iter_mut().flatten().find(|ident| {
			(ident.system.is_some() && ident.system == identifier.system)
				|| (ident.r#type.is_some() && ident.r#type == identifier.r#type)
		}) {
			*ident = identifier;

			false
		} else {
			self.identifier_mut().push(Some(identifier));

			if !self.identifier_ext_mut().is_empty()
				&& self.identifier_mut().len() == self.identifier_ext_mut().len() + 1
			{
				self.identifier_ext_mut().push(None);
			}

			true
		}
	}

	/// Return the first identifier value for a given system.
	fn identifier_with_system(&self, system: &str) -> Option<&String> {
		self.identifier()
			.iter()
			.flatten()
			.filter(|ident| ident.system.as_ref().map_or(false, |sys| sys == system))
			.find_map(|ident| ident.value.as_ref())
	}

	/// Return a list of identifiers for a given system.
	fn identifiers_with_system(&self, system: &str) -> Vec<&Identifier> {
		self.identifier()
			.iter()
			.flatten()
			.filter(|ident| ident.system.as_ref().map_or(false, |sys| sys == system))
			.collect()
	}

	/// Return the first identifier value for a given type.
	fn identifier_with_type(&self, type_system: &str, type_code: &str) -> Option<&String> {
		self.identifier()
			.iter()
			.flatten()
			.filter(|ident| {
				ident.r#type.as_ref().map_or(false, |ty| {
					ty.coding.iter().flatten().any(|coding| {
						coding.system.as_deref() == Some(type_system)
							&& coding.code.as_deref() == Some(type_code)
					})
				})
			})
			.find_map(|ident| ident.value.as_ref())
	}

	/// Return a list of identifiers for a given type.
	fn identifiers_with_type(&self, type_system: &str, type_code: &str) -> Vec<&Identifier> {
		self.identifier()
			.iter()
			.flatten()
			.filter(|ident| {
				ident.r#type.as_ref().map_or(false, |ty| {
					ty.coding.iter().flatten().any(|coding| {
						coding.system.as_deref() == Some(type_system)
							&& coding.code.as_deref() == Some(type_code)
					})
				})
			})
			.collect()
	}
}

/// Implement the IdentifiableResource trait for the resources and the resource
/// enum.
macro_rules! impl_identifiable_resource {
	([$($resource:ident),*$(,)?]) => {
		$(impl_identifiable_resource!($resource);)*

		impl Resource {
			/// Return the resource as identifiable resource.
			#[must_use]
			#[inline]
			pub fn as_identifiable_resource(&self) -> Option<&dyn IdentifiableResource> {
				match self {
					$(
						Self::$resource(r) => Some(r),
					)*
					_ => None,
				}
			}

			/// Return the resource as mutable identifiable resource.
			#[must_use]
			#[inline]
			pub fn as_identifiable_resource_mut(&mut self) -> Option<&mut dyn IdentifiableResource> {
				match self {
					$(
						Self::$resource(r) => Some(r),
					)*
					_ => None,
				}
			}
		}
	};
	($resource:ident) => {
		impl IdentifiableResource for $resource {
			#[inline]
			fn identifier(&self) -> &Vec<Option<Identifier>> {
				&self.identifier
			}

			#[inline]
			fn identifier_mut(&mut self) -> &mut Vec<Option<Identifier>> {
				&mut self.identifier
			}

			#[inline]
			fn set_identifier(&mut self, value: Vec<Option<Identifier>>) {
				self.identifier = value;
			}

			#[inline]
			fn identifier_ext(&self) -> &Vec<Option<FieldExtension>> {
				&self.identifier_ext
			}

			#[inline]
			fn identifier_ext_mut(&mut self) -> &mut Vec<Option<FieldExtension>> {
				&mut self.identifier_ext
			}

			#[inline]
			fn set_identifier_ext(&mut self, value: Vec<Option<FieldExtension>>) {
				self.identifier_ext = value;
			}
		}
	};
}

impl_identifiable_resource!([
	Account,
	ActivityDefinition,
	AllergyIntolerance,
	Appointment,
	AppointmentResponse,
	Basic,
	BodySite,
	CarePlan,
	CareTeam,
	Claim,
	ClaimResponse,
	ClinicalImpression,
	Communication,
	CommunicationRequest,
	Condition,
	Coverage,
	DataElement,
	Device,
	DeviceRequest,
	DeviceUseStatement,
	DiagnosticReport,
	DocumentManifest,
	DocumentReference,
	EligibilityRequest,
	EligibilityResponse,
	Encounter,
	Endpoint,
	EnrollmentRequest,
	EnrollmentResponse,
	EpisodeOfCare,
	ExplanationOfBenefit,
	FamilyMemberHistory,
	Flag,
	Goal,
	Group,
	HealthcareService,
	ImagingStudy,
	Immunization,
	ImmunizationRecommendation,
	Library,
	List,
	Location,
	Measure,
	Media,
	MedicationAdministration,
	MedicationDispense,
	MedicationRequest,
	MedicationStatement,
	NutritionOrder,
	Observation,
	Organization,
	Patient,
	PaymentNotice,
	PaymentReconciliation,
	Person,
	PlanDefinition,
	Practitioner,
	PractitionerRole,
	Procedure,
	ProcedureRequest,
	ProcessRequest,
	ProcessResponse,
	Questionnaire,
	ReferralRequest,
	RelatedPerson,
	RequestGroup,
	ResearchStudy,
	Schedule,
	Sequence,
	ServiceDefinition,
	Slot,
	Specimen,
	StructureDefinition,
	StructureMap,
	Substance,
	Task,
	ValueSet,
	VisionPrescription,
]);
