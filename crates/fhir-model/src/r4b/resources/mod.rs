//! Resource definitions.

#[rustfmt::skip] // Too much for rustfmt
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
}

/// Implement the IdentifiableResource trait for the resources and the resource
/// enum.
macro_rules! impl_identifiable_resource {
	([$($resource:ident),*$(,)?]) => {
		$(impl_identifiable_resource!($resource);)*

		impl Resource {
			/// Return the resource as identifiable resource.
			#[must_use]
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
			fn identifier(&self) -> &Vec<Option<Identifier>> {
				&self.identifier
			}

			fn identifier_mut(&mut self) -> &mut Vec<Option<Identifier>> {
				&mut self.identifier
			}

			fn set_identifier(&mut self, value: Vec<Option<Identifier>>) {
				self.identifier = value;
			}

			fn identifier_ext(&self) -> &Vec<Option<FieldExtension>> {
				&self.identifier_ext
			}

			fn identifier_ext_mut(&mut self) -> &mut Vec<Option<FieldExtension>> {
				&mut self.identifier_ext
			}

			fn set_identifier_ext(&mut self, value: Vec<Option<FieldExtension>>) {
				self.identifier_ext = value;
			}
		}
	};
}

impl_identifiable_resource!([
	Account,
	ActivityDefinition,
	AdministrableProductDefinition,
	AllergyIntolerance,
	Appointment,
	AppointmentResponse,
	Basic,
	BiologicallyDerivedProduct,
	BodyStructure,
	CarePlan,
	CareTeam,
	CatalogEntry,
	ChargeItem,
	ChargeItemDefinition,
	Citation,
	Claim,
	ClaimResponse,
	ClinicalImpression,
	ClinicalUseDefinition,
	CodeSystem,
	Communication,
	CommunicationRequest,
	Condition,
	Consent,
	Contract,
	Coverage,
	CoverageEligibilityRequest,
	CoverageEligibilityResponse,
	DetectedIssue,
	Device,
	DeviceDefinition,
	DeviceMetric,
	DeviceRequest,
	DeviceUseStatement,
	DiagnosticReport,
	DocumentManifest,
	DocumentReference,
	Encounter,
	Endpoint,
	EnrollmentRequest,
	EnrollmentResponse,
	EpisodeOfCare,
	EventDefinition,
	Evidence,
	EvidenceReport,
	EvidenceVariable,
	ExampleScenario,
	ExplanationOfBenefit,
	FamilyMemberHistory,
	Flag,
	Goal,
	Group,
	GuidanceResponse,
	HealthcareService,
	ImagingStudy,
	Immunization,
	ImmunizationEvaluation,
	ImmunizationRecommendation,
	InsurancePlan,
	Invoice,
	Library,
	List,
	Location,
	ManufacturedItemDefinition,
	Measure,
	MeasureReport,
	Media,
	Medication,
	MedicationAdministration,
	MedicationDispense,
	MedicationRequest,
	MedicationStatement,
	MedicinalProductDefinition,
	MessageDefinition,
	MolecularSequence,
	NutritionOrder,
	Observation,
	ObservationDefinition,
	Organization,
	OrganizationAffiliation,
	PackagedProductDefinition,
	Patient,
	PaymentNotice,
	PaymentReconciliation,
	Person,
	PlanDefinition,
	Practitioner,
	PractitionerRole,
	Procedure,
	Questionnaire,
	RegulatedAuthorization,
	RelatedPerson,
	RequestGroup,
	ResearchDefinition,
	ResearchElementDefinition,
	ResearchStudy,
	ResearchSubject,
	RiskAssessment,
	Schedule,
	ServiceRequest,
	Slot,
	Specimen,
	StructureDefinition,
	StructureMap,
	SubscriptionTopic,
	Substance,
	SubstanceDefinition,
	SupplyDelivery,
	SupplyRequest,
	Task,
	ValueSet,
	VisionPrescription
]);
