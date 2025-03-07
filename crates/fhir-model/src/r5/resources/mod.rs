//! Resource definitions.

#[rustfmt::skip] // Too much for rustfmt
#[allow(clippy::doc_lazy_continuation, reason = "Comments from FHIR spec")]
mod generated;

pub use generated::*;

use crate::identifiable_resource::identifiable_resource_code;

identifiable_resource_code!(
	r5,
	[
		Account,
		ActivityDefinition,
		ActorDefinition,
		AdministrableProductDefinition,
		AdverseEvent,
		AllergyIntolerance,
		Appointment,
		AppointmentResponse,
		ArtifactAssessment,
		Basic,
		BiologicallyDerivedProduct,
		BiologicallyDerivedProductDispense,
		BodyStructure,
		CapabilityStatement,
		CarePlan,
		CareTeam,
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
		Composition,
		ConceptMap,
		Condition,
		ConditionDefinition,
		Consent,
		Contract,
		Coverage,
		CoverageEligibilityRequest,
		CoverageEligibilityResponse,
		DetectedIssue,
		Device,
		DeviceAssociation,
		DeviceDefinition,
		DeviceDispense,
		DeviceMetric,
		DeviceRequest,
		DeviceUsage,
		DiagnosticReport,
		DocumentReference,
		Encounter,
		EncounterHistory,
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
		FormularyItem,
		GenomicStudy,
		Goal,
		GraphDefinition,
		Group,
		GuidanceResponse,
		HealthcareService,
		ImagingSelection,
		ImagingStudy,
		Immunization,
		ImmunizationEvaluation,
		ImmunizationRecommendation,
		ImplementationGuide,
		InsurancePlan,
		InventoryItem,
		InventoryReport,
		Invoice,
		Library,
		List,
		Location,
		ManufacturedItemDefinition,
		Measure,
		MeasureReport,
		Medication,
		MedicationAdministration,
		MedicationDispense,
		MedicationKnowledge,
		MedicationRequest,
		MedicationStatement,
		MedicinalProductDefinition,
		MessageDefinition,
		MolecularSequence,
		NamingSystem,
		NutritionIntake,
		NutritionOrder,
		Observation,
		OperationDefinition,
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
		QuestionnaireResponse,
		RegulatedAuthorization,
		RelatedPerson,
		RequestOrchestration,
		Requirements,
		ResearchStudy,
		ResearchSubject,
		RiskAssessment,
		Schedule,
		SearchParameter,
		ServiceRequest,
		Slot,
		Specimen,
		StructureDefinition,
		StructureMap,
		Subscription,
		SubscriptionTopic,
		Substance,
		SubstanceDefinition,
		SupplyDelivery,
		SupplyRequest,
		Task,
		TerminologyCapabilities,
		TestPlan,
		TestScript,
		Transport,
		ValueSet,
		VisionPrescription
	]
);
