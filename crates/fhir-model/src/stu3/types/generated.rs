//! Generated code! Take a look at the generator-crate for changing this file!
#![allow(clippy::too_many_lines)]
use ::core::num::NonZeroU32;
use serde::{Serialize, Deserialize};
#[cfg(feature = "builders")]
use derive_builder::Builder;
use super::super::codes;
#[allow(unused_imports)]
use crate::{Base64Binary, Date, DateTime, Instant, Time, Integer64};
/** Base StructureDefinition for Address Type

 **[Address](http://hl7.org/fhir/StructureDefinition/Address) v3.0.2**

 An address expressed using postal conventions (as opposed to GPS or other location definition formats)

 An address expressed using postal conventions (as opposed to GPS or other location definition formats).  This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery.  There are a variety of postal address formats defined around the world.

 Note: address is for postal addresses, not physical locations. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Address(pub Box<AddressInner>);
/** Base StructureDefinition for Address Type

 **[Address](http://hl7.org/fhir/StructureDefinition/Address) v3.0.2**

 An address expressed using postal conventions (as opposed to GPS or other location definition formats)

 An address expressed using postal conventions (as opposed to GPS or other location definition formats).  This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery.  There are a variety of postal address formats defined around the world.

 Note: address is for postal addresses, not physical locations. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "AddressBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct AddressInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[AddressUse](http://hl7.org/fhir/ValueSet/address-use); home | work | temp | old - purpose of this address**

 The purpose of this address.

 This is labeled as "Is Modifier" because applications should not mistake a temporary or old address etc.for a current/permanent one. Applications can assume that an address is current unless it explicitly says that it is temporary or old. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "use")]
    pub r#use: Option<codes::AddressUse>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_use")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#use_ext: Option<FieldExtension>,
    /** **[AddressType](http://hl7.org/fhir/ValueSet/address-type); postal | physical | both**

 Distinguishes between physical addresses (those you can visit) and mailing addresses (e.g. PO Boxes and care-of addresses). Most addresses are both.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "type")]
    pub r#type: Option<codes::AddressType>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **Text representation of the address**

 A full text representation of the address.

 Can provide both a text representation and parts. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_text")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub text_ext: Option<FieldExtension>,
    /** **Street name, number, direction & P.O. Box etc.**

 This component contains the house number, apartment number, street name, street direction,  P.O. Box number, delivery hints, and similar address information.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "line")]
    pub line: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_line")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub line_ext: Vec<Option<FieldExtension>>,
    /** **Name of city, town etc.**

 The name of the city, town, village or other community or delivery center.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "city")]
    pub city: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_city")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub city_ext: Option<FieldExtension>,
    /** **District name (aka county)**

 The name of the administrative area (county).

 District is sometimes known as county, but in some regions 'county' is used in place of city (municipality), so county name should be conveyed in city instead. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "district")]
    pub district: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_district")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub district_ext: Option<FieldExtension>,
    /** **Sub-unit of country (abbreviations ok)**

 Sub-unit of a country with limited sovereignty in a federally organized country. A code may be used if codes are in common use (i.e. US 2 letter state codes).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "state")]
    pub state: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_state")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub state_ext: Option<FieldExtension>,
    /** **Postal code for area**

 A postal code designating a region defined by the postal service.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_postalCode")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub postal_code_ext: Option<FieldExtension>,
    /** **Country (e.g. can be ISO 3166 2 or 3 letter code)**

 Country - a nation as commonly understood or generally accepted.

 ISO 3166 3 letter codes can be used in place of a full country name. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "country")]
    pub country: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_country")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub country_ext: Option<FieldExtension>,
    /** **Time period when address was/is in use**

 Time period when address was/is in use.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "period")]
    pub period: Option<Period>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_period")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl AddressBuilder {
    #[doc = concat!("Finalize building ", "Address", ".")]
    pub fn build(self) -> Result<Address, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<AddressInner> for Address {
    fn from(inner: AddressInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Address {
    type Target = AddressInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Address {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Address {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> AddressBuilder {
        AddressBuilder::default()
    }
}
/** Base StructureDefinition for Age Type

 **[Age](http://hl7.org/fhir/StructureDefinition/Age) v3.0.2**

 A duration of time during which an organism (or a process) has existed

 A duration of time during which an organism (or a process) has existed.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Age(pub Box<AgeInner>);
/** Base StructureDefinition for Age Type

 **[Age](http://hl7.org/fhir/StructureDefinition/Age) v3.0.2**

 A duration of time during which an organism (or a process) has existed

 A duration of time during which an organism (or a process) has existed.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "AgeBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct AgeInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerical value (with implicit precision)**

 The value of the measured amount. The value includes an implicit precision in the presentation of the value.

 The implicit precision in the value should always be honored. Monetary values have their own rules for handling precision (refer to standard accounting text books). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /** **[QuantityComparator](http://hl7.org/fhir/ValueSet/quantity-comparator); < | <= | >= | > - how to understand the value**

 How the value should be understood and represented - whether the actual value is greater or less than the stated value due to measurement issues; e.g. if the comparator is "<" , then the real value is < stated value.

 This is labeled as "Is Modifier" because the comparator modifies the interpretation of the value significantly. If there is no comparator, then there is no modification of the value. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comparator")]
    pub comparator: Option<codes::QuantityComparator>,
    /** **Unit representation**

 A human-readable form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    /** **System that defines coded unit form**

 The identification of the system that provides the coded form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /** **Coded form of the unit**

 A computer processable form of the unit in some unit representation system.

 The preferred system is UCUM, but SNOMED CT can also be used (for customary units) or ISO 4217 for currency.  The context of use may additionally require a code from a particular system. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
}
#[cfg(feature = "builders")]
impl AgeBuilder {
    #[doc = concat!("Finalize building ", "Age", ".")]
    pub fn build(self) -> Result<Age, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<AgeInner> for Age {
    fn from(inner: AgeInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Age {
    type Target = AgeInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Age {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Age {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> AgeBuilder {
        AgeBuilder::default()
    }
}
/** Base StructureDefinition for Annotation Type

 **[Annotation](http://hl7.org/fhir/StructureDefinition/Annotation) v3.0.2**

 Text node with attribution

 A  text note which also  contains information about who made the statement and when.

 For systems that do not have structured annotations, they can simply communicate a single annotation with no author or time.  This element may need to be included in narrative because of the potential for modifying information.  *Annotations SHOULD NOT* be used to communicate "modifying" information that could be computable. (This is a SHOULD because enforcing user behavior is nearly impossible). */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Annotation(pub Box<AnnotationInner>);
/** Base StructureDefinition for Annotation Type

 **[Annotation](http://hl7.org/fhir/StructureDefinition/Annotation) v3.0.2**

 Text node with attribution

 A  text note which also  contains information about who made the statement and when.

 For systems that do not have structured annotations, they can simply communicate a single annotation with no author or time.  This element may need to be included in narrative because of the potential for modifying information.  *Annotations SHOULD NOT* be used to communicate "modifying" information that could be computable. (This is a SHOULD because enforcing user behavior is nearly impossible). */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "AnnotationBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct AnnotationInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Individual responsible for the annotation**

 The individual responsible for making the annotation.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub author: Option<AnnotationAuthor>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub author_ext: Option<AnnotationAuthorExtension>,
    /** **When the annotation was made**

 Indicates when this particular annotation was made.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "time")]
    pub time: Option<DateTime>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_time")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub time_ext: Option<FieldExtension>,
    /** **The annotation  - text content**

 The text of the annotation.

 */
    #[serde(rename = "text")]
    pub text: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_text")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub text_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl AnnotationBuilder {
    #[doc = concat!("Finalize building ", "Annotation", ".")]
    pub fn build(self) -> Result<Annotation, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<AnnotationInner> for Annotation {
    fn from(inner: AnnotationInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Annotation {
    type Target = AnnotationInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Annotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Annotation {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> AnnotationBuilder {
        AnnotationBuilder::default()
    }
}
/// Choice of types for the author[x] field in Annotation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AnnotationAuthor {
    /// Variant accepting the Reference type.
    #[serde(rename = "authorReference")]
    Reference(Reference),
    /// Variant accepting the String type.
    #[serde(rename = "authorString")]
    String(String),
}
/// Extension value for AnnotationAuthor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AnnotationAuthorExtension {
    /// Variant accepting the Reference extension.
    #[serde(rename = "_authorReference")]
    Reference(FieldExtension),
    /// Variant accepting the String extension.
    #[serde(rename = "_authorString")]
    String(FieldExtension),
}
/** Base StructureDefinition for Attachment Type

 **[Attachment](http://hl7.org/fhir/StructureDefinition/Attachment) v3.0.2**

 Content in a format defined elsewhere

 For referring to data content defined in other formats.

 When providing a summary view (for example with Observation.value[x]) Attachment should be represented with a brief display text such as "Attachment". */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Attachment(pub Box<AttachmentInner>);
/** Base StructureDefinition for Attachment Type

 **[Attachment](http://hl7.org/fhir/StructureDefinition/Attachment) v3.0.2**

 Content in a format defined elsewhere

 For referring to data content defined in other formats.

 When providing a summary view (for example with Observation.value[x]) Attachment should be represented with a brief display text such as "Attachment". */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "AttachmentBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct AttachmentInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[MimeType](http://www.rfc-editor.org/bcp/bcp13.txt); Mime type of the content, with charset etc.**

 Identifies the type of the data in the attachment and allows a method to be chosen to interpret or render the data. Includes mime type parameters such as charset where appropriate.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_contentType")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub content_type_ext: Option<FieldExtension>,
    /** **[Language](http://hl7.org/fhir/ValueSet/languages); Human language of the content (BCP-47)**

 The human language of the content. The value can be any valid value according to BCP 47.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "language")]
    pub language: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_language")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub language_ext: Option<FieldExtension>,
    /** **Data inline, base64ed**

 The actual data of the attachment - a sequence of bytes. In XML, represented using base64.

 The base64-encoded data SHALL be expressed in the same character set as the base resource XML or JSON. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "data")]
    pub data: Option<Base64Binary>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_data")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub data_ext: Option<FieldExtension>,
    /** **Uri where the data can be found**

 An alternative location where the data can be accessed.

 If both data and url are provided, the url SHALL point to the same content as the data contains. Urls may be relative references or may reference transient locations such as a wrapping envelope using cid: though this has ramifications for using signatures. Relative URLs are interpreted relative to the service url, like a resource reference, rather than relative to the resource itself. If a URL is provided, it SHALL resolve to actual data. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_url")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub url_ext: Option<FieldExtension>,
    /** **Number of bytes of content (if url provided)**

 The number of bytes of data that make up this attachment (before base64 encoding, if that is done).

 The number of bytes is redundant if the data is provided as a base64binary, but is useful if the data is provided as a url reference. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "size")]
    pub size: Option<u32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_size")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub size_ext: Option<FieldExtension>,
    /** **Hash of the data (sha-1, base64ed)**

 The calculated hash of the data using SHA-1. Represented using base64.

 The hash is calculated on the data prior to base64 encoding, if the data is based64 encoded. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "hash")]
    pub hash: Option<Base64Binary>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_hash")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub hash_ext: Option<FieldExtension>,
    /** **Label to display in place of the data**

 A label or set of text to display in place of the data.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_title")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub title_ext: Option<FieldExtension>,
    /** **Date attachment was first created**

 The date that the attachment was first created.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "creation")]
    pub creation: Option<DateTime>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_creation")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub creation_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl AttachmentBuilder {
    #[doc = concat!("Finalize building ", "Attachment", ".")]
    pub fn build(self) -> Result<Attachment, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<AttachmentInner> for Attachment {
    fn from(inner: AttachmentInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Attachment {
    type Target = AttachmentInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Attachment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Attachment {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> AttachmentBuilder {
        AttachmentBuilder::default()
    }
}
/** Base StructureDefinition for CodeableConcept Type

 **[CodeableConcept](http://hl7.org/fhir/StructureDefinition/CodeableConcept) v3.0.2**

 Concept - reference to a terminology or just  text

 A concept that may be defined by a formal reference to a terminology or ontology or may be provided by text.

 Not all terminology uses fit this general pattern. In some cases, models should not use CodeableConcept and use Coding directly and provide their own structure for managing text, codings, translations and the relationship between elements and pre- and post-coordination. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CodeableConcept(pub Box<CodeableConceptInner>);
/** Base StructureDefinition for CodeableConcept Type

 **[CodeableConcept](http://hl7.org/fhir/StructureDefinition/CodeableConcept) v3.0.2**

 Concept - reference to a terminology or just  text

 A concept that may be defined by a formal reference to a terminology or ontology or may be provided by text.

 Not all terminology uses fit this general pattern. In some cases, models should not use CodeableConcept and use Coding directly and provide their own structure for managing text, codings, translations and the relationship between elements and pre- and post-coordination. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "CodeableConceptBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct CodeableConceptInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Code defined by a terminology system**

 A reference to a code defined by a terminology system.

 Codes may be defined very casually in enumerations, or code lists, up to very formal definitions such as SNOMED CT - see the HL7 v3 Core Principles for more information.  Ordering of codings is undefined and SHALL NOT be used to infer meaning. Generally, at most only one of the coding values will be labeled as UserSelected = true. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "coding")]
    pub coding: Vec<Option<Coding>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_coding")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub coding_ext: Vec<Option<FieldExtension>>,
    /** **Plain text representation of the concept**

 A human language representation of the concept as seen/selected/uttered by the user who entered the data and/or which represents the intended meaning of the user.

 Very often the text is the same as a displayName of one of the codings. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_text")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub text_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl CodeableConceptBuilder {
    #[doc = concat!("Finalize building ", "CodeableConcept", ".")]
    pub fn build(self) -> Result<CodeableConcept, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<CodeableConceptInner> for CodeableConcept {
    fn from(inner: CodeableConceptInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for CodeableConcept {
    type Target = CodeableConceptInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for CodeableConcept {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl CodeableConcept {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> CodeableConceptBuilder {
        CodeableConceptBuilder::default()
    }
}
/** Base StructureDefinition for Coding Type

 **[Coding](http://hl7.org/fhir/StructureDefinition/Coding) v3.0.2**

 A reference to a code defined by a terminology system

 A reference to a code defined by a terminology system.

 Codes may be defined very casually in enumerations or code lists, up to very formal definitions such as SNOMED CT - see the HL7 v3 Core Principles for more information. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Coding(pub Box<CodingInner>);
/** Base StructureDefinition for Coding Type

 **[Coding](http://hl7.org/fhir/StructureDefinition/Coding) v3.0.2**

 A reference to a code defined by a terminology system

 A reference to a code defined by a terminology system.

 Codes may be defined very casually in enumerations or code lists, up to very formal definitions such as SNOMED CT - see the HL7 v3 Core Principles for more information. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "CodingBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct CodingInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Identity of the terminology system**

 The identification of the code system that defines the meaning of the symbol in the code.

 The URI may be an OID (urn:oid:...) or a UUID (urn:uuid:...).  OIDs and UUIDs SHALL be references to the HL7 OID registry. Otherwise, the URI should come from HL7's list of FHIR defined special URIs or it should de-reference to some definition that establish the system clearly and unambiguously. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_system")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub system_ext: Option<FieldExtension>,
    /** **Version of the system - if relevant**

 The version of the code system which was used when choosing this code. Note that a well-maintained code system does not need the version reported, because the meaning of codes is consistent across versions. However this cannot consistently be assured. and when the meaning is not guaranteed to be consistent, the version SHOULD be exchanged.

 Where the terminology does not clearly define what string should be used to identify code system versions, the recommendation is to use the date (expressed in FHIR date format) on which that version was officially published as the version date. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "version")]
    pub version: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_version")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub version_ext: Option<FieldExtension>,
    /** **Symbol in syntax defined by the system**

 A symbol in syntax defined by the system. The symbol may be a predefined code or an expression in a syntax defined by the coding system (e.g. post-coordination).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_ext: Option<FieldExtension>,
    /** **Representation defined by the system**

 A representation of the meaning of the code in the system, following the rules of the system.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "display")]
    pub display: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_display")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub display_ext: Option<FieldExtension>,
    /** **If this coding was chosen directly by the user**

 Indicates that this coding was chosen by a user directly - i.e. off a pick list of available items (codes or displays).

 Amongst a set of alternatives, a directly chosen code is the most appropriate starting point for new translations. There is some ambiguity about what exactly 'directly chosen' implies, and trading partner agreement may be needed to clarify the use of this element and its consequences more completely. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "userSelected")]
    pub user_selected: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_userSelected")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub user_selected_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl CodingBuilder {
    #[doc = concat!("Finalize building ", "Coding", ".")]
    pub fn build(self) -> Result<Coding, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<CodingInner> for Coding {
    fn from(inner: CodingInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Coding {
    type Target = CodingInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Coding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Coding {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> CodingBuilder {
        CodingBuilder::default()
    }
}
/** Base StructureDefinition for ContactDetail Type

 **[ContactDetail](http://hl7.org/fhir/StructureDefinition/ContactDetail) v3.0.2**

 Contact information

 Specifies contact information for a person or organization.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ContactDetail(pub Box<ContactDetailInner>);
/** Base StructureDefinition for ContactDetail Type

 **[ContactDetail](http://hl7.org/fhir/StructureDefinition/ContactDetail) v3.0.2**

 Contact information

 Specifies contact information for a person or organization.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ContactDetailBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ContactDetailInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Name of an individual to contact**

 The name of an individual to contact.

 If there is no named individual, the telecom information is for the organization as a whole. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_name")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub name_ext: Option<FieldExtension>,
    /** **Contact details for individual or organization**

 The contact details for the individual (if a name was provided) or the organization.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "telecom")]
    pub telecom: Vec<Option<ContactPoint>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_telecom")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub telecom_ext: Vec<Option<FieldExtension>>,
}
#[cfg(feature = "builders")]
impl ContactDetailBuilder {
    #[doc = concat!("Finalize building ", "ContactDetail", ".")]
    pub fn build(self) -> Result<ContactDetail, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ContactDetailInner> for ContactDetail {
    fn from(inner: ContactDetailInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for ContactDetail {
    type Target = ContactDetailInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for ContactDetail {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ContactDetail {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ContactDetailBuilder {
        ContactDetailBuilder::default()
    }
}
/** Base StructureDefinition for ContactPoint Type

 **[ContactPoint](http://hl7.org/fhir/StructureDefinition/ContactPoint) v3.0.2**

 Details of a Technology mediated contact point (phone, fax, email, etc.)

 Details for all kinds of technology mediated contact points for a person or organization, including telephone, email, etc.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ContactPoint(pub Box<ContactPointInner>);
/** Base StructureDefinition for ContactPoint Type

 **[ContactPoint](http://hl7.org/fhir/StructureDefinition/ContactPoint) v3.0.2**

 Details of a Technology mediated contact point (phone, fax, email, etc.)

 Details for all kinds of technology mediated contact points for a person or organization, including telephone, email, etc.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ContactPointBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ContactPointInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[ContactPointSystem](http://hl7.org/fhir/ValueSet/contact-point-system); phone | fax | email | pager | url | sms | other**

 Telecommunications form for contact point - what communications system is required to make use of the contact.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<codes::ContactPointSystem>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_system")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub system_ext: Option<FieldExtension>,
    /** **The actual contact point details**

 The actual contact point details, in a form that is meaningful to the designated communication system (i.e. phone number or email address).

 Additional text data such as phone extension numbers, or notes about use of the contact are sometimes included in the value. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_value")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<FieldExtension>,
    /** **[ContactPointUse](http://hl7.org/fhir/ValueSet/contact-point-use); home | work | temp | old | mobile - purpose of this contact point**

 Identifies the purpose for the contact point.

 This is labeled as "Is Modifier" because applications should not mistake a temporary or old contact etc.for a current/permanent one. Applications can assume that a contact is current unless it explicitly says that it is temporary or old. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "use")]
    pub r#use: Option<codes::ContactPointUse>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_use")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#use_ext: Option<FieldExtension>,
    /** **Specify preferred order of use (1 = highest)**

 Specifies a preferred order in which to use a set of contacts. Contacts are ranked with lower values coming before higher values.

 Note that rank does not necessarily follow the order in which the contacts are represented in the instance. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "rank")]
    pub rank: Option<NonZeroU32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_rank")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub rank_ext: Option<FieldExtension>,
    /** **Time period when the contact point was/is in use**

 Time period when the contact point was/is in use.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "period")]
    pub period: Option<Period>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_period")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ContactPointBuilder {
    #[doc = concat!("Finalize building ", "ContactPoint", ".")]
    pub fn build(self) -> Result<ContactPoint, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ContactPointInner> for ContactPoint {
    fn from(inner: ContactPointInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for ContactPoint {
    type Target = ContactPointInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for ContactPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ContactPoint {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ContactPointBuilder {
        ContactPointBuilder::default()
    }
}
/** Base StructureDefinition for Contributor Type

 **[Contributor](http://hl7.org/fhir/StructureDefinition/Contributor) v3.0.2**

 Contributor information

 A contributor to the content of a knowledge asset, including authors, editors, reviewers, and endorsers.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Contributor(pub Box<ContributorInner>);
/** Base StructureDefinition for Contributor Type

 **[Contributor](http://hl7.org/fhir/StructureDefinition/Contributor) v3.0.2**

 Contributor information

 A contributor to the content of a knowledge asset, including authors, editors, reviewers, and endorsers.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ContributorBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ContributorInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[ContributorType](http://hl7.org/fhir/ValueSet/contributor-type); author | editor | reviewer | endorser**

 The type of contributor.

 */
    #[serde(rename = "type")]
    pub r#type: codes::ContributorType,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **Who contributed the content**

 The name of the individual or organization responsible for the contribution.

 */
    #[serde(rename = "name")]
    pub name: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_name")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub name_ext: Option<FieldExtension>,
    /** **Contact details of the contributor**

 Contact details to assist a user in finding and communicating with the contributor.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "contact")]
    pub contact: Vec<Option<ContactDetail>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_contact")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub contact_ext: Vec<Option<FieldExtension>>,
}
#[cfg(feature = "builders")]
impl ContributorBuilder {
    #[doc = concat!("Finalize building ", "Contributor", ".")]
    pub fn build(self) -> Result<Contributor, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ContributorInner> for Contributor {
    fn from(inner: ContributorInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Contributor {
    type Target = ContributorInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Contributor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Contributor {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ContributorBuilder {
        ContributorBuilder::default()
    }
}
/** Base StructureDefinition for Count Type

 **[Count](http://hl7.org/fhir/StructureDefinition/Count) v3.0.2**

 A measured or measurable amount

 A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Count(pub Box<CountInner>);
/** Base StructureDefinition for Count Type

 **[Count](http://hl7.org/fhir/StructureDefinition/Count) v3.0.2**

 A measured or measurable amount

 A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "CountBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct CountInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerical value (with implicit precision)**

 The value of the measured amount. The value includes an implicit precision in the presentation of the value.

 The implicit precision in the value should always be honored. Monetary values have their own rules for handling precision (refer to standard accounting text books). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /** **[QuantityComparator](http://hl7.org/fhir/ValueSet/quantity-comparator); < | <= | >= | > - how to understand the value**

 How the value should be understood and represented - whether the actual value is greater or less than the stated value due to measurement issues; e.g. if the comparator is "<" , then the real value is < stated value.

 This is labeled as "Is Modifier" because the comparator modifies the interpretation of the value significantly. If there is no comparator, then there is no modification of the value. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comparator")]
    pub comparator: Option<codes::QuantityComparator>,
    /** **Unit representation**

 A human-readable form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    /** **System that defines coded unit form**

 The identification of the system that provides the coded form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /** **Coded form of the unit**

 A computer processable form of the unit in some unit representation system.

 The preferred system is UCUM, but SNOMED CT can also be used (for customary units) or ISO 4217 for currency.  The context of use may additionally require a code from a particular system. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
}
#[cfg(feature = "builders")]
impl CountBuilder {
    #[doc = concat!("Finalize building ", "Count", ".")]
    pub fn build(self) -> Result<Count, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<CountInner> for Count {
    fn from(inner: CountInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Count {
    type Target = CountInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Count {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Count {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> CountBuilder {
        CountBuilder::default()
    }
}
/** Base StructureDefinition for DataRequirement Type

 **[DataRequirement](http://hl7.org/fhir/StructureDefinition/DataRequirement) v3.0.2**

 Describes a required data item

 Describes a required data item for evaluation in terms of the type of data, and optional code or date-based filters of the data.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DataRequirement(pub Box<DataRequirementInner>);
/** Base StructureDefinition for DataRequirement Type

 **[DataRequirement](http://hl7.org/fhir/StructureDefinition/DataRequirement) v3.0.2**

 Describes a required data item

 Describes a required data item for evaluation in terms of the type of data, and optional code or date-based filters of the data.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "DataRequirementBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct DataRequirementInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[FHIRAllTypes](http://hl7.org/fhir/ValueSet/all-types); The type of the required data**

 The type of the required data, specified as the type name of a resource. For profiles, this value is set to the type of the base resource of the profile.

 */
    #[serde(rename = "type")]
    pub r#type: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **The profile of the required data**

 The profile of the required data, specified as the uri of the profile definition.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "profile")]
    pub profile: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_profile")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub profile_ext: Vec<Option<FieldExtension>>,
    /** **Indicates that specific structure elements are referenced by the knowledge module**

 Indicates that specific elements of the type are referenced by the knowledge module and must be supported by the consumer in order to obtain an effective evaluation. This does not mean that a value is required for this element, only that the consuming system must understand the element and be able to provide values for it if they are available. Note that the value for this element can be a path to allow references to nested elements. In that case, all the elements along the path must be supported.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "mustSupport")]
    pub must_support: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_mustSupport")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub must_support_ext: Vec<Option<FieldExtension>>,
    /** **What codes are expected**

 Code filters specify additional constraints on the data, specifying the value set of interest for a particular element of the data.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "codeFilter")]
    pub code_filter: Vec<Option<DataRequirementCodeFilter>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_codeFilter")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub code_filter_ext: Vec<Option<FieldExtension>>,
    /** **What dates/date ranges are expected**

 Date filters specify additional constraints on the data in terms of the applicable date range for specific elements.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "dateFilter")]
    pub date_filter: Vec<Option<DataRequirementDateFilter>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_dateFilter")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub date_filter_ext: Vec<Option<FieldExtension>>,
}
#[cfg(feature = "builders")]
impl DataRequirementBuilder {
    #[doc = concat!("Finalize building ", "DataRequirement", ".")]
    pub fn build(self) -> Result<DataRequirement, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<DataRequirementInner> for DataRequirement {
    fn from(inner: DataRequirementInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for DataRequirement {
    type Target = DataRequirementInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for DataRequirement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DataRequirement {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> DataRequirementBuilder {
        DataRequirementBuilder::default()
    }
}
/// Sub-fields of the codeFilter field in DataRequirement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "DataRequirementCodeFilterBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct DataRequirementCodeFilter {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **The code-valued attribute of the filter**

 The code-valued attribute of the filter. The specified path must be resolvable from the type of the required data. The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to traverse multiple-cardinality sub-elements. Note that the index must be an integer constant. The path must resolve to an element of type code, Coding, or CodeableConcept.

 */
    #[serde(rename = "path")]
    pub path: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_path")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub path_ext: Option<FieldExtension>,
    /** **Valueset for the filter**

 The valueset for the code filter. The valueSet and value elements are exclusive. If valueSet is specified, the filter will return only those data items for which the value of the code-valued element specified in the path is a member of the specified valueset.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub value_set: Option<DataRequirementCodeFilterValueSet>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_set_ext: Option<DataRequirementCodeFilterValueSetExtension>,
    /** **What code is expected**

 The codes for the code filter. Only one of valueSet, valueCode, valueCoding, or valueCodeableConcept may be specified. If values are given, the filter will return only those data items for which the code-valued attribute specified by the path has a value that is one of the specified codes.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "valueCode")]
    pub value_code: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_valueCode")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub value_code_ext: Vec<Option<FieldExtension>>,
    /** **What Coding is expected**

 The Codings for the code filter. Only one of valueSet, valueCode, valueConding, or valueCodeableConcept may be specified. If values are given, the filter will return only those data items for which the code-valued attribute specified by the path has a value that is one of the specified Codings.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "valueCoding")]
    pub value_coding: Vec<Option<Coding>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_valueCoding")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub value_coding_ext: Vec<Option<FieldExtension>>,
    /** **What CodeableConcept is expected**

 The CodeableConcepts for the code filter. Only one of valueSet, valueCode, valueConding, or valueCodeableConcept may be specified. If values are given, the filter will return only those data items for which the code-valued attribute specified by the path has a value that is one of the specified CodeableConcepts.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Vec<Option<CodeableConcept>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_valueCodeableConcept")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub value_codeable_concept_ext: Vec<Option<FieldExtension>>,
}
#[cfg(feature = "builders")]
impl DataRequirementCodeFilter {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> DataRequirementCodeFilterBuilder {
        DataRequirementCodeFilterBuilder::default()
    }
}
/// Choice of types for the valueSet[x] field in DataRequirementCodeFilter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataRequirementCodeFilterValueSet {
    /// Variant accepting the String type.
    #[serde(rename = "valueSetString")]
    String(String),
    /// Variant accepting the Reference type.
    #[serde(rename = "valueSetReference")]
    Reference(Reference),
}
/// Extension value for DataRequirementCodeFilterValueSet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataRequirementCodeFilterValueSetExtension {
    /// Variant accepting the String extension.
    #[serde(rename = "_valueSetString")]
    String(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_valueSetReference")]
    Reference(FieldExtension),
}
/// Sub-fields of the dateFilter field in DataRequirement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "DataRequirementDateFilterBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct DataRequirementDateFilter {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **The date-valued attribute of the filter**

 The date-valued attribute of the filter. The specified path must be resolvable from the type of the required data. The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers ([x]) to traverse multiple-cardinality sub-elements. Note that the index must be an integer constant. The path must resolve to an element of type dateTime, Period, Schedule, or Timing.

 */
    #[serde(rename = "path")]
    pub path: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_path")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub path_ext: Option<FieldExtension>,
    /** **The value of the filter, as a Period, DateTime, or Duration value**

 The value of the filter. If period is specified, the filter will return only those data items that fall within the bounds determined by the Period, inclusive of the period boundaries. If dateTime is specified, the filter will return only those data items that are equal to the specified dateTime. If a Duration is specified, the filter will return only those data items that fall within Duration from now.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub value: Option<DataRequirementDateFilterValue>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<DataRequirementDateFilterValueExtension>,
}
#[cfg(feature = "builders")]
impl DataRequirementDateFilter {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> DataRequirementDateFilterBuilder {
        DataRequirementDateFilterBuilder::default()
    }
}
/// Choice of types for the value[x] field in DataRequirementDateFilter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataRequirementDateFilterValue {
    /// Variant accepting the DateTime type.
    #[serde(rename = "valueDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Period type.
    #[serde(rename = "valuePeriod")]
    Period(Period),
    /// Variant accepting the Duration type.
    #[serde(rename = "valueDuration")]
    Duration(Duration),
}
/// Extension value for DataRequirementDateFilterValue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataRequirementDateFilterValueExtension {
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_valueDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Period extension.
    #[serde(rename = "_valuePeriod")]
    Period(FieldExtension),
    /// Variant accepting the Duration extension.
    #[serde(rename = "_valueDuration")]
    Duration(FieldExtension),
}
/** Base StructureDefinition for Distance Type

 **[Distance](http://hl7.org/fhir/StructureDefinition/Distance) v3.0.2**

 A length - a value with a unit that is a physical distance

 A length - a value with a unit that is a physical distance.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Distance(pub Box<DistanceInner>);
/** Base StructureDefinition for Distance Type

 **[Distance](http://hl7.org/fhir/StructureDefinition/Distance) v3.0.2**

 A length - a value with a unit that is a physical distance

 A length - a value with a unit that is a physical distance.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "DistanceBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct DistanceInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerical value (with implicit precision)**

 The value of the measured amount. The value includes an implicit precision in the presentation of the value.

 The implicit precision in the value should always be honored. Monetary values have their own rules for handling precision (refer to standard accounting text books). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /** **[QuantityComparator](http://hl7.org/fhir/ValueSet/quantity-comparator); < | <= | >= | > - how to understand the value**

 How the value should be understood and represented - whether the actual value is greater or less than the stated value due to measurement issues; e.g. if the comparator is "<" , then the real value is < stated value.

 This is labeled as "Is Modifier" because the comparator modifies the interpretation of the value significantly. If there is no comparator, then there is no modification of the value. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comparator")]
    pub comparator: Option<codes::QuantityComparator>,
    /** **Unit representation**

 A human-readable form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    /** **System that defines coded unit form**

 The identification of the system that provides the coded form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /** **Coded form of the unit**

 A computer processable form of the unit in some unit representation system.

 The preferred system is UCUM, but SNOMED CT can also be used (for customary units) or ISO 4217 for currency.  The context of use may additionally require a code from a particular system. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
}
#[cfg(feature = "builders")]
impl DistanceBuilder {
    #[doc = concat!("Finalize building ", "Distance", ".")]
    pub fn build(self) -> Result<Distance, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<DistanceInner> for Distance {
    fn from(inner: DistanceInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Distance {
    type Target = DistanceInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Distance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Distance {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> DistanceBuilder {
        DistanceBuilder::default()
    }
}
/** Base StructureDefinition for Dosage Type

 **[Dosage](http://hl7.org/fhir/StructureDefinition/Dosage) v3.0.2**

 How the medication is/was taken or should be taken

 Indicates how the medication is/was taken or should be taken by the patient.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Dosage(pub Box<DosageInner>);
/** Base StructureDefinition for Dosage Type

 **[Dosage](http://hl7.org/fhir/StructureDefinition/Dosage) v3.0.2**

 How the medication is/was taken or should be taken

 Indicates how the medication is/was taken or should be taken by the patient.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "DosageBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct DosageInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **The order of the dosage instructions**

 Indicates the order in which the dosage instructions should be applied or interpreted.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "sequence")]
    pub sequence: Option<i32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_sequence")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub sequence_ext: Option<FieldExtension>,
    /** **Free text dosage instructions e.g. SIG**

 Free text dosage instructions e.g. SIG.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_text")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub text_ext: Option<FieldExtension>,
    /** **[AdditionalInstruction](http://hl7.org/fhir/ValueSet/additional-instruction-codes); Supplemental instruction - e.g. "with meals"**

 Supplemental instruction - e.g. "with meals".

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "additionalInstruction")]
    pub additional_instruction: Vec<Option<CodeableConcept>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_additionalInstruction")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub additional_instruction_ext: Vec<Option<FieldExtension>>,
    /** **Patient or consumer oriented instructions**

 Instructions in terms that are understood by the patient or consumer.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "patientInstruction")]
    pub patient_instruction: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_patientInstruction")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub patient_instruction_ext: Option<FieldExtension>,
    /** **When medication should be administered**

 When medication should be administered.

 This attribute may not always be populated while the Dosage.text is expected to be populated.  If both are populated, then the Dosage.text should reflect the content of the Dosage.timing. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "timing")]
    pub timing: Option<Timing>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_timing")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub timing_ext: Option<FieldExtension>,
    /** **Take "as needed" (for x)**

 Indicates whether the Medication is only taken when needed within a specific dosing schedule (Boolean option), or it indicates the precondition for taking the Medication (CodeableConcept).

 Can express "as needed" without a reason by setting the Boolean = True.  In this case the CodeableConcept is not populated.  Or you can express "as needed" with a reason by including the CodeableConcept.  In this case the Boolean is assumed to be True.  If you set the Boolean to False, then the dose is given according to the schedule and is not "prn" or "as needed". */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub as_needed: Option<DosageAsNeeded>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub as_needed_ext: Option<DosageAsNeededExtension>,
    /** **[MedicationAdministrationSite](http://hl7.org/fhir/ValueSet/approach-site-codes); Body site to administer to**

 Body site to administer to.

 If the use case requires attributes from the BodySite resource (e.g. to identify and track separately) then use the standard extension [body-site-instance](extension-body-site-instance.html).  May be a summary code, or a reference to a very precise definition of the location, or both. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "site")]
    pub site: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_site")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub site_ext: Option<FieldExtension>,
    /** **[RouteOfAdministration](http://hl7.org/fhir/ValueSet/route-codes); How drug should enter body**

 How drug should enter body.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "route")]
    pub route: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_route")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub route_ext: Option<FieldExtension>,
    /** **[MedicationAdministrationMethod](http://hl7.org/fhir/ValueSet/administration-method-codes); Technique for administering medication**

 Technique for administering medication.

 Terminologies used often pre-coordinate this term with the route and or form of administration. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "method")]
    pub method: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_method")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub method_ext: Option<FieldExtension>,
    /** **Amount of medication per dose**

 Amount of medication per dose.

 Note that this specifies the quantity of the specified medication, not the quantity for each active ingredient(s). Each ingredient amount can be communicated in the Medication resource. For example, if one wants to communicate that a tablet was 375 mg, where the dose was one tablet, you can use the Medication resource to document that the tablet was comprised of 375 mg of drug XYZ. Alternatively if the dose was 375 mg, then you may only need to use the Medication resource to indicate this was a tablet. If the example were an IV such as dopamine and you wanted to communicate that 400mg of dopamine was mixed in 500 ml of some IV solution, then this would all be communicated in the Medication resource. If the administration is not intended to be instantaneous (rate is present or timing has a duration), this can be specified to convey the total amount to be administered over the period of time as indicated by the schedule e.g. 500 ml in dose, with timing used to convey that this should be done over 4 hours. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub dose: Option<DosageDose>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub dose_ext: Option<DosageDoseExtension>,
    /** **Upper limit on medication per unit of time**

 Upper limit on medication per unit of time.

 This is intended for use as an adjunct to the dosage when there is an upper cap.  For example "2 tablets every 4 hours to a maximum of 8/day". */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "maxDosePerPeriod")]
    pub max_dose_per_period: Option<Ratio>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_maxDosePerPeriod")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_dose_per_period_ext: Option<FieldExtension>,
    /** **Upper limit on medication per administration**

 Upper limit on medication per administration.

 This is intended for use as an adjunct to the dosage when there is an upper cap.  For example, a body surface area related dose with a maximum amount, such as 1.5 mg/m2 (maximum 2 mg) IV over 5 – 10 minutes would have doseQuantity of 1.5 mg/m2 and maxDosePerAdministration of 2 mg. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "maxDosePerAdministration")]
    pub max_dose_per_administration: Option<Quantity>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_maxDosePerAdministration")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_dose_per_administration_ext: Option<FieldExtension>,
    /** **Upper limit on medication per lifetime of the patient**

 Upper limit on medication per lifetime of the patient.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "maxDosePerLifetime")]
    pub max_dose_per_lifetime: Option<Quantity>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_maxDosePerLifetime")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_dose_per_lifetime_ext: Option<FieldExtension>,
    /** **Amount of medication per unit of time**

 Amount of medication per unit of time.

 It is possible to supply both a rate and a doseQuantity to provide full details about how the medication is to be administered and supplied. If the rate is intended to change over time, depending on local rules/regulations, each change should be captured as a new version of the MedicationRequest with an updated rate, or captured with a new MedicationRequest with the new rate. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub rate: Option<DosageRate>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub rate_ext: Option<DosageRateExtension>,
}
#[cfg(feature = "builders")]
impl DosageBuilder {
    #[doc = concat!("Finalize building ", "Dosage", ".")]
    pub fn build(self) -> Result<Dosage, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<DosageInner> for Dosage {
    fn from(inner: DosageInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Dosage {
    type Target = DosageInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Dosage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Dosage {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> DosageBuilder {
        DosageBuilder::default()
    }
}
/// Choice of types for the asNeeded[x] field in Dosage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DosageAsNeeded {
    /// Variant accepting the Boolean type.
    #[serde(rename = "asNeededBoolean")]
    Boolean(bool),
    /// Variant accepting the CodeableConcept type.
    #[serde(rename = "asNeededCodeableConcept")]
    CodeableConcept(CodeableConcept),
}
/// Extension value for DosageAsNeeded.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DosageAsNeededExtension {
    /// Variant accepting the Boolean extension.
    #[serde(rename = "_asNeededBoolean")]
    Boolean(FieldExtension),
    /// Variant accepting the CodeableConcept extension.
    #[serde(rename = "_asNeededCodeableConcept")]
    CodeableConcept(FieldExtension),
}
/// Choice of types for the dose[x] field in Dosage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DosageDose {
    /// Variant accepting the Range type.
    #[serde(rename = "doseRange")]
    Range(Range),
    /// Variant accepting the Quantity type.
    #[serde(rename = "doseQuantity")]
    Quantity(Quantity),
}
/// Extension value for DosageDose.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DosageDoseExtension {
    /// Variant accepting the Range extension.
    #[serde(rename = "_doseRange")]
    Range(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_doseQuantity")]
    Quantity(FieldExtension),
}
/// Choice of types for the rate[x] field in Dosage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DosageRate {
    /// Variant accepting the Ratio type.
    #[serde(rename = "rateRatio")]
    Ratio(Ratio),
    /// Variant accepting the Range type.
    #[serde(rename = "rateRange")]
    Range(Range),
    /// Variant accepting the Quantity type.
    #[serde(rename = "rateQuantity")]
    Quantity(Quantity),
}
/// Extension value for DosageRate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DosageRateExtension {
    /// Variant accepting the Ratio extension.
    #[serde(rename = "_rateRatio")]
    Ratio(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_rateRange")]
    Range(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_rateQuantity")]
    Quantity(FieldExtension),
}
/** Base StructureDefinition for Duration Type

 **[Duration](http://hl7.org/fhir/StructureDefinition/Duration) v3.0.2**

 A length of time

 A length of time.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Duration(pub Box<DurationInner>);
/** Base StructureDefinition for Duration Type

 **[Duration](http://hl7.org/fhir/StructureDefinition/Duration) v3.0.2**

 A length of time

 A length of time.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "DurationBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct DurationInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerical value (with implicit precision)**

 The value of the measured amount. The value includes an implicit precision in the presentation of the value.

 The implicit precision in the value should always be honored. Monetary values have their own rules for handling precision (refer to standard accounting text books). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /** **[QuantityComparator](http://hl7.org/fhir/ValueSet/quantity-comparator); < | <= | >= | > - how to understand the value**

 How the value should be understood and represented - whether the actual value is greater or less than the stated value due to measurement issues; e.g. if the comparator is "<" , then the real value is < stated value.

 This is labeled as "Is Modifier" because the comparator modifies the interpretation of the value significantly. If there is no comparator, then there is no modification of the value. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comparator")]
    pub comparator: Option<codes::QuantityComparator>,
    /** **Unit representation**

 A human-readable form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    /** **System that defines coded unit form**

 The identification of the system that provides the coded form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /** **Coded form of the unit**

 A computer processable form of the unit in some unit representation system.

 The preferred system is UCUM, but SNOMED CT can also be used (for customary units) or ISO 4217 for currency.  The context of use may additionally require a code from a particular system. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
}
#[cfg(feature = "builders")]
impl DurationBuilder {
    #[doc = concat!("Finalize building ", "Duration", ".")]
    pub fn build(self) -> Result<Duration, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<DurationInner> for Duration {
    fn from(inner: DurationInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Duration {
    type Target = DurationInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Duration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Duration {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> DurationBuilder {
        DurationBuilder::default()
    }
}
/** Base StructureDefinition for ElementDefinition Type

 **[ElementDefinition](http://hl7.org/fhir/StructureDefinition/ElementDefinition) v3.0.2**

 Definition of an element in a resource or extension

 Captures constraints on each element within the resource, profile, or extension.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ElementDefinition(pub Box<ElementDefinitionInner>);
/** Base StructureDefinition for ElementDefinition Type

 **[ElementDefinition](http://hl7.org/fhir/StructureDefinition/ElementDefinition) v3.0.2**

 Definition of an element in a resource or extension

 Captures constraints on each element within the resource, profile, or extension.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ElementDefinitionInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Path of the element in the hierarchy of elements**

 The path identifies the element and is expressed as a "."-separated list of ancestor elements, beginning with the name of the resource or extension.

 */
    #[serde(rename = "path")]
    pub path: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_path")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub path_ext: Option<FieldExtension>,
    /** **[PropertyRepresentation](http://hl7.org/fhir/ValueSet/property-representation); xmlAttr | xmlText | typeAttr | cdaText | xhtml**

 Codes that define how this element is represented in instances, when the deviation varies from the normal case.

 In resources, this is rarely used except for special cases where the representation deviates from the normal, and can only be done in the base standard (and profiles must reproduce what the base standard does). This element is used quite commonly in Logical models when the logical models represent a specific serialization format (e.g. CDA, v2 etc). */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "representation")]
    pub representation: Vec<Option<codes::PropertyRepresentation>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_representation")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub representation_ext: Vec<Option<FieldExtension>>,
    /** **Name for this particular element (in a set of slices)**

 The name of this element definition slice, when slicing is working. The name must be a token with no dots or spaces. This is a unique name referring to a specific set of constraints applied to this element, used to provide a name to different slices of the same element.

 The name SHALL be unique within the structure within the context of the constrained resource element.  (Though to avoid confusion, uniqueness across all elements is recommended.). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "sliceName")]
    pub slice_name: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_sliceName")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub slice_name_ext: Option<FieldExtension>,
    /** **Name for element to display with or prompt for element**

 A single preferred label which is the text to display beside the element indicating its meaning or to use to prompt for the element in a user display or form.

 See also the extension (http://hl7.org/fhir/StructureDefinition/elementdefinition-question)[extension-elementdefinition-question.html]. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_label")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub label_ext: Option<FieldExtension>,
    /** **[ElementDefinitionCode](http://hl7.org/fhir/ValueSet/observation-codes); Corresponding codes in terminologies**

 A code that has the same meaning as the element in a particular terminology.

 The concept SHALL be properly aligned with the data element definition and other constraints, as defined in the code system, including relationships, of any code listed here.  Where multiple codes exist in a terminology that could correspond to the data element, the most granular code(s) should be selected, so long as they are not more restrictive than the data element itself. The mappings may be used to provide more or less granular or structured equivalences in the code system. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Vec<Option<Coding>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub code_ext: Vec<Option<FieldExtension>>,
    /** **This element is sliced - slices follow**

 Indicates that the element is sliced into a set of alternative definitions (i.e. in a structure definition, there are multiple different constraints on a single element in the base resource). Slicing can be used in any resource that has cardinality ..* on the base resource, or any resource with a choice of types. The set of slices is any elements that come after this in the element sequence that have the same path, until a shorter path occurs (the shorter path terminates the set).

 The first element in the sequence, the one that carries the slicing, is the definition that applies to all the slices. This is based on the unconstrained element, but can apply any constraints as appropriate. This may include the common constraints on the children of the element. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "slicing")]
    pub slicing: Option<ElementDefinitionSlicing>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_slicing")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub slicing_ext: Option<FieldExtension>,
    /** **Concise definition for space-constrained presentation**

 A concise description of what this element means (e.g. for use in autogenerated summaries).

 May change the term to provide language more appropriate to the context of the profile or to reflect slicing. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "short")]
    pub short: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_short")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub short_ext: Option<FieldExtension>,
    /** **Full formal definition as narrative text**

 Provides a complete explanation of the meaning of the data element for human readability.  For the case of elements derived from existing elements (e.g. constraints), the definition SHALL be consistent with the base definition, but convey the meaning of the element in the particular context of use of the resource.

 It is easy for a different definition to change the meaning of an element and this can have nasty downstream consequences. Please be careful when providing definitions. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "definition")]
    pub definition: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_definition")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub definition_ext: Option<FieldExtension>,
    /** **Comments about the use of this element**

 Explanatory notes and implementation guidance about the data element, including notes about how to use the data properly, exceptions to proper use, etc.

 If it is possible to capture usage rules using constraints, that mechanism should be used in preference to this element. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_comment")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub comment_ext: Option<FieldExtension>,
    /** **Why this resource has been created**

 This element is for traceability of why the element was created and why the constraints exist as they do. This may be used to point to source materials or specifications that drove the structure of this element.

 This element does not describe the usage of the element (that's done in comments), rather it's for traceability of *why* the element is either needed or why the constraints exist as they do.  This may be used to point to source materials or specifications that drove the structure of this data element. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "requirements")]
    pub requirements: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_requirements")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub requirements_ext: Option<FieldExtension>,
    /** **Other names**

 Identifies additional names by which this element might also be known.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "alias")]
    pub alias: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_alias")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub alias_ext: Vec<Option<FieldExtension>>,
    /** **Minimum Cardinality**

 The minimum number of times this element SHALL appear in the instance.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "min")]
    pub min: Option<u32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_min")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub min_ext: Option<FieldExtension>,
    /** **Maximum Cardinality (a number or *)**

 The maximum number of times this element is permitted to appear in the instance.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "max")]
    pub max: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_max")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_ext: Option<FieldExtension>,
    /** **Base definition information for tools**

 Information about the base definition of the element, provided to make it unnecessary for tools to trace the deviation of the element through the derived and related profiles. This information is provided when the element definition is not the original definition of an element - i.g. either in a constraint on another type, or for elements from a super type in a snap shot.

 The base information does not carry any information that could not be determined from the path and related profiles, but making this determination requires both that the related profiles are available, and that the algorithm to determine them be available. So they are deformalised into this location for tooling convenience, and to ensure that the base information is available without dependencies. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "base")]
    pub base: Option<ElementDefinitionBase>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_base")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub base_ext: Option<FieldExtension>,
    /** **Reference to definition of content for the element**

 Identifies the identity of an element defined elsewhere in the profile whose content rules should be applied to the current element.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "contentReference")]
    pub content_reference: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_contentReference")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub content_reference_ext: Option<FieldExtension>,
    /** **Data type and Profile for this element**

 The data type or resource that the value of this element is permitted to be.

 The Type of the element can be left blank in a differential constraint, in which case the type is inherited from the resource. Abstract types are not permitted to appear as a type when multiple types are listed.  (I.e. Abstract types cannot be part of a choice). */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "type")]
    pub r#type: Vec<Option<ElementDefinitionType>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub r#type_ext: Vec<Option<FieldExtension>>,
    /** **Specified value if missing from instance**

 The value that should be used if there is no value stated in the instance (e.g. 'if not otherwise specified, the abstract is false').

 Default values can only be specified on a resource, data type, or extension definition, and never in a profile that applies to one of these. Specifying a default value means that the property can never been unknown - it must always have a value. Further, the default value can never be changed. For these reasons, default values are (and should be) used extremely sparingly. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub default_value: Option<ElementDefinitionDefaultValue>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub default_value_ext: Option<ElementDefinitionDefaultValueExtension>,
    /** **Implicit meaning when this element is missing**

 The Implicit meaning that is to be understood when this element is missing (e.g. 'when this element is missing, the period is ongoing'.

 Implicit meanings for missing values can only be specified on a resource, data type, or extension definition, and never in a profile that applies to one of these. An implicit meaning for a missing value can never be changed, and specifying one has the consequence that constraining its use in profiles eliminates use cases as possibilities, not merely moving them out of scope. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "meaningWhenMissing")]
    pub meaning_when_missing: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_meaningWhenMissing")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub meaning_when_missing_ext: Option<FieldExtension>,
    /** **What the order of the elements means**

 If present, indicates that the order of the repeating element has meaning and describes what that meaning is.  If absent, it means that the order of the element has no meaning.

 This element can only be asserted on repeating elements and can only be introduced when defining resources or data types.  It can be further refined profiled elements but if absent in the base type, a profile cannot assert meaning. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "orderMeaning")]
    pub order_meaning: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_orderMeaning")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub order_meaning_ext: Option<FieldExtension>,
    /** **Value must be exactly this**

 Specifies a value that SHALL be exactly the value  for this element in the instance. For purposes of comparison, non-significant whitespace is ignored, and all values must be an exact match (case and accent sensitive). Missing elements/attributes must also be missing.

 This is not recommended for Coding and CodeableConcept since these often have highly contextual properties such as version or display. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub fixed: Option<ElementDefinitionFixed>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub fixed_ext: Option<ElementDefinitionFixedExtension>,
    /** **Value must have at least these property values**

 Specifies a value that the value in the instance SHALL follow - that is, any value in the pattern must be found in the instance. Other additional values may be found too. This is effectively constraint by example.  The values of elements present in the pattern must match exactly (case-sensitive, accent-sensitive, etc.).

 Mostly used for fixing values of CodeableConcept. At present, pattern[x] is not recommended as a basis for slicing while issues related to this are investigated during the STU period. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub pattern: Option<ElementDefinitionPattern>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub pattern_ext: Option<ElementDefinitionPatternExtension>,
    /** **Example value (as defined for type)**

 A sample value for this element demonstrating the type of information that would typically be found in the element.

 Examples will most commonly be present for data where it's not implicitly obvious from either the data type or value set what the values might be.  (I.e. Example values for dates or quantities would generally be unnecessary.)  If the example value is fully populated, the publication tool can generate an instance automatically. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "example")]
    pub example: Vec<Option<ElementDefinitionExample>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_example")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub example_ext: Vec<Option<FieldExtension>>,
    /** **Minimum Allowed Value (for some types)**

 The minimum allowed value for the element. The value is inclusive. This is allowed for the types date, dateTime, instant, time, decimal, integer, and Quantity.

 Except for date/date/instant, the type of the minValue[x] SHALL be the same as the specified type of the element. For the date/dateTime/instant values, the type of minValue[x] SHALL be either the same, or a a [Duration](datatypes.html#duration) which specifies a relative time limit to the current time. The duration value is positive, and is subtracted from the current clock to determine the minimum allowable value.   A minimum value for a Quantity is interpreted as an canonical minimum - e.g. you cannot provide 100mg if the minimum value is 10g. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub min_value: Option<ElementDefinitionMinValue>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub min_value_ext: Option<ElementDefinitionMinValueExtension>,
    /** **Maximum Allowed Value (for some types)**

 The maximum allowed value for the element. The value is inclusive. This is allowed for the types date, dateTime, instant, time, decimal, integer, and Quantity.

 Except for date/date/instant, the type of the maxValue[x] SHALL be the same as the specified type of the element. For the date/dateTime/instant values, the type of maxValue[x] SHALL be either the same, or a a [Duration](datatypes.html#duration) which specifies a relative time limit to the current time. The duration value is positive, and is added to the current clock to determine the maximum allowable value.   A maximum value for a Quantity is interpreted as an canonical maximum - e.g. you cannot provide 10g if the maximum value is 50mg. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub max_value: Option<ElementDefinitionMaxValue>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_value_ext: Option<ElementDefinitionMaxValueExtension>,
    /** **Max length for strings**

 Indicates the maximum length in characters that is permitted to be present in conformant instances and which is expected to be supported by conformant consumers that support the element.

 Receivers are not required to reject instances that exceed the maximum length.  The full length could be stored.  In some cases, data might be truncated, though truncation should be undertaken with care and an understanding of the consequences of doing so. If not specified, there is no conformance expectation for length support. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "maxLength")]
    pub max_length: Option<i32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_maxLength")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_length_ext: Option<FieldExtension>,
    /** **Reference to invariant about presence**

 A reference to an invariant that may make additional statements about the cardinality or value in the instance.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "condition")]
    pub condition: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_condition")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub condition_ext: Vec<Option<FieldExtension>>,
    /** **Condition that must evaluate to true**

 Formal constraints such as co-occurrence and other constraints that can be computationally evaluated within the context of the instance.

 Constraints should be declared on the "context" element - the lowest element in the hierarchy that is common to all nodes referenced by the constraint. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "constraint")]
    pub constraint: Vec<Option<ElementDefinitionConstraint>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_constraint")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub constraint_ext: Vec<Option<FieldExtension>>,
    /** **If the element must supported**

 If true, implementations that produce or consume resources SHALL provide "support" for the element in some meaningful way.  If false, the element may be ignored and not supported.

 "Something useful" is context dependent and impossible to describe in the base FHIR specification. For this reason, tue mustSupport flag is never set to true by the FHIR specification itself - it is only set to true in profiles.

This is done in [Resource Profiles](profiling.html#mustsupport), where the profile labels an element as mustSupport=true. When a profile does this, it SHALL also make clear exactly what kind of "support" is required, as this can mean many things.

Note that an element that has the property IsModifier is not necessarily a "key" element (e.g. one of the important elements to make use of the resource), nor is it automatically mustSupport - however both of these things are more likely to be true for IsModifier elements than for other elements. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "mustSupport")]
    pub must_support: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_mustSupport")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub must_support_ext: Option<FieldExtension>,
    /** **If this modifies the meaning of other elements**

 If true, the value of this element affects the interpretation of the element or resource that contains it, and the value of the element cannot be ignored. Typically, this is used for status, negation and qualification codes. The effect of this is that the element cannot be ignored by systems: they SHALL either recognize the element and process it, and/or a pre-determination has been made that it is not relevant to their particular system.

 Only the definition of an element can set IsModifier true - either the specification itself or where an extension is originally defined. Once set, it cannot be changed in derived profiles. An element/extension that has isModifier=true SHOULD also have a minimum cardinality of 1, so that there is no lack of clarity about what to do if it is missing. If it can be missing, the definition SHALL make the meaning of a missing element clear. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "isModifier")]
    pub is_modifier: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_isModifier")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub is_modifier_ext: Option<FieldExtension>,
    /** **Include when _summary = true?**

 Whether the element should be included if a client requests a search with the parameter _summary=true.

 Some resources include a set of simple metadata, and some very large data. This element is used to reduce the quantity of data returned in searches. Note that servers may pre-cache summarized resources for optimal performance, so servers may not support per-profile use of the isSummary flag. When a request is made with _summary=true, serailisers only include elements marked as 'isSummary = true'. Other than Attachment.data, all data type properties are included in the summary form. Modifier elements or elements with minimum cardinality = 1 must be marked as summary elements. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "isSummary")]
    pub is_summary: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_isSummary")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub is_summary_ext: Option<FieldExtension>,
    /** **ValueSet details if this is coded**

 Binds to a value set if this element is coded (code, Coding, CodeableConcept, Quantity), or the data types (string, uri).

 For a CodeableConcept, when no codes are allowed - only text, use a binding of strength "required" with a description explaining that no coded values are allowed and what sort of information to put in the "text" element. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "binding")]
    pub binding: Option<ElementDefinitionBinding>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_binding")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub binding_ext: Option<FieldExtension>,
    /** **Map element to another set of definitions**

 Identifies a concept from an external specification that roughly corresponds to this element.

 Mappings are not necessarily specific enough for safe translation. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "mapping")]
    pub mapping: Vec<Option<ElementDefinitionMapping>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_mapping")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub mapping_ext: Vec<Option<FieldExtension>>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionBuilder {
    #[doc = concat!("Finalize building ", "ElementDefinition", ".")]
    pub fn build(self) -> Result<ElementDefinition, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ElementDefinitionInner> for ElementDefinition {
    fn from(inner: ElementDefinitionInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for ElementDefinition {
    type Target = ElementDefinitionInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for ElementDefinition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ElementDefinition {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ElementDefinitionBuilder {
        ElementDefinitionBuilder::default()
    }
}
/// Sub-fields of the slicing field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionSlicingBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionSlicing {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Element values that are used to distinguish the slices**

 Designates which child elements are used to discriminate between the slices when processing an instance. If one or more discriminators are provided, the value of the child elements in the instance data SHALL completely distinguish which slice the element in the resource matches based on the allowed values for those elements in each of the slices.

 If there is no discriminator, the content is hard to process, so this should be avoided. If the base element has a cardinality of ..1, and there is a choice of types, the discriminator must be "@type". */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "discriminator")]
    pub discriminator: Vec<Option<ElementDefinitionSlicingDiscriminator>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_discriminator")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub discriminator_ext: Vec<Option<FieldExtension>>,
    /** **Text description of how slicing works (or not)**

 A human-readable text description of how the slicing works. If there is no discriminator, this is required to be present to provide whatever information is possible about how the slices can be differentiated.

 If it's really not possible to differentiate them, the design should be re-evaluated to make the content usable. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_description")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub description_ext: Option<FieldExtension>,
    /** **If elements must be in same order as slices**

 If the matching elements have to occur in the same order as defined in the profile.

 Order should only be required when it is a pressing concern for presentation. Profile authors should consider making the order a feature of the rules about the narrative, not the rules about the data - requiring ordered data makes the profile much less re-usable. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "ordered")]
    pub ordered: Option<bool>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_ordered")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub ordered_ext: Option<FieldExtension>,
    /** **[SlicingRules](http://hl7.org/fhir/ValueSet/resource-slicing-rules); closed | open | openAtEnd**

 Whether additional slices are allowed or not. When the slices are ordered, profile authors can also say that additional slices are only allowed at the end.

 Allowing additional elements makes for a much for flexible template - it's open for use in wider contexts, but also means that the content of the resource is not closed, and applications have to decide how to handle content not described by the profile. */
    #[serde(rename = "rules")]
    pub rules: codes::SlicingRules,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_rules")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub rules_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionSlicing {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionSlicingBuilder {
        ElementDefinitionSlicingBuilder::default()
    }
}
/// Sub-fields of the discriminator field in ElementDefinitionSlicing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionSlicingDiscriminatorBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionSlicingDiscriminator {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[DiscriminatorType](http://hl7.org/fhir/ValueSet/discriminator-type); value | exists | pattern | type | profile**

 How the element value is interpreted when discrimination is evaluated.

 */
    #[serde(rename = "type")]
    pub r#type: codes::DiscriminatorType,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **Path to element value**

 A FHIRPath expression, using a restricted subset of FHIRPath, that is used to identify the element on which discrimination is based.

 The only FHIRPath functions that are allowed are resolve(), and extension(url). */
    #[serde(rename = "path")]
    pub path: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_path")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub path_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionSlicingDiscriminator {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionSlicingDiscriminatorBuilder {
        ElementDefinitionSlicingDiscriminatorBuilder::default()
    }
}
/// Sub-fields of the base field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionBaseBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionBase {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Path that identifies the base element**

 The Path that identifies the base element - this matches the ElementDefinition.path for that element. Across FHIR, there is only one base definition of any element - that is, an element definition on a [StructureDefinition](structuredefinition.html#) without a StructureDefinition.base.

 */
    #[serde(rename = "path")]
    pub path: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_path")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub path_ext: Option<FieldExtension>,
    /** **Min cardinality of the base element**

 Minimum cardinality of the base element identified by the path.

 This is provided for consistency with max, and may affect code generation of mandatory elements of the base resource are generated differently (some reference implementations have done this). */
    #[serde(rename = "min")]
    pub min: u32,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_min")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub min_ext: Option<FieldExtension>,
    /** **Max cardinality of the base element**

 Maximum cardinality of the base element identified by the path.

 This is provided to code generation, since the serialization representation in JSON differs depending on whether the base element has max > 1. Also, some forms of code generation may differ. */
    #[serde(rename = "max")]
    pub max: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_max")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionBase {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionBaseBuilder {
        ElementDefinitionBaseBuilder::default()
    }
}
/// Sub-fields of the type field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionTypeBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionType {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[FHIRDefinedTypeExt](http://hl7.org/fhir/ValueSet/defined-types); Data type or Resource (reference to definition)**

 URL of Data type or Resource that is a(or the) type used for this element. References are URLs that are relative to http://hl7.org/fhir/StructureDefinition e.g. "string" is a reference to http://hl7.org/fhir/StructureDefinition/string. Absolute URLs are only allowed in logical models.

 If the element is a reference to another resource, this element contains "Reference", and the targetProfile element defines what resources can be referenced. The targetProfile may be a reference to the general definition of a resource (e.g. http://hl7.org/fhir/StructureDefinition/Patient). There would be one pair of type/code for each allowed target resource type. */
    #[serde(rename = "code")]
    pub code: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_ext: Option<FieldExtension>,
    /** **Profile (StructureDefinition) to apply (or IG)**

 Identifies a profile structure or implementation Guide that SHALL hold for the datatype this element refers to. Can be a local reference - to a contained StructureDefinition, or a reference to another StructureDefinition or Implementation Guide by a canonical URL. When an implementation guide is specified, the resource SHALL conform to at least one profile defined in the implementation guide.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "profile")]
    pub profile: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_profile")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub profile_ext: Option<FieldExtension>,
    /** **Profile (StructureDefinition) to apply to reference target (or IG)**

 Identifies a profile structure or implementation Guide that SHALL hold for the target of the reference this element refers to. Can be a local reference - to a contained StructureDefinition, or a reference to another StructureDefinition or Implementation Guide by a canonical URL. When an implementation guide is specified, the resource SHALL conform to at least one profile defined in the implementation guide.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "targetProfile")]
    pub target_profile: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_targetProfile")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub target_profile_ext: Option<FieldExtension>,
    /** **[AggregationMode](http://hl7.org/fhir/ValueSet/resource-aggregation-mode); contained | referenced | bundled - how aggregated**

 If the type is a reference to another resource, how the resource is or can be aggregated - is it a contained resource, or a reference, and if the context is a bundle, is it included in the bundle.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "aggregation")]
    pub aggregation: Vec<Option<codes::AggregationMode>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_aggregation")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub aggregation_ext: Vec<Option<FieldExtension>>,
    /** **[ReferenceVersionRules](http://hl7.org/fhir/ValueSet/reference-version-rules); either | independent | specific**

 Whether this reference needs to be version specific or version independent, or whether either can be used.

 The base specification never makes a rule as to which form is allowed, but implementation guides may do this. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "versioning")]
    pub versioning: Option<codes::ReferenceVersionRules>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_versioning")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub versioning_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionType {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionTypeBuilder {
        ElementDefinitionTypeBuilder::default()
    }
}
/// Choice of types for the defaultValue[x] field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionDefaultValue {
    /// Variant accepting the Base64Binary type.
    #[serde(rename = "defaultValueBase64Binary")]
    Base64Binary(Base64Binary),
    /// Variant accepting the Boolean type.
    #[serde(rename = "defaultValueBoolean")]
    Boolean(bool),
    /// Variant accepting the Code type.
    #[serde(rename = "defaultValueCode")]
    Code(String),
    /// Variant accepting the Date type.
    #[serde(rename = "defaultValueDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "defaultValueDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Decimal type.
    #[serde(rename = "defaultValueDecimal")]
    Decimal(f64),
    /// Variant accepting the Id type.
    #[serde(rename = "defaultValueId")]
    Id(String),
    /// Variant accepting the Instant type.
    #[serde(rename = "defaultValueInstant")]
    Instant(Instant),
    /// Variant accepting the Integer type.
    #[serde(rename = "defaultValueInteger")]
    Integer(i32),
    /// Variant accepting the Markdown type.
    #[serde(rename = "defaultValueMarkdown")]
    Markdown(String),
    /// Variant accepting the Oid type.
    #[serde(rename = "defaultValueOid")]
    Oid(String),
    /// Variant accepting the PositiveInt type.
    #[serde(rename = "defaultValuePositiveInt")]
    PositiveInt(NonZeroU32),
    /// Variant accepting the String type.
    #[serde(rename = "defaultValueString")]
    String(String),
    /// Variant accepting the Time type.
    #[serde(rename = "defaultValueTime")]
    Time(Time),
    /// Variant accepting the UnsignedInt type.
    #[serde(rename = "defaultValueUnsignedInt")]
    UnsignedInt(u32),
    /// Variant accepting the Uri type.
    #[serde(rename = "defaultValueUri")]
    Uri(String),
    /// Variant accepting the Address type.
    #[serde(rename = "defaultValueAddress")]
    Address(Address),
    /// Variant accepting the Age type.
    #[serde(rename = "defaultValueAge")]
    Age(Age),
    /// Variant accepting the Annotation type.
    #[serde(rename = "defaultValueAnnotation")]
    Annotation(Annotation),
    /// Variant accepting the Attachment type.
    #[serde(rename = "defaultValueAttachment")]
    Attachment(Attachment),
    /// Variant accepting the CodeableConcept type.
    #[serde(rename = "defaultValueCodeableConcept")]
    CodeableConcept(CodeableConcept),
    /// Variant accepting the Coding type.
    #[serde(rename = "defaultValueCoding")]
    Coding(Coding),
    /// Variant accepting the ContactPoint type.
    #[serde(rename = "defaultValueContactPoint")]
    ContactPoint(ContactPoint),
    /// Variant accepting the Count type.
    #[serde(rename = "defaultValueCount")]
    Count(Count),
    /// Variant accepting the Distance type.
    #[serde(rename = "defaultValueDistance")]
    Distance(Distance),
    /// Variant accepting the Duration type.
    #[serde(rename = "defaultValueDuration")]
    Duration(Duration),
    /// Variant accepting the HumanName type.
    #[serde(rename = "defaultValueHumanName")]
    HumanName(HumanName),
    /// Variant accepting the Identifier type.
    #[serde(rename = "defaultValueIdentifier")]
    Identifier(Identifier),
    /// Variant accepting the Money type.
    #[serde(rename = "defaultValueMoney")]
    Money(Money),
    /// Variant accepting the Period type.
    #[serde(rename = "defaultValuePeriod")]
    Period(Period),
    /// Variant accepting the Quantity type.
    #[serde(rename = "defaultValueQuantity")]
    Quantity(Quantity),
    /// Variant accepting the Range type.
    #[serde(rename = "defaultValueRange")]
    Range(Range),
    /// Variant accepting the Ratio type.
    #[serde(rename = "defaultValueRatio")]
    Ratio(Ratio),
    /// Variant accepting the Reference type.
    #[serde(rename = "defaultValueReference")]
    Reference(Reference),
    /// Variant accepting the SampledData type.
    #[serde(rename = "defaultValueSampledData")]
    SampledData(SampledData),
    /// Variant accepting the Signature type.
    #[serde(rename = "defaultValueSignature")]
    Signature(Signature),
    /// Variant accepting the Timing type.
    #[serde(rename = "defaultValueTiming")]
    Timing(Timing),
    /// Variant accepting the Meta type.
    #[serde(rename = "defaultValueMeta")]
    Meta(Meta),
}
/// Extension value for ElementDefinitionDefaultValue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionDefaultValueExtension {
    /// Variant accepting the Base64Binary extension.
    #[serde(rename = "_defaultValueBase64Binary")]
    Base64Binary(FieldExtension),
    /// Variant accepting the Boolean extension.
    #[serde(rename = "_defaultValueBoolean")]
    Boolean(FieldExtension),
    /// Variant accepting the Code extension.
    #[serde(rename = "_defaultValueCode")]
    Code(FieldExtension),
    /// Variant accepting the Date extension.
    #[serde(rename = "_defaultValueDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_defaultValueDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Decimal extension.
    #[serde(rename = "_defaultValueDecimal")]
    Decimal(FieldExtension),
    /// Variant accepting the Id extension.
    #[serde(rename = "_defaultValueId")]
    Id(FieldExtension),
    /// Variant accepting the Instant extension.
    #[serde(rename = "_defaultValueInstant")]
    Instant(FieldExtension),
    /// Variant accepting the Integer extension.
    #[serde(rename = "_defaultValueInteger")]
    Integer(FieldExtension),
    /// Variant accepting the Markdown extension.
    #[serde(rename = "_defaultValueMarkdown")]
    Markdown(FieldExtension),
    /// Variant accepting the Oid extension.
    #[serde(rename = "_defaultValueOid")]
    Oid(FieldExtension),
    /// Variant accepting the PositiveInt extension.
    #[serde(rename = "_defaultValuePositiveInt")]
    PositiveInt(FieldExtension),
    /// Variant accepting the String extension.
    #[serde(rename = "_defaultValueString")]
    String(FieldExtension),
    /// Variant accepting the Time extension.
    #[serde(rename = "_defaultValueTime")]
    Time(FieldExtension),
    /// Variant accepting the UnsignedInt extension.
    #[serde(rename = "_defaultValueUnsignedInt")]
    UnsignedInt(FieldExtension),
    /// Variant accepting the Uri extension.
    #[serde(rename = "_defaultValueUri")]
    Uri(FieldExtension),
    /// Variant accepting the Address extension.
    #[serde(rename = "_defaultValueAddress")]
    Address(FieldExtension),
    /// Variant accepting the Age extension.
    #[serde(rename = "_defaultValueAge")]
    Age(FieldExtension),
    /// Variant accepting the Annotation extension.
    #[serde(rename = "_defaultValueAnnotation")]
    Annotation(FieldExtension),
    /// Variant accepting the Attachment extension.
    #[serde(rename = "_defaultValueAttachment")]
    Attachment(FieldExtension),
    /// Variant accepting the CodeableConcept extension.
    #[serde(rename = "_defaultValueCodeableConcept")]
    CodeableConcept(FieldExtension),
    /// Variant accepting the Coding extension.
    #[serde(rename = "_defaultValueCoding")]
    Coding(FieldExtension),
    /// Variant accepting the ContactPoint extension.
    #[serde(rename = "_defaultValueContactPoint")]
    ContactPoint(FieldExtension),
    /// Variant accepting the Count extension.
    #[serde(rename = "_defaultValueCount")]
    Count(FieldExtension),
    /// Variant accepting the Distance extension.
    #[serde(rename = "_defaultValueDistance")]
    Distance(FieldExtension),
    /// Variant accepting the Duration extension.
    #[serde(rename = "_defaultValueDuration")]
    Duration(FieldExtension),
    /// Variant accepting the HumanName extension.
    #[serde(rename = "_defaultValueHumanName")]
    HumanName(FieldExtension),
    /// Variant accepting the Identifier extension.
    #[serde(rename = "_defaultValueIdentifier")]
    Identifier(FieldExtension),
    /// Variant accepting the Money extension.
    #[serde(rename = "_defaultValueMoney")]
    Money(FieldExtension),
    /// Variant accepting the Period extension.
    #[serde(rename = "_defaultValuePeriod")]
    Period(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_defaultValueQuantity")]
    Quantity(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_defaultValueRange")]
    Range(FieldExtension),
    /// Variant accepting the Ratio extension.
    #[serde(rename = "_defaultValueRatio")]
    Ratio(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_defaultValueReference")]
    Reference(FieldExtension),
    /// Variant accepting the SampledData extension.
    #[serde(rename = "_defaultValueSampledData")]
    SampledData(FieldExtension),
    /// Variant accepting the Signature extension.
    #[serde(rename = "_defaultValueSignature")]
    Signature(FieldExtension),
    /// Variant accepting the Timing extension.
    #[serde(rename = "_defaultValueTiming")]
    Timing(FieldExtension),
    /// Variant accepting the Meta extension.
    #[serde(rename = "_defaultValueMeta")]
    Meta(FieldExtension),
}
/// Choice of types for the fixed[x] field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionFixed {
    /// Variant accepting the Base64Binary type.
    #[serde(rename = "fixedBase64Binary")]
    Base64Binary(Base64Binary),
    /// Variant accepting the Boolean type.
    #[serde(rename = "fixedBoolean")]
    Boolean(bool),
    /// Variant accepting the Code type.
    #[serde(rename = "fixedCode")]
    Code(String),
    /// Variant accepting the Date type.
    #[serde(rename = "fixedDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "fixedDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Decimal type.
    #[serde(rename = "fixedDecimal")]
    Decimal(f64),
    /// Variant accepting the Id type.
    #[serde(rename = "fixedId")]
    Id(String),
    /// Variant accepting the Instant type.
    #[serde(rename = "fixedInstant")]
    Instant(Instant),
    /// Variant accepting the Integer type.
    #[serde(rename = "fixedInteger")]
    Integer(i32),
    /// Variant accepting the Markdown type.
    #[serde(rename = "fixedMarkdown")]
    Markdown(String),
    /// Variant accepting the Oid type.
    #[serde(rename = "fixedOid")]
    Oid(String),
    /// Variant accepting the PositiveInt type.
    #[serde(rename = "fixedPositiveInt")]
    PositiveInt(NonZeroU32),
    /// Variant accepting the String type.
    #[serde(rename = "fixedString")]
    String(String),
    /// Variant accepting the Time type.
    #[serde(rename = "fixedTime")]
    Time(Time),
    /// Variant accepting the UnsignedInt type.
    #[serde(rename = "fixedUnsignedInt")]
    UnsignedInt(u32),
    /// Variant accepting the Uri type.
    #[serde(rename = "fixedUri")]
    Uri(String),
    /// Variant accepting the Address type.
    #[serde(rename = "fixedAddress")]
    Address(Address),
    /// Variant accepting the Age type.
    #[serde(rename = "fixedAge")]
    Age(Age),
    /// Variant accepting the Annotation type.
    #[serde(rename = "fixedAnnotation")]
    Annotation(Annotation),
    /// Variant accepting the Attachment type.
    #[serde(rename = "fixedAttachment")]
    Attachment(Attachment),
    /// Variant accepting the CodeableConcept type.
    #[serde(rename = "fixedCodeableConcept")]
    CodeableConcept(CodeableConcept),
    /// Variant accepting the Coding type.
    #[serde(rename = "fixedCoding")]
    Coding(Coding),
    /// Variant accepting the ContactPoint type.
    #[serde(rename = "fixedContactPoint")]
    ContactPoint(ContactPoint),
    /// Variant accepting the Count type.
    #[serde(rename = "fixedCount")]
    Count(Count),
    /// Variant accepting the Distance type.
    #[serde(rename = "fixedDistance")]
    Distance(Distance),
    /// Variant accepting the Duration type.
    #[serde(rename = "fixedDuration")]
    Duration(Duration),
    /// Variant accepting the HumanName type.
    #[serde(rename = "fixedHumanName")]
    HumanName(HumanName),
    /// Variant accepting the Identifier type.
    #[serde(rename = "fixedIdentifier")]
    Identifier(Identifier),
    /// Variant accepting the Money type.
    #[serde(rename = "fixedMoney")]
    Money(Money),
    /// Variant accepting the Period type.
    #[serde(rename = "fixedPeriod")]
    Period(Period),
    /// Variant accepting the Quantity type.
    #[serde(rename = "fixedQuantity")]
    Quantity(Quantity),
    /// Variant accepting the Range type.
    #[serde(rename = "fixedRange")]
    Range(Range),
    /// Variant accepting the Ratio type.
    #[serde(rename = "fixedRatio")]
    Ratio(Ratio),
    /// Variant accepting the Reference type.
    #[serde(rename = "fixedReference")]
    Reference(Reference),
    /// Variant accepting the SampledData type.
    #[serde(rename = "fixedSampledData")]
    SampledData(SampledData),
    /// Variant accepting the Signature type.
    #[serde(rename = "fixedSignature")]
    Signature(Signature),
    /// Variant accepting the Timing type.
    #[serde(rename = "fixedTiming")]
    Timing(Timing),
    /// Variant accepting the Meta type.
    #[serde(rename = "fixedMeta")]
    Meta(Meta),
}
/// Extension value for ElementDefinitionFixed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionFixedExtension {
    /// Variant accepting the Base64Binary extension.
    #[serde(rename = "_fixedBase64Binary")]
    Base64Binary(FieldExtension),
    /// Variant accepting the Boolean extension.
    #[serde(rename = "_fixedBoolean")]
    Boolean(FieldExtension),
    /// Variant accepting the Code extension.
    #[serde(rename = "_fixedCode")]
    Code(FieldExtension),
    /// Variant accepting the Date extension.
    #[serde(rename = "_fixedDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_fixedDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Decimal extension.
    #[serde(rename = "_fixedDecimal")]
    Decimal(FieldExtension),
    /// Variant accepting the Id extension.
    #[serde(rename = "_fixedId")]
    Id(FieldExtension),
    /// Variant accepting the Instant extension.
    #[serde(rename = "_fixedInstant")]
    Instant(FieldExtension),
    /// Variant accepting the Integer extension.
    #[serde(rename = "_fixedInteger")]
    Integer(FieldExtension),
    /// Variant accepting the Markdown extension.
    #[serde(rename = "_fixedMarkdown")]
    Markdown(FieldExtension),
    /// Variant accepting the Oid extension.
    #[serde(rename = "_fixedOid")]
    Oid(FieldExtension),
    /// Variant accepting the PositiveInt extension.
    #[serde(rename = "_fixedPositiveInt")]
    PositiveInt(FieldExtension),
    /// Variant accepting the String extension.
    #[serde(rename = "_fixedString")]
    String(FieldExtension),
    /// Variant accepting the Time extension.
    #[serde(rename = "_fixedTime")]
    Time(FieldExtension),
    /// Variant accepting the UnsignedInt extension.
    #[serde(rename = "_fixedUnsignedInt")]
    UnsignedInt(FieldExtension),
    /// Variant accepting the Uri extension.
    #[serde(rename = "_fixedUri")]
    Uri(FieldExtension),
    /// Variant accepting the Address extension.
    #[serde(rename = "_fixedAddress")]
    Address(FieldExtension),
    /// Variant accepting the Age extension.
    #[serde(rename = "_fixedAge")]
    Age(FieldExtension),
    /// Variant accepting the Annotation extension.
    #[serde(rename = "_fixedAnnotation")]
    Annotation(FieldExtension),
    /// Variant accepting the Attachment extension.
    #[serde(rename = "_fixedAttachment")]
    Attachment(FieldExtension),
    /// Variant accepting the CodeableConcept extension.
    #[serde(rename = "_fixedCodeableConcept")]
    CodeableConcept(FieldExtension),
    /// Variant accepting the Coding extension.
    #[serde(rename = "_fixedCoding")]
    Coding(FieldExtension),
    /// Variant accepting the ContactPoint extension.
    #[serde(rename = "_fixedContactPoint")]
    ContactPoint(FieldExtension),
    /// Variant accepting the Count extension.
    #[serde(rename = "_fixedCount")]
    Count(FieldExtension),
    /// Variant accepting the Distance extension.
    #[serde(rename = "_fixedDistance")]
    Distance(FieldExtension),
    /// Variant accepting the Duration extension.
    #[serde(rename = "_fixedDuration")]
    Duration(FieldExtension),
    /// Variant accepting the HumanName extension.
    #[serde(rename = "_fixedHumanName")]
    HumanName(FieldExtension),
    /// Variant accepting the Identifier extension.
    #[serde(rename = "_fixedIdentifier")]
    Identifier(FieldExtension),
    /// Variant accepting the Money extension.
    #[serde(rename = "_fixedMoney")]
    Money(FieldExtension),
    /// Variant accepting the Period extension.
    #[serde(rename = "_fixedPeriod")]
    Period(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_fixedQuantity")]
    Quantity(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_fixedRange")]
    Range(FieldExtension),
    /// Variant accepting the Ratio extension.
    #[serde(rename = "_fixedRatio")]
    Ratio(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_fixedReference")]
    Reference(FieldExtension),
    /// Variant accepting the SampledData extension.
    #[serde(rename = "_fixedSampledData")]
    SampledData(FieldExtension),
    /// Variant accepting the Signature extension.
    #[serde(rename = "_fixedSignature")]
    Signature(FieldExtension),
    /// Variant accepting the Timing extension.
    #[serde(rename = "_fixedTiming")]
    Timing(FieldExtension),
    /// Variant accepting the Meta extension.
    #[serde(rename = "_fixedMeta")]
    Meta(FieldExtension),
}
/// Choice of types for the pattern[x] field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionPattern {
    /// Variant accepting the Base64Binary type.
    #[serde(rename = "patternBase64Binary")]
    Base64Binary(Base64Binary),
    /// Variant accepting the Boolean type.
    #[serde(rename = "patternBoolean")]
    Boolean(bool),
    /// Variant accepting the Code type.
    #[serde(rename = "patternCode")]
    Code(String),
    /// Variant accepting the Date type.
    #[serde(rename = "patternDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "patternDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Decimal type.
    #[serde(rename = "patternDecimal")]
    Decimal(f64),
    /// Variant accepting the Id type.
    #[serde(rename = "patternId")]
    Id(String),
    /// Variant accepting the Instant type.
    #[serde(rename = "patternInstant")]
    Instant(Instant),
    /// Variant accepting the Integer type.
    #[serde(rename = "patternInteger")]
    Integer(i32),
    /// Variant accepting the Markdown type.
    #[serde(rename = "patternMarkdown")]
    Markdown(String),
    /// Variant accepting the Oid type.
    #[serde(rename = "patternOid")]
    Oid(String),
    /// Variant accepting the PositiveInt type.
    #[serde(rename = "patternPositiveInt")]
    PositiveInt(NonZeroU32),
    /// Variant accepting the String type.
    #[serde(rename = "patternString")]
    String(String),
    /// Variant accepting the Time type.
    #[serde(rename = "patternTime")]
    Time(Time),
    /// Variant accepting the UnsignedInt type.
    #[serde(rename = "patternUnsignedInt")]
    UnsignedInt(u32),
    /// Variant accepting the Uri type.
    #[serde(rename = "patternUri")]
    Uri(String),
    /// Variant accepting the Address type.
    #[serde(rename = "patternAddress")]
    Address(Address),
    /// Variant accepting the Age type.
    #[serde(rename = "patternAge")]
    Age(Age),
    /// Variant accepting the Annotation type.
    #[serde(rename = "patternAnnotation")]
    Annotation(Annotation),
    /// Variant accepting the Attachment type.
    #[serde(rename = "patternAttachment")]
    Attachment(Attachment),
    /// Variant accepting the CodeableConcept type.
    #[serde(rename = "patternCodeableConcept")]
    CodeableConcept(CodeableConcept),
    /// Variant accepting the Coding type.
    #[serde(rename = "patternCoding")]
    Coding(Coding),
    /// Variant accepting the ContactPoint type.
    #[serde(rename = "patternContactPoint")]
    ContactPoint(ContactPoint),
    /// Variant accepting the Count type.
    #[serde(rename = "patternCount")]
    Count(Count),
    /// Variant accepting the Distance type.
    #[serde(rename = "patternDistance")]
    Distance(Distance),
    /// Variant accepting the Duration type.
    #[serde(rename = "patternDuration")]
    Duration(Duration),
    /// Variant accepting the HumanName type.
    #[serde(rename = "patternHumanName")]
    HumanName(HumanName),
    /// Variant accepting the Identifier type.
    #[serde(rename = "patternIdentifier")]
    Identifier(Identifier),
    /// Variant accepting the Money type.
    #[serde(rename = "patternMoney")]
    Money(Money),
    /// Variant accepting the Period type.
    #[serde(rename = "patternPeriod")]
    Period(Period),
    /// Variant accepting the Quantity type.
    #[serde(rename = "patternQuantity")]
    Quantity(Quantity),
    /// Variant accepting the Range type.
    #[serde(rename = "patternRange")]
    Range(Range),
    /// Variant accepting the Ratio type.
    #[serde(rename = "patternRatio")]
    Ratio(Ratio),
    /// Variant accepting the Reference type.
    #[serde(rename = "patternReference")]
    Reference(Reference),
    /// Variant accepting the SampledData type.
    #[serde(rename = "patternSampledData")]
    SampledData(SampledData),
    /// Variant accepting the Signature type.
    #[serde(rename = "patternSignature")]
    Signature(Signature),
    /// Variant accepting the Timing type.
    #[serde(rename = "patternTiming")]
    Timing(Timing),
    /// Variant accepting the Meta type.
    #[serde(rename = "patternMeta")]
    Meta(Meta),
}
/// Extension value for ElementDefinitionPattern.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionPatternExtension {
    /// Variant accepting the Base64Binary extension.
    #[serde(rename = "_patternBase64Binary")]
    Base64Binary(FieldExtension),
    /// Variant accepting the Boolean extension.
    #[serde(rename = "_patternBoolean")]
    Boolean(FieldExtension),
    /// Variant accepting the Code extension.
    #[serde(rename = "_patternCode")]
    Code(FieldExtension),
    /// Variant accepting the Date extension.
    #[serde(rename = "_patternDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_patternDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Decimal extension.
    #[serde(rename = "_patternDecimal")]
    Decimal(FieldExtension),
    /// Variant accepting the Id extension.
    #[serde(rename = "_patternId")]
    Id(FieldExtension),
    /// Variant accepting the Instant extension.
    #[serde(rename = "_patternInstant")]
    Instant(FieldExtension),
    /// Variant accepting the Integer extension.
    #[serde(rename = "_patternInteger")]
    Integer(FieldExtension),
    /// Variant accepting the Markdown extension.
    #[serde(rename = "_patternMarkdown")]
    Markdown(FieldExtension),
    /// Variant accepting the Oid extension.
    #[serde(rename = "_patternOid")]
    Oid(FieldExtension),
    /// Variant accepting the PositiveInt extension.
    #[serde(rename = "_patternPositiveInt")]
    PositiveInt(FieldExtension),
    /// Variant accepting the String extension.
    #[serde(rename = "_patternString")]
    String(FieldExtension),
    /// Variant accepting the Time extension.
    #[serde(rename = "_patternTime")]
    Time(FieldExtension),
    /// Variant accepting the UnsignedInt extension.
    #[serde(rename = "_patternUnsignedInt")]
    UnsignedInt(FieldExtension),
    /// Variant accepting the Uri extension.
    #[serde(rename = "_patternUri")]
    Uri(FieldExtension),
    /// Variant accepting the Address extension.
    #[serde(rename = "_patternAddress")]
    Address(FieldExtension),
    /// Variant accepting the Age extension.
    #[serde(rename = "_patternAge")]
    Age(FieldExtension),
    /// Variant accepting the Annotation extension.
    #[serde(rename = "_patternAnnotation")]
    Annotation(FieldExtension),
    /// Variant accepting the Attachment extension.
    #[serde(rename = "_patternAttachment")]
    Attachment(FieldExtension),
    /// Variant accepting the CodeableConcept extension.
    #[serde(rename = "_patternCodeableConcept")]
    CodeableConcept(FieldExtension),
    /// Variant accepting the Coding extension.
    #[serde(rename = "_patternCoding")]
    Coding(FieldExtension),
    /// Variant accepting the ContactPoint extension.
    #[serde(rename = "_patternContactPoint")]
    ContactPoint(FieldExtension),
    /// Variant accepting the Count extension.
    #[serde(rename = "_patternCount")]
    Count(FieldExtension),
    /// Variant accepting the Distance extension.
    #[serde(rename = "_patternDistance")]
    Distance(FieldExtension),
    /// Variant accepting the Duration extension.
    #[serde(rename = "_patternDuration")]
    Duration(FieldExtension),
    /// Variant accepting the HumanName extension.
    #[serde(rename = "_patternHumanName")]
    HumanName(FieldExtension),
    /// Variant accepting the Identifier extension.
    #[serde(rename = "_patternIdentifier")]
    Identifier(FieldExtension),
    /// Variant accepting the Money extension.
    #[serde(rename = "_patternMoney")]
    Money(FieldExtension),
    /// Variant accepting the Period extension.
    #[serde(rename = "_patternPeriod")]
    Period(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_patternQuantity")]
    Quantity(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_patternRange")]
    Range(FieldExtension),
    /// Variant accepting the Ratio extension.
    #[serde(rename = "_patternRatio")]
    Ratio(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_patternReference")]
    Reference(FieldExtension),
    /// Variant accepting the SampledData extension.
    #[serde(rename = "_patternSampledData")]
    SampledData(FieldExtension),
    /// Variant accepting the Signature extension.
    #[serde(rename = "_patternSignature")]
    Signature(FieldExtension),
    /// Variant accepting the Timing extension.
    #[serde(rename = "_patternTiming")]
    Timing(FieldExtension),
    /// Variant accepting the Meta extension.
    #[serde(rename = "_patternMeta")]
    Meta(FieldExtension),
}
/// Sub-fields of the example field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionExampleBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionExample {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Describes the purpose of this example**

 Describes the purpose of this example amoung the set of examples.

 */
    #[serde(rename = "label")]
    pub label: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_label")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub label_ext: Option<FieldExtension>,
    /** **Value of Example (one of allowed types)**

 The actual value for the element, which must be one of the types allowed for this element.

 */
    #[serde(flatten)]
    pub value: ElementDefinitionExampleValue,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<ElementDefinitionExampleValueExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionExample {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionExampleBuilder {
        ElementDefinitionExampleBuilder::default()
    }
}
/// Choice of types for the value[x] field in ElementDefinitionExample
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionExampleValue {
    /// Variant accepting the Base64Binary type.
    #[serde(rename = "valueBase64Binary")]
    Base64Binary(Base64Binary),
    /// Variant accepting the Boolean type.
    #[serde(rename = "valueBoolean")]
    Boolean(bool),
    /// Variant accepting the Code type.
    #[serde(rename = "valueCode")]
    Code(String),
    /// Variant accepting the Date type.
    #[serde(rename = "valueDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "valueDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Decimal type.
    #[serde(rename = "valueDecimal")]
    Decimal(f64),
    /// Variant accepting the Id type.
    #[serde(rename = "valueId")]
    Id(String),
    /// Variant accepting the Instant type.
    #[serde(rename = "valueInstant")]
    Instant(Instant),
    /// Variant accepting the Integer type.
    #[serde(rename = "valueInteger")]
    Integer(i32),
    /// Variant accepting the Markdown type.
    #[serde(rename = "valueMarkdown")]
    Markdown(String),
    /// Variant accepting the Oid type.
    #[serde(rename = "valueOid")]
    Oid(String),
    /// Variant accepting the PositiveInt type.
    #[serde(rename = "valuePositiveInt")]
    PositiveInt(NonZeroU32),
    /// Variant accepting the String type.
    #[serde(rename = "valueString")]
    String(String),
    /// Variant accepting the Time type.
    #[serde(rename = "valueTime")]
    Time(Time),
    /// Variant accepting the UnsignedInt type.
    #[serde(rename = "valueUnsignedInt")]
    UnsignedInt(u32),
    /// Variant accepting the Uri type.
    #[serde(rename = "valueUri")]
    Uri(String),
    /// Variant accepting the Address type.
    #[serde(rename = "valueAddress")]
    Address(Address),
    /// Variant accepting the Age type.
    #[serde(rename = "valueAge")]
    Age(Age),
    /// Variant accepting the Annotation type.
    #[serde(rename = "valueAnnotation")]
    Annotation(Annotation),
    /// Variant accepting the Attachment type.
    #[serde(rename = "valueAttachment")]
    Attachment(Attachment),
    /// Variant accepting the CodeableConcept type.
    #[serde(rename = "valueCodeableConcept")]
    CodeableConcept(CodeableConcept),
    /// Variant accepting the Coding type.
    #[serde(rename = "valueCoding")]
    Coding(Coding),
    /// Variant accepting the ContactPoint type.
    #[serde(rename = "valueContactPoint")]
    ContactPoint(ContactPoint),
    /// Variant accepting the Count type.
    #[serde(rename = "valueCount")]
    Count(Count),
    /// Variant accepting the Distance type.
    #[serde(rename = "valueDistance")]
    Distance(Distance),
    /// Variant accepting the Duration type.
    #[serde(rename = "valueDuration")]
    Duration(Duration),
    /// Variant accepting the HumanName type.
    #[serde(rename = "valueHumanName")]
    HumanName(HumanName),
    /// Variant accepting the Identifier type.
    #[serde(rename = "valueIdentifier")]
    Identifier(Identifier),
    /// Variant accepting the Money type.
    #[serde(rename = "valueMoney")]
    Money(Money),
    /// Variant accepting the Period type.
    #[serde(rename = "valuePeriod")]
    Period(Period),
    /// Variant accepting the Quantity type.
    #[serde(rename = "valueQuantity")]
    Quantity(Quantity),
    /// Variant accepting the Range type.
    #[serde(rename = "valueRange")]
    Range(Range),
    /// Variant accepting the Ratio type.
    #[serde(rename = "valueRatio")]
    Ratio(Ratio),
    /// Variant accepting the Reference type.
    #[serde(rename = "valueReference")]
    Reference(Reference),
    /// Variant accepting the SampledData type.
    #[serde(rename = "valueSampledData")]
    SampledData(SampledData),
    /// Variant accepting the Signature type.
    #[serde(rename = "valueSignature")]
    Signature(Signature),
    /// Variant accepting the Timing type.
    #[serde(rename = "valueTiming")]
    Timing(Timing),
    /// Variant accepting the Meta type.
    #[serde(rename = "valueMeta")]
    Meta(Meta),
}
/// Extension value for ElementDefinitionExampleValue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionExampleValueExtension {
    /// Variant accepting the Base64Binary extension.
    #[serde(rename = "_valueBase64Binary")]
    Base64Binary(FieldExtension),
    /// Variant accepting the Boolean extension.
    #[serde(rename = "_valueBoolean")]
    Boolean(FieldExtension),
    /// Variant accepting the Code extension.
    #[serde(rename = "_valueCode")]
    Code(FieldExtension),
    /// Variant accepting the Date extension.
    #[serde(rename = "_valueDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_valueDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Decimal extension.
    #[serde(rename = "_valueDecimal")]
    Decimal(FieldExtension),
    /// Variant accepting the Id extension.
    #[serde(rename = "_valueId")]
    Id(FieldExtension),
    /// Variant accepting the Instant extension.
    #[serde(rename = "_valueInstant")]
    Instant(FieldExtension),
    /// Variant accepting the Integer extension.
    #[serde(rename = "_valueInteger")]
    Integer(FieldExtension),
    /// Variant accepting the Markdown extension.
    #[serde(rename = "_valueMarkdown")]
    Markdown(FieldExtension),
    /// Variant accepting the Oid extension.
    #[serde(rename = "_valueOid")]
    Oid(FieldExtension),
    /// Variant accepting the PositiveInt extension.
    #[serde(rename = "_valuePositiveInt")]
    PositiveInt(FieldExtension),
    /// Variant accepting the String extension.
    #[serde(rename = "_valueString")]
    String(FieldExtension),
    /// Variant accepting the Time extension.
    #[serde(rename = "_valueTime")]
    Time(FieldExtension),
    /// Variant accepting the UnsignedInt extension.
    #[serde(rename = "_valueUnsignedInt")]
    UnsignedInt(FieldExtension),
    /// Variant accepting the Uri extension.
    #[serde(rename = "_valueUri")]
    Uri(FieldExtension),
    /// Variant accepting the Address extension.
    #[serde(rename = "_valueAddress")]
    Address(FieldExtension),
    /// Variant accepting the Age extension.
    #[serde(rename = "_valueAge")]
    Age(FieldExtension),
    /// Variant accepting the Annotation extension.
    #[serde(rename = "_valueAnnotation")]
    Annotation(FieldExtension),
    /// Variant accepting the Attachment extension.
    #[serde(rename = "_valueAttachment")]
    Attachment(FieldExtension),
    /// Variant accepting the CodeableConcept extension.
    #[serde(rename = "_valueCodeableConcept")]
    CodeableConcept(FieldExtension),
    /// Variant accepting the Coding extension.
    #[serde(rename = "_valueCoding")]
    Coding(FieldExtension),
    /// Variant accepting the ContactPoint extension.
    #[serde(rename = "_valueContactPoint")]
    ContactPoint(FieldExtension),
    /// Variant accepting the Count extension.
    #[serde(rename = "_valueCount")]
    Count(FieldExtension),
    /// Variant accepting the Distance extension.
    #[serde(rename = "_valueDistance")]
    Distance(FieldExtension),
    /// Variant accepting the Duration extension.
    #[serde(rename = "_valueDuration")]
    Duration(FieldExtension),
    /// Variant accepting the HumanName extension.
    #[serde(rename = "_valueHumanName")]
    HumanName(FieldExtension),
    /// Variant accepting the Identifier extension.
    #[serde(rename = "_valueIdentifier")]
    Identifier(FieldExtension),
    /// Variant accepting the Money extension.
    #[serde(rename = "_valueMoney")]
    Money(FieldExtension),
    /// Variant accepting the Period extension.
    #[serde(rename = "_valuePeriod")]
    Period(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_valueQuantity")]
    Quantity(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_valueRange")]
    Range(FieldExtension),
    /// Variant accepting the Ratio extension.
    #[serde(rename = "_valueRatio")]
    Ratio(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_valueReference")]
    Reference(FieldExtension),
    /// Variant accepting the SampledData extension.
    #[serde(rename = "_valueSampledData")]
    SampledData(FieldExtension),
    /// Variant accepting the Signature extension.
    #[serde(rename = "_valueSignature")]
    Signature(FieldExtension),
    /// Variant accepting the Timing extension.
    #[serde(rename = "_valueTiming")]
    Timing(FieldExtension),
    /// Variant accepting the Meta extension.
    #[serde(rename = "_valueMeta")]
    Meta(FieldExtension),
}
/// Choice of types for the minValue[x] field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionMinValue {
    /// Variant accepting the Date type.
    #[serde(rename = "minValueDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "minValueDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Instant type.
    #[serde(rename = "minValueInstant")]
    Instant(Instant),
    /// Variant accepting the Time type.
    #[serde(rename = "minValueTime")]
    Time(Time),
    /// Variant accepting the Decimal type.
    #[serde(rename = "minValueDecimal")]
    Decimal(f64),
    /// Variant accepting the Integer type.
    #[serde(rename = "minValueInteger")]
    Integer(i32),
    /// Variant accepting the PositiveInt type.
    #[serde(rename = "minValuePositiveInt")]
    PositiveInt(NonZeroU32),
    /// Variant accepting the UnsignedInt type.
    #[serde(rename = "minValueUnsignedInt")]
    UnsignedInt(u32),
    /// Variant accepting the Quantity type.
    #[serde(rename = "minValueQuantity")]
    Quantity(Quantity),
}
/// Extension value for ElementDefinitionMinValue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionMinValueExtension {
    /// Variant accepting the Date extension.
    #[serde(rename = "_minValueDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_minValueDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Instant extension.
    #[serde(rename = "_minValueInstant")]
    Instant(FieldExtension),
    /// Variant accepting the Time extension.
    #[serde(rename = "_minValueTime")]
    Time(FieldExtension),
    /// Variant accepting the Decimal extension.
    #[serde(rename = "_minValueDecimal")]
    Decimal(FieldExtension),
    /// Variant accepting the Integer extension.
    #[serde(rename = "_minValueInteger")]
    Integer(FieldExtension),
    /// Variant accepting the PositiveInt extension.
    #[serde(rename = "_minValuePositiveInt")]
    PositiveInt(FieldExtension),
    /// Variant accepting the UnsignedInt extension.
    #[serde(rename = "_minValueUnsignedInt")]
    UnsignedInt(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_minValueQuantity")]
    Quantity(FieldExtension),
}
/// Choice of types for the maxValue[x] field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionMaxValue {
    /// Variant accepting the Date type.
    #[serde(rename = "maxValueDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "maxValueDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Instant type.
    #[serde(rename = "maxValueInstant")]
    Instant(Instant),
    /// Variant accepting the Time type.
    #[serde(rename = "maxValueTime")]
    Time(Time),
    /// Variant accepting the Decimal type.
    #[serde(rename = "maxValueDecimal")]
    Decimal(f64),
    /// Variant accepting the Integer type.
    #[serde(rename = "maxValueInteger")]
    Integer(i32),
    /// Variant accepting the PositiveInt type.
    #[serde(rename = "maxValuePositiveInt")]
    PositiveInt(NonZeroU32),
    /// Variant accepting the UnsignedInt type.
    #[serde(rename = "maxValueUnsignedInt")]
    UnsignedInt(u32),
    /// Variant accepting the Quantity type.
    #[serde(rename = "maxValueQuantity")]
    Quantity(Quantity),
}
/// Extension value for ElementDefinitionMaxValue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionMaxValueExtension {
    /// Variant accepting the Date extension.
    #[serde(rename = "_maxValueDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_maxValueDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Instant extension.
    #[serde(rename = "_maxValueInstant")]
    Instant(FieldExtension),
    /// Variant accepting the Time extension.
    #[serde(rename = "_maxValueTime")]
    Time(FieldExtension),
    /// Variant accepting the Decimal extension.
    #[serde(rename = "_maxValueDecimal")]
    Decimal(FieldExtension),
    /// Variant accepting the Integer extension.
    #[serde(rename = "_maxValueInteger")]
    Integer(FieldExtension),
    /// Variant accepting the PositiveInt extension.
    #[serde(rename = "_maxValuePositiveInt")]
    PositiveInt(FieldExtension),
    /// Variant accepting the UnsignedInt extension.
    #[serde(rename = "_maxValueUnsignedInt")]
    UnsignedInt(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_maxValueQuantity")]
    Quantity(FieldExtension),
}
/// Sub-fields of the constraint field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionConstraintBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionConstraint {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Target of 'condition' reference above**

 Allows identification of which elements have their cardinalities impacted by the constraint.  Will not be referenced for constraints that do not affect cardinality.

 */
    #[serde(rename = "key")]
    pub key: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_key")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub key_ext: Option<FieldExtension>,
    /** **Why this constraint is necessary or appropriate**

 Description of why this constraint is necessary or appropriate.

 To be used if the reason for the constraint may not be intuitive to all implementers. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "requirements")]
    pub requirements: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_requirements")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub requirements_ext: Option<FieldExtension>,
    /** **[ConstraintSeverity](http://hl7.org/fhir/ValueSet/constraint-severity); error | warning**

 Identifies the impact constraint violation has on the conformance of the instance.

 This allows constraints to be asserted as "shall" (error) and "should" (warning). */
    #[serde(rename = "severity")]
    pub severity: codes::ConstraintSeverity,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_severity")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub severity_ext: Option<FieldExtension>,
    /** **Human description of constraint**

 Text that can be used to describe the constraint in messages identifying that the constraint has been violated.

 Should be expressed in business terms as much as possible. */
    #[serde(rename = "human")]
    pub human: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_human")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub human_ext: Option<FieldExtension>,
    /** **FHIRPath expression of constraint**

 A [FHIRPath](http://hl7.org/fluentpath) expression of constraint that can be executed to see if this constraint is met.

 */
    #[serde(rename = "expression")]
    pub expression: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_expression")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub expression_ext: Option<FieldExtension>,
    /** **XPath expression of constraint**

 An XPath expression of constraint that can be executed to see if this constraint is met.

 Elements SHALL use "f" as the namespace prefix for the FHIR namespace, and "x" for the xhtml namespace, and SHALL NOT use any other prefixes.

Note: we are considering deprecating the xpath element. Implementer feedback is welcome. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "xpath")]
    pub xpath: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_xpath")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub xpath_ext: Option<FieldExtension>,
    /** **Reference to original source of constraint**

 A reference to the original source of the constraint, for traceability purposes.

 This is used when, e.g. rendering, where it is not useful to present inherited constraints when rendering the snapshot. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "source")]
    pub source: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_source")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub source_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionConstraint {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionConstraintBuilder {
        ElementDefinitionConstraintBuilder::default()
    }
}
/// Sub-fields of the binding field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionBindingBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionBinding {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[BindingStrength](http://hl7.org/fhir/ValueSet/binding-strength); required | extensible | preferred | example**

 Indicates the degree of conformance expectations associated with this binding - that is, the degree to which the provided value set must be adhered to in the instances.

 For further discussion, see [Using Terminologies](terminologies.html). */
    #[serde(rename = "strength")]
    pub strength: codes::BindingStrength,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_strength")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub strength_ext: Option<FieldExtension>,
    /** **Human explanation of the value set**

 Describes the intended use of this particular set of codes.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_description")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub description_ext: Option<FieldExtension>,
    /** **Source of value set**

 Points to the value set or external definition (e.g. implicit value set) that identifies the set of codes to be used. If the binding refers to an explicit value set - the normal case - then use a Reference(ValueSet) preferably containing the canonical URL for the value set. If the reference is to an implicit value set - usually, an IETF RFC that defines a grammar, such as mime types - then use a uri.

 For value sets with a referenceResource, the display can contain the value set description.  The reference may be version-specific or not. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub value_set: Option<ElementDefinitionBindingValueSet>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_set_ext: Option<ElementDefinitionBindingValueSetExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionBinding {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionBindingBuilder {
        ElementDefinitionBindingBuilder::default()
    }
}
/// Choice of types for the valueSet[x] field in ElementDefinitionBinding
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionBindingValueSet {
    /// Variant accepting the Uri type.
    #[serde(rename = "valueSetUri")]
    Uri(String),
    /// Variant accepting the Reference type.
    #[serde(rename = "valueSetReference")]
    Reference(Reference),
}
/// Extension value for ElementDefinitionBindingValueSet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementDefinitionBindingValueSetExtension {
    /// Variant accepting the Uri extension.
    #[serde(rename = "_valueSetUri")]
    Uri(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_valueSetReference")]
    Reference(FieldExtension),
}
/// Sub-fields of the mapping field in ElementDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ElementDefinitionMappingBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct ElementDefinitionMapping {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Reference to mapping declaration**

 An internal reference to the definition of a mapping.

 */
    #[serde(rename = "identity")]
    pub identity: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_identity")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub identity_ext: Option<FieldExtension>,
    /** **[MimeType](http://www.rfc-editor.org/bcp/bcp13.txt); Computable language of mapping**

 Identifies the computable language in which mapping.map is expressed.

 If omitted, then there can be no expectation of computational interpretation of the mapping. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "language")]
    pub language: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_language")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub language_ext: Option<FieldExtension>,
    /** **Details of the mapping**

 Expresses what part of the target specification corresponds to this element.

 For most mappings, the syntax is undefined.  Syntax will be provided for mappings to the RIM.  Multiple mappings may be possible and may include constraints on other resource elements that identify when a particular mapping applies. */
    #[serde(rename = "map")]
    pub map: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_map")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub map_ext: Option<FieldExtension>,
    /** **Comments about the mapping or its use**

 Comments that provide information about the mapping or its use.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_comment")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub comment_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ElementDefinitionMapping {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> ElementDefinitionMappingBuilder {
        ElementDefinitionMappingBuilder::default()
    }
}
/** Base StructureDefinition for Extension Type

 **[Extension](http://hl7.org/fhir/StructureDefinition/Extension) v3.0.2**

 Optional Extensions Element

 Optional Extension Element - found in all resources.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Extension(pub Box<ExtensionInner>);
/** Base StructureDefinition for Extension Type

 **[Extension](http://hl7.org/fhir/StructureDefinition/Extension) v3.0.2**

 Optional Extensions Element

 Optional Extension Element - found in all resources.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ExtensionBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ExtensionInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **identifies the meaning of the extension**

 Source of the definition for the extension code - a logical name or a URL.

 The definition may point directly to a computable or human-readable definition of the extensibility codes, or it may be a logical URI as declared in some other specification. The definition SHALL be a URI for the Structure Definition defining the extension. */
    #[serde(rename = "url")]
    pub url: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_url")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub url_ext: Option<FieldExtension>,
    /** **Value of extension**

 Value of extension - may be a resource or one of a constrained set of the data types (see Extensibility in the spec for list).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub value: Option<ExtensionValue>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<ExtensionValueExtension>,
}
#[cfg(feature = "builders")]
impl ExtensionBuilder {
    #[doc = concat!("Finalize building ", "Extension", ".")]
    pub fn build(self) -> Result<Extension, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ExtensionInner> for Extension {
    fn from(inner: ExtensionInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Extension {
    type Target = ExtensionInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Extension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Extension {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ExtensionBuilder {
        ExtensionBuilder::default()
    }
}
/// Choice of types for the value[x] field in Extension
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExtensionValue {
    /// Variant accepting the Base64Binary type.
    #[serde(rename = "valueBase64Binary")]
    Base64Binary(Base64Binary),
    /// Variant accepting the Boolean type.
    #[serde(rename = "valueBoolean")]
    Boolean(bool),
    /// Variant accepting the Code type.
    #[serde(rename = "valueCode")]
    Code(String),
    /// Variant accepting the Date type.
    #[serde(rename = "valueDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "valueDateTime")]
    DateTime(DateTime),
    /// Variant accepting the Decimal type.
    #[serde(rename = "valueDecimal")]
    Decimal(f64),
    /// Variant accepting the Id type.
    #[serde(rename = "valueId")]
    Id(String),
    /// Variant accepting the Instant type.
    #[serde(rename = "valueInstant")]
    Instant(Instant),
    /// Variant accepting the Integer type.
    #[serde(rename = "valueInteger")]
    Integer(i32),
    /// Variant accepting the Markdown type.
    #[serde(rename = "valueMarkdown")]
    Markdown(String),
    /// Variant accepting the Oid type.
    #[serde(rename = "valueOid")]
    Oid(String),
    /// Variant accepting the PositiveInt type.
    #[serde(rename = "valuePositiveInt")]
    PositiveInt(NonZeroU32),
    /// Variant accepting the String type.
    #[serde(rename = "valueString")]
    String(String),
    /// Variant accepting the Time type.
    #[serde(rename = "valueTime")]
    Time(Time),
    /// Variant accepting the UnsignedInt type.
    #[serde(rename = "valueUnsignedInt")]
    UnsignedInt(u32),
    /// Variant accepting the Uri type.
    #[serde(rename = "valueUri")]
    Uri(String),
    /// Variant accepting the Address type.
    #[serde(rename = "valueAddress")]
    Address(Address),
    /// Variant accepting the Age type.
    #[serde(rename = "valueAge")]
    Age(Age),
    /// Variant accepting the Annotation type.
    #[serde(rename = "valueAnnotation")]
    Annotation(Annotation),
    /// Variant accepting the Attachment type.
    #[serde(rename = "valueAttachment")]
    Attachment(Attachment),
    /// Variant accepting the CodeableConcept type.
    #[serde(rename = "valueCodeableConcept")]
    CodeableConcept(CodeableConcept),
    /// Variant accepting the Coding type.
    #[serde(rename = "valueCoding")]
    Coding(Coding),
    /// Variant accepting the ContactPoint type.
    #[serde(rename = "valueContactPoint")]
    ContactPoint(ContactPoint),
    /// Variant accepting the Count type.
    #[serde(rename = "valueCount")]
    Count(Count),
    /// Variant accepting the Distance type.
    #[serde(rename = "valueDistance")]
    Distance(Distance),
    /// Variant accepting the Duration type.
    #[serde(rename = "valueDuration")]
    Duration(Duration),
    /// Variant accepting the HumanName type.
    #[serde(rename = "valueHumanName")]
    HumanName(HumanName),
    /// Variant accepting the Identifier type.
    #[serde(rename = "valueIdentifier")]
    Identifier(Identifier),
    /// Variant accepting the Money type.
    #[serde(rename = "valueMoney")]
    Money(Money),
    /// Variant accepting the Period type.
    #[serde(rename = "valuePeriod")]
    Period(Period),
    /// Variant accepting the Quantity type.
    #[serde(rename = "valueQuantity")]
    Quantity(Quantity),
    /// Variant accepting the Range type.
    #[serde(rename = "valueRange")]
    Range(Range),
    /// Variant accepting the Ratio type.
    #[serde(rename = "valueRatio")]
    Ratio(Ratio),
    /// Variant accepting the Reference type.
    #[serde(rename = "valueReference")]
    Reference(Reference),
    /// Variant accepting the SampledData type.
    #[serde(rename = "valueSampledData")]
    SampledData(SampledData),
    /// Variant accepting the Signature type.
    #[serde(rename = "valueSignature")]
    Signature(Signature),
    /// Variant accepting the Timing type.
    #[serde(rename = "valueTiming")]
    Timing(Timing),
    /// Variant accepting the Meta type.
    #[serde(rename = "valueMeta")]
    Meta(Meta),
}
/// Extension value for ExtensionValue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExtensionValueExtension {
    /// Variant accepting the Base64Binary extension.
    #[serde(rename = "_valueBase64Binary")]
    Base64Binary(FieldExtension),
    /// Variant accepting the Boolean extension.
    #[serde(rename = "_valueBoolean")]
    Boolean(FieldExtension),
    /// Variant accepting the Code extension.
    #[serde(rename = "_valueCode")]
    Code(FieldExtension),
    /// Variant accepting the Date extension.
    #[serde(rename = "_valueDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_valueDateTime")]
    DateTime(FieldExtension),
    /// Variant accepting the Decimal extension.
    #[serde(rename = "_valueDecimal")]
    Decimal(FieldExtension),
    /// Variant accepting the Id extension.
    #[serde(rename = "_valueId")]
    Id(FieldExtension),
    /// Variant accepting the Instant extension.
    #[serde(rename = "_valueInstant")]
    Instant(FieldExtension),
    /// Variant accepting the Integer extension.
    #[serde(rename = "_valueInteger")]
    Integer(FieldExtension),
    /// Variant accepting the Markdown extension.
    #[serde(rename = "_valueMarkdown")]
    Markdown(FieldExtension),
    /// Variant accepting the Oid extension.
    #[serde(rename = "_valueOid")]
    Oid(FieldExtension),
    /// Variant accepting the PositiveInt extension.
    #[serde(rename = "_valuePositiveInt")]
    PositiveInt(FieldExtension),
    /// Variant accepting the String extension.
    #[serde(rename = "_valueString")]
    String(FieldExtension),
    /// Variant accepting the Time extension.
    #[serde(rename = "_valueTime")]
    Time(FieldExtension),
    /// Variant accepting the UnsignedInt extension.
    #[serde(rename = "_valueUnsignedInt")]
    UnsignedInt(FieldExtension),
    /// Variant accepting the Uri extension.
    #[serde(rename = "_valueUri")]
    Uri(FieldExtension),
    /// Variant accepting the Address extension.
    #[serde(rename = "_valueAddress")]
    Address(FieldExtension),
    /// Variant accepting the Age extension.
    #[serde(rename = "_valueAge")]
    Age(FieldExtension),
    /// Variant accepting the Annotation extension.
    #[serde(rename = "_valueAnnotation")]
    Annotation(FieldExtension),
    /// Variant accepting the Attachment extension.
    #[serde(rename = "_valueAttachment")]
    Attachment(FieldExtension),
    /// Variant accepting the CodeableConcept extension.
    #[serde(rename = "_valueCodeableConcept")]
    CodeableConcept(FieldExtension),
    /// Variant accepting the Coding extension.
    #[serde(rename = "_valueCoding")]
    Coding(FieldExtension),
    /// Variant accepting the ContactPoint extension.
    #[serde(rename = "_valueContactPoint")]
    ContactPoint(FieldExtension),
    /// Variant accepting the Count extension.
    #[serde(rename = "_valueCount")]
    Count(FieldExtension),
    /// Variant accepting the Distance extension.
    #[serde(rename = "_valueDistance")]
    Distance(FieldExtension),
    /// Variant accepting the Duration extension.
    #[serde(rename = "_valueDuration")]
    Duration(FieldExtension),
    /// Variant accepting the HumanName extension.
    #[serde(rename = "_valueHumanName")]
    HumanName(FieldExtension),
    /// Variant accepting the Identifier extension.
    #[serde(rename = "_valueIdentifier")]
    Identifier(FieldExtension),
    /// Variant accepting the Money extension.
    #[serde(rename = "_valueMoney")]
    Money(FieldExtension),
    /// Variant accepting the Period extension.
    #[serde(rename = "_valuePeriod")]
    Period(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_valueQuantity")]
    Quantity(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_valueRange")]
    Range(FieldExtension),
    /// Variant accepting the Ratio extension.
    #[serde(rename = "_valueRatio")]
    Ratio(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_valueReference")]
    Reference(FieldExtension),
    /// Variant accepting the SampledData extension.
    #[serde(rename = "_valueSampledData")]
    SampledData(FieldExtension),
    /// Variant accepting the Signature extension.
    #[serde(rename = "_valueSignature")]
    Signature(FieldExtension),
    /// Variant accepting the Timing extension.
    #[serde(rename = "_valueTiming")]
    Timing(FieldExtension),
    /// Variant accepting the Meta extension.
    #[serde(rename = "_valueMeta")]
    Meta(FieldExtension),
}
/** Base StructureDefinition for HumanName Type

 **[HumanName](http://hl7.org/fhir/StructureDefinition/HumanName) v3.0.2**

 Name of a human - parts and usage

 A human's name with the ability to identify parts and usage.

 Names may be changed, or repudiated, or people may have different names in different contexts. Names may be divided into parts of different type that have variable significance depending on context, though the division into parts does not always matter. With personal names, the different parts may or may not be imbued with some implicit meaning; various cultures associate different importance with the name parts and the degree to which systems must care about name parts around the world varies widely. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HumanName(pub Box<HumanNameInner>);
/** Base StructureDefinition for HumanName Type

 **[HumanName](http://hl7.org/fhir/StructureDefinition/HumanName) v3.0.2**

 Name of a human - parts and usage

 A human's name with the ability to identify parts and usage.

 Names may be changed, or repudiated, or people may have different names in different contexts. Names may be divided into parts of different type that have variable significance depending on context, though the division into parts does not always matter. With personal names, the different parts may or may not be imbued with some implicit meaning; various cultures associate different importance with the name parts and the degree to which systems must care about name parts around the world varies widely. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "HumanNameBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct HumanNameInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[NameUse](http://hl7.org/fhir/ValueSet/name-use); usual | official | temp | nickname | anonymous | old | maiden**

 Identifies the purpose for this name.

 This is labeled as "Is Modifier" because applications should not mistake a temporary or old name etc.for a current/permanent one. Applications can assume that a name is current unless it explicitly says that it is temporary or old. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "use")]
    pub r#use: Option<codes::NameUse>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_use")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#use_ext: Option<FieldExtension>,
    /** **Text representation of the full name**

 A full text representation of the name.

 Can provide both a text representation and structured parts. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "text")]
    pub text: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_text")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub text_ext: Option<FieldExtension>,
    /** **Family name (often called 'Surname')**

 The part of a name that links to the genealogy. In some cultures (e.g. Eritrea) the family name of a son is the first name of his father.

 Family Name may be decomposed into specific parts using extensions (de, nl, es related cultures). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "family")]
    pub family: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_family")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub family_ext: Option<FieldExtension>,
    /** **Given names (not always 'first'). Includes middle names**

 Given name.

 If only initials are recorded, they may be used in place of the full name.  Not called "first name" since given names do not always come first. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "given")]
    pub given: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_given")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub given_ext: Vec<Option<FieldExtension>>,
    /** **Parts that come before the name**

 Part of the name that is acquired as a title due to academic, legal, employment or nobility status, etc. and that appears at the start of the name.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "prefix")]
    pub prefix: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_prefix")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub prefix_ext: Vec<Option<FieldExtension>>,
    /** **Parts that come after the name**

 Part of the name that is acquired as a title due to academic, legal, employment or nobility status, etc. and that appears at the end of the name.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "suffix")]
    pub suffix: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_suffix")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub suffix_ext: Vec<Option<FieldExtension>>,
    /** **Time period when name was/is in use**

 Indicates the period of time when this name was valid for the named person.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "period")]
    pub period: Option<Period>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_period")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl HumanNameBuilder {
    #[doc = concat!("Finalize building ", "HumanName", ".")]
    pub fn build(self) -> Result<HumanName, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<HumanNameInner> for HumanName {
    fn from(inner: HumanNameInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for HumanName {
    type Target = HumanNameInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for HumanName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl HumanName {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> HumanNameBuilder {
        HumanNameBuilder::default()
    }
}
/** Base StructureDefinition for Identifier Type

 **[Identifier](http://hl7.org/fhir/StructureDefinition/Identifier) v3.0.2**

 An identifier intended for computation

 A technical identifier - identifies some entity uniquely and unambiguously.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Identifier(pub Box<IdentifierInner>);
/** Base StructureDefinition for Identifier Type

 **[Identifier](http://hl7.org/fhir/StructureDefinition/Identifier) v3.0.2**

 An identifier intended for computation

 A technical identifier - identifies some entity uniquely and unambiguously.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "IdentifierBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct IdentifierInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[IdentifierUse](http://hl7.org/fhir/ValueSet/identifier-use); usual | official | temp | secondary (If known)**

 The purpose of this identifier.

 This is labeled as "Is Modifier" because applications should not mistake a temporary id for a permanent one. Applications can assume that an identifier is permanent unless it explicitly says that it is temporary. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "use")]
    pub r#use: Option<codes::IdentifierUse>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_use")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#use_ext: Option<FieldExtension>,
    /** **[IdentifierType](http://hl7.org/fhir/ValueSet/identifier-type); Description of identifier**

 A coded type for the identifier that can be used to determine which identifier to use for a specific purpose.

 This element deals only with general categories of identifiers.  It SHOULD not be used for codes that correspond 1..1 with the Identifier.system. Some identifiers may fall into multiple categories due to common usage.

Where the system is known, a type is unnecessary because the type is always part of the system definition. However systems often need to handle identifiers where the system is not known. There is not a 1:1 relationship between type and system, since many different systems have the same type. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **The namespace for the identifier value**

 Establishes the namespace for the value - that is, a URL that describes a set values that are unique.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_system")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub system_ext: Option<FieldExtension>,
    /** **The value that is unique**

 The portion of the identifier typically relevant to the user and which is unique within the context of the system.

 If the value is a full URI, then the system SHALL be urn:ietf:rfc:3986.  The value's primary purpose is computational mapping.  As a result, it may be normalized for comparison purposes (e.g. removing non-significant whitespace, dashes, etc.)  A value formatted for human display can be conveyed using the [Rendered Value extension](extension-rendered-value.html). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_value")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<FieldExtension>,
    /** **Time period when id is/was valid for use**

 Time period during which identifier is/was valid for use.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "period")]
    pub period: Option<Period>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_period")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_ext: Option<FieldExtension>,
    /** **Organization that issued id (may be just text)**

 Organization that issued/manages the identifier.

 The Identifier.assigner may omit the .reference element and only contain a .display element reflecting the name or other textual information about the assigning organization. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "assigner")]
    pub assigner: Option<Reference>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_assigner")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub assigner_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl IdentifierBuilder {
    #[doc = concat!("Finalize building ", "Identifier", ".")]
    pub fn build(self) -> Result<Identifier, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<IdentifierInner> for Identifier {
    fn from(inner: IdentifierInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Identifier {
    type Target = IdentifierInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Identifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Identifier {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> IdentifierBuilder {
        IdentifierBuilder::default()
    }
}
/** Base StructureDefinition for Meta Type

 **[Meta](http://hl7.org/fhir/StructureDefinition/Meta) v3.0.2**

 Metadata about a resource

 The metadata about a resource. This is content in the resource that is maintained by the infrastructure. Changes to the content may not always be associated with version changes to the resource.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Meta(pub Box<MetaInner>);
/** Base StructureDefinition for Meta Type

 **[Meta](http://hl7.org/fhir/StructureDefinition/Meta) v3.0.2**

 Metadata about a resource

 The metadata about a resource. This is content in the resource that is maintained by the infrastructure. Changes to the content may not always be associated with version changes to the resource.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "MetaBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct MetaInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Version specific identifier**

 The version specific identifier, as it appears in the version portion of the URL. This values changes when the resource is created, updated, or deleted.

 The server assigns this value, and ignores what the client specifies, except in the case that the server is imposing version integrity on updates/deletes. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "versionId")]
    pub version_id: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_versionId")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub version_id_ext: Option<FieldExtension>,
    /** **When the resource version last changed**

 When the resource last changed - e.g. when the version changed.

 This value is always populated except when the resource is first being created. The server / resource manager sets this value; what a client provides is irrelevant. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<Instant>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_lastUpdated")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub last_updated_ext: Option<FieldExtension>,
    /** **Profiles this resource claims to conform to**

 A list of profiles (references to [StructureDefinition](structuredefinition.html#) resources) that this resource claims to conform to. The URL is a reference to [StructureDefinition.url]().

 It is up to the server and/or other infrastructure of policy to determine whether/how these claims are verified and/or updated over time.  The list of profile URLs is a set. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "profile")]
    pub profile: Vec<Option<String>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_profile")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub profile_ext: Vec<Option<FieldExtension>>,
    /** **[SecurityLabels](http://hl7.org/fhir/ValueSet/security-labels); Security Labels applied to this resource**

 Security labels applied to this resource. These tags connect specific resources to the overall security policy and infrastructure.

 The security labels can be updated without changing the stated version of the resource  The list of security labels is a set. Uniqueness is based the system/code, and version and display are ignored. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "security")]
    pub security: Vec<Option<Coding>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_security")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub security_ext: Vec<Option<FieldExtension>>,
    /** **[Tags](http://hl7.org/fhir/ValueSet/common-tags); Tags applied to this resource**

 Tags applied to this resource. Tags are intended to be used to identify and relate resources to process and workflow, and applications are not required to consider the tags when interpreting the meaning of a resource.

 The tags can be updated without changing the stated version of the resource.  The list of tags is a set. Uniqueness is based the system/code, and version and display are ignored. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "tag")]
    pub tag: Vec<Option<Coding>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_tag")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub tag_ext: Vec<Option<FieldExtension>>,
}
#[cfg(feature = "builders")]
impl MetaBuilder {
    #[doc = concat!("Finalize building ", "Meta", ".")]
    pub fn build(self) -> Result<Meta, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<MetaInner> for Meta {
    fn from(inner: MetaInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Meta {
    type Target = MetaInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Meta {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Meta {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> MetaBuilder {
        MetaBuilder::default()
    }
}
/** Base StructureDefinition for Money Type

 **[Money](http://hl7.org/fhir/StructureDefinition/Money) v3.0.2**

 An amount of economic utility in some recognized currency

 An amount of economic utility in some recognized currency.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Money(pub Box<MoneyInner>);
/** Base StructureDefinition for Money Type

 **[Money](http://hl7.org/fhir/StructureDefinition/Money) v3.0.2**

 An amount of economic utility in some recognized currency

 An amount of economic utility in some recognized currency.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "MoneyBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct MoneyInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerical value (with implicit precision)**

 The value of the measured amount. The value includes an implicit precision in the presentation of the value.

 The implicit precision in the value should always be honored. Monetary values have their own rules for handling precision (refer to standard accounting text books). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /** **[QuantityComparator](http://hl7.org/fhir/ValueSet/quantity-comparator); < | <= | >= | > - how to understand the value**

 How the value should be understood and represented - whether the actual value is greater or less than the stated value due to measurement issues; e.g. if the comparator is "<" , then the real value is < stated value.

 This is labeled as "Is Modifier" because the comparator modifies the interpretation of the value significantly. If there is no comparator, then there is no modification of the value. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comparator")]
    pub comparator: Option<codes::QuantityComparator>,
    /** **Unit representation**

 A human-readable form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    /** **System that defines coded unit form**

 The identification of the system that provides the coded form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /** **Coded form of the unit**

 A computer processable form of the unit in some unit representation system.

 The preferred system is UCUM, but SNOMED CT can also be used (for customary units) or ISO 4217 for currency.  The context of use may additionally require a code from a particular system. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
}
#[cfg(feature = "builders")]
impl MoneyBuilder {
    #[doc = concat!("Finalize building ", "Money", ".")]
    pub fn build(self) -> Result<Money, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<MoneyInner> for Money {
    fn from(inner: MoneyInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Money {
    type Target = MoneyInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Money {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Money {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> MoneyBuilder {
        MoneyBuilder::default()
    }
}
/** Base StructureDefinition for Narrative Type

 **[Narrative](http://hl7.org/fhir/StructureDefinition/Narrative) v3.0.2**

 A human-readable formatted text, including images

 A human-readable formatted text, including images.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Narrative(pub Box<NarrativeInner>);
/** Base StructureDefinition for Narrative Type

 **[Narrative](http://hl7.org/fhir/StructureDefinition/Narrative) v3.0.2**

 A human-readable formatted text, including images

 A human-readable formatted text, including images.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "NarrativeBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct NarrativeInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[NarrativeStatus](http://hl7.org/fhir/ValueSet/narrative-status); generated | extensions | additional | empty**

 The status of the narrative - whether it's entirely generated (from just the defined data or the extensions too), or whether a human authored it and it may contain additional data.

 */
    #[serde(rename = "status")]
    pub status: codes::NarrativeStatus,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_status")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub status_ext: Option<FieldExtension>,
    /** **Limited xhtml content**

 The actual narrative content, a stripped down version of XHTML.

 The contents of the html element are an XHTML fragment containing only the basic html formatting elements described in chapters 7-11 and 15 of the HTML 4.0 standard, <a> elements (either name or href), images and internally contained stylesheets. The XHTML content may not contain a head, a body, external stylesheet references, scripts, forms, base/link/xlink, frames, iframes and objects. */
    #[serde(rename = "div")]
    pub div: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_div")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub div_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl NarrativeBuilder {
    #[doc = concat!("Finalize building ", "Narrative", ".")]
    pub fn build(self) -> Result<Narrative, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<NarrativeInner> for Narrative {
    fn from(inner: NarrativeInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Narrative {
    type Target = NarrativeInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Narrative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Narrative {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> NarrativeBuilder {
        NarrativeBuilder::default()
    }
}
/** Base StructureDefinition for ParameterDefinition Type

 **[ParameterDefinition](http://hl7.org/fhir/StructureDefinition/ParameterDefinition) v3.0.2**

 Definition of a parameter to a module

 The parameters to the module. This collection specifies both the input and output parameters. Input parameters are provided by the caller as part of the $evaluate operation. Output parameters are included in the GuidanceResponse.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParameterDefinition(pub Box<ParameterDefinitionInner>);
/** Base StructureDefinition for ParameterDefinition Type

 **[ParameterDefinition](http://hl7.org/fhir/StructureDefinition/ParameterDefinition) v3.0.2**

 Definition of a parameter to a module

 The parameters to the module. This collection specifies both the input and output parameters. Input parameters are provided by the caller as part of the $evaluate operation. Output parameters are included in the GuidanceResponse.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ParameterDefinitionBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ParameterDefinitionInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Name used to access the parameter value**

 The name of the parameter used to allow access to the value of the parameter in evaluation contexts.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_name")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub name_ext: Option<FieldExtension>,
    /** **[ParameterUse](http://hl7.org/fhir/ValueSet/operation-parameter-use); in | out**

 Whether the parameter is input or output for the module.

 */
    #[serde(rename = "use")]
    pub r#use: codes::OperationParameterUse,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_use")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#use_ext: Option<FieldExtension>,
    /** **Minimum cardinality**

 The minimum number of times this parameter SHALL appear in the request or response.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "min")]
    pub min: Option<i32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_min")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub min_ext: Option<FieldExtension>,
    /** **Maximum cardinality (a number of *)**

 The maximum number of times this element is permitted to appear in the request or response.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "max")]
    pub max: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_max")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub max_ext: Option<FieldExtension>,
    /** **A brief description of the parameter**

 A brief discussion of what the parameter is for and how it is used by the module.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "documentation")]
    pub documentation: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_documentation")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub documentation_ext: Option<FieldExtension>,
    /** **[FHIRAllTypes](http://hl7.org/fhir/ValueSet/all-types); What type of value**

 The type of the parameter.

 */
    #[serde(rename = "type")]
    pub r#type: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **What profile the value is expected to be**

 If specified, this indicates a profile that the input data must conform to, or that the output data will conform to.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "profile")]
    pub profile: Option<Reference>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_profile")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub profile_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ParameterDefinitionBuilder {
    #[doc = concat!("Finalize building ", "ParameterDefinition", ".")]
    pub fn build(self) -> Result<ParameterDefinition, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ParameterDefinitionInner> for ParameterDefinition {
    fn from(inner: ParameterDefinitionInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for ParameterDefinition {
    type Target = ParameterDefinitionInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for ParameterDefinition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ParameterDefinition {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ParameterDefinitionBuilder {
        ParameterDefinitionBuilder::default()
    }
}
/** Base StructureDefinition for Period Type

 **[Period](http://hl7.org/fhir/StructureDefinition/Period) v3.0.2**

 Time range defined by start and end date/time

 A time period defined by a start and end date and optionally time.

 This is not a duration - that's a measure of time (a separate type), but a duration that occurs at a fixed value of time. A Period specifies a range of time; the context of use will specify whether the entire range applies (e.g. "the patient was an inpatient of the hospital for this time range") or one value from the range applies (e.g. "give to the patient between these two times"). If duration is required, specify the type as Interval|Duration. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Period(pub Box<PeriodInner>);
/** Base StructureDefinition for Period Type

 **[Period](http://hl7.org/fhir/StructureDefinition/Period) v3.0.2**

 Time range defined by start and end date/time

 A time period defined by a start and end date and optionally time.

 This is not a duration - that's a measure of time (a separate type), but a duration that occurs at a fixed value of time. A Period specifies a range of time; the context of use will specify whether the entire range applies (e.g. "the patient was an inpatient of the hospital for this time range") or one value from the range applies (e.g. "give to the patient between these two times"). If duration is required, specify the type as Interval|Duration. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "PeriodBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct PeriodInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Starting time with inclusive boundary**

 The start of the period. The boundary is inclusive.

 If the low element is missing, the meaning is that the low boundary is not known. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "start")]
    pub start: Option<DateTime>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_start")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub start_ext: Option<FieldExtension>,
    /** **End time with inclusive boundary, if not ongoing**

 The end of the period. If the end of the period is missing, it means that the period is ongoing. The start may be in the past, and the end date in the future, which means that period is expected/planned to end at that time.

 The high value includes any matching date/time. i.e. 2012-02-03T10:00:00 is in a period that has a end value of 2012-02-03. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "end")]
    pub end: Option<DateTime>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_end")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub end_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl PeriodBuilder {
    #[doc = concat!("Finalize building ", "Period", ".")]
    pub fn build(self) -> Result<Period, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<PeriodInner> for Period {
    fn from(inner: PeriodInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Period {
    type Target = PeriodInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Period {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Period {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> PeriodBuilder {
        PeriodBuilder::default()
    }
}
/** Base StructureDefinition for Quantity Type

 **[Quantity](http://hl7.org/fhir/StructureDefinition/Quantity) v3.0.2**

 A measured or measurable amount

 A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Quantity(pub Box<QuantityInner>);
/** Base StructureDefinition for Quantity Type

 **[Quantity](http://hl7.org/fhir/StructureDefinition/Quantity) v3.0.2**

 A measured or measurable amount

 A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "QuantityBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct QuantityInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerical value (with implicit precision)**

 The value of the measured amount. The value includes an implicit precision in the presentation of the value.

 The implicit precision in the value should always be honored. Monetary values have their own rules for handling precision (refer to standard accounting text books). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_value")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<FieldExtension>,
    /** **[QuantityComparator](http://hl7.org/fhir/ValueSet/quantity-comparator); < | <= | >= | > - how to understand the value**

 How the value should be understood and represented - whether the actual value is greater or less than the stated value due to measurement issues; e.g. if the comparator is "<" , then the real value is < stated value.

 This is labeled as "Is Modifier" because the comparator modifies the interpretation of the value significantly. If there is no comparator, then there is no modification of the value. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comparator")]
    pub comparator: Option<codes::QuantityComparator>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_comparator")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub comparator_ext: Option<FieldExtension>,
    /** **Unit representation**

 A human-readable form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_unit")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub unit_ext: Option<FieldExtension>,
    /** **System that defines coded unit form**

 The identification of the system that provides the coded form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_system")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub system_ext: Option<FieldExtension>,
    /** **Coded form of the unit**

 A computer processable form of the unit in some unit representation system.

 The preferred system is UCUM, but SNOMED CT can also be used (for customary units) or ISO 4217 for currency.  The context of use may additionally require a code from a particular system. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl QuantityBuilder {
    #[doc = concat!("Finalize building ", "Quantity", ".")]
    pub fn build(self) -> Result<Quantity, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<QuantityInner> for Quantity {
    fn from(inner: QuantityInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Quantity {
    type Target = QuantityInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Quantity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Quantity {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> QuantityBuilder {
        QuantityBuilder::default()
    }
}
/** Base StructureDefinition for Range Type

 **[Range](http://hl7.org/fhir/StructureDefinition/Range) v3.0.2**

 Set of values bounded by low and high

 A set of ordered Quantities defined by a low and high limit.

 The stated low and high value are assumed to have arbitrarily high precision when it comes to determining which values are in the range. I.e. 1.99 is not in the range 2 -> 3. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Range(pub Box<RangeInner>);
/** Base StructureDefinition for Range Type

 **[Range](http://hl7.org/fhir/StructureDefinition/Range) v3.0.2**

 Set of values bounded by low and high

 A set of ordered Quantities defined by a low and high limit.

 The stated low and high value are assumed to have arbitrarily high precision when it comes to determining which values are in the range. I.e. 1.99 is not in the range 2 -> 3. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "RangeBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct RangeInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Low limit**

 The low limit. The boundary is inclusive.

 If the low element is missing, the low boundary is not known. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "low")]
    pub low: Option<Quantity>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_low")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub low_ext: Option<FieldExtension>,
    /** **High limit**

 The high limit. The boundary is inclusive.

 If the high element is missing, the high boundary is not known. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "high")]
    pub high: Option<Quantity>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_high")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub high_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl RangeBuilder {
    #[doc = concat!("Finalize building ", "Range", ".")]
    pub fn build(self) -> Result<Range, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<RangeInner> for Range {
    fn from(inner: RangeInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Range {
    type Target = RangeInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Range {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Range {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> RangeBuilder {
        RangeBuilder::default()
    }
}
/** Base StructureDefinition for Ratio Type

 **[Ratio](http://hl7.org/fhir/StructureDefinition/Ratio) v3.0.2**

 A ratio of two Quantity values - a numerator and a denominator

 A relationship of two Quantity values - expressed as a numerator and a denominator.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Ratio(pub Box<RatioInner>);
/** Base StructureDefinition for Ratio Type

 **[Ratio](http://hl7.org/fhir/StructureDefinition/Ratio) v3.0.2**

 A ratio of two Quantity values - a numerator and a denominator

 A relationship of two Quantity values - expressed as a numerator and a denominator.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "RatioBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct RatioInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerator value**

 The value of the numerator.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "numerator")]
    pub numerator: Option<Quantity>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_numerator")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub numerator_ext: Option<FieldExtension>,
    /** **Denominator value**

 The value of the denominator.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "denominator")]
    pub denominator: Option<Quantity>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_denominator")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub denominator_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl RatioBuilder {
    #[doc = concat!("Finalize building ", "Ratio", ".")]
    pub fn build(self) -> Result<Ratio, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<RatioInner> for Ratio {
    fn from(inner: RatioInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Ratio {
    type Target = RatioInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Ratio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Ratio {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> RatioBuilder {
        RatioBuilder::default()
    }
}
/** Base StructureDefinition for Reference Type

 **[Reference](http://hl7.org/fhir/StructureDefinition/Reference) v3.0.2**

 A reference from one resource to another

 A reference from one resource to another.

 References SHALL be a reference to an actual FHIR resource, and SHALL be resolveable (allowing for access control, temporary unavailability, etc). Resolution can be either by retrieval from the URL, or, where applicable by resource type, by treating an absolute reference as a canonical URL and looking it up in a local registry/repository. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Reference(pub Box<ReferenceInner>);
/** Base StructureDefinition for Reference Type

 **[Reference](http://hl7.org/fhir/StructureDefinition/Reference) v3.0.2**

 A reference from one resource to another

 A reference from one resource to another.

 References SHALL be a reference to an actual FHIR resource, and SHALL be resolveable (allowing for access control, temporary unavailability, etc). Resolution can be either by retrieval from the URL, or, where applicable by resource type, by treating an absolute reference as a canonical URL and looking it up in a local registry/repository. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "ReferenceBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct ReferenceInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Literal reference, Relative, internal or absolute URL**

 A reference to a location at which the other resource is found. The reference may be a relative reference, in which case it is relative to the service base URL, or an absolute URL that resolves to the location where the resource is found. The reference may be version specific or not. If the reference is not to a FHIR RESTful server, then it should be assumed to be version specific. Internal fragment references (start with '#') refer to contained resources.

 Using absolute URLs provides a stable scalable approach suitable for a cloud/web context, while using relative/logical references provides a flexible approach suitable for use when trading across closed eco-system boundaries.   Absolute URLs do not need to point to a FHIR RESTful server, though this is the preferred approach. If the URL conforms to the structure "/[type]/[id]" then it should be assumed that the reference is to a FHIR RESTful server. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "reference")]
    pub reference: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_reference")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub reference_ext: Option<FieldExtension>,
    /** **Logical reference, when literal reference is not known**

 An identifier for the other resource. This is used when there is no way to reference the other resource directly, either because the entity is not available through a FHIR server, or because there is no way for the author of the resource to convert a known identifier to an actual location. There is no requirement that a Reference.identifier point to something that is actually exposed as a FHIR instance, but it SHALL point to a business concept that would be expected to be exposed as a FHIR instance, and that instance would need to be of a FHIR resource type allowed by the reference.

 When an identifier is provided in place of a reference, any system processing the reference will only be able to resolve the identifier to a reference if it understands the business context in which the identifier is used. Sometimes this is global (e.g. a national identifier) but often it is not. For this reason, none of the useful mechanisms described for working with references (e.g. chaining, includes) are possible, nor should servers be expected to be able resolve the reference. Servers may accept an identifier based reference untouched, resolve it, and/or reject it - see CapabilityStatement.rest.resource.referencePolicy.

When both an identifier and a literal reference are provided, the literal reference is preferred. Applications processing the resource are allowed - but not required - to check that the identifier matches the literal reference

Applications converting a logical reference to a literal reference may choose to leave the logical reference present, or remove it. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "identifier")]
    pub identifier: Option<Identifier>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_identifier")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub identifier_ext: Option<FieldExtension>,
    /** **Text alternative for the resource**

 Plain text narrative that identifies the resource in addition to the resource reference.

 This is generally not the same as the Resource.text of the referenced resource.  The purpose is to identify what's being referenced, not to fully describe it. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "display")]
    pub display: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_display")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub display_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl ReferenceBuilder {
    #[doc = concat!("Finalize building ", "Reference", ".")]
    pub fn build(self) -> Result<Reference, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<ReferenceInner> for Reference {
    fn from(inner: ReferenceInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Reference {
    type Target = ReferenceInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Reference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Reference {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> ReferenceBuilder {
        ReferenceBuilder::default()
    }
}
/** Base StructureDefinition for RelatedArtifact Type

 **[RelatedArtifact](http://hl7.org/fhir/StructureDefinition/RelatedArtifact) v3.0.2**

 Related artifacts for a knowledge resource

 Related artifacts such as additional documentation, justification, or bibliographic references.

 Each related artifact is either an attachment, or a reference to another knowledge resource, but not both. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RelatedArtifact(pub Box<RelatedArtifactInner>);
/** Base StructureDefinition for RelatedArtifact Type

 **[RelatedArtifact](http://hl7.org/fhir/StructureDefinition/RelatedArtifact) v3.0.2**

 Related artifacts for a knowledge resource

 Related artifacts such as additional documentation, justification, or bibliographic references.

 Each related artifact is either an attachment, or a reference to another knowledge resource, but not both. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "RelatedArtifactBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct RelatedArtifactInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[RelatedArtifactType](http://hl7.org/fhir/ValueSet/related-artifact-type); documentation | justification | citation | predecessor | successor | derived-from | depends-on | composed-of**

 The type of relationship to the related artifact.

 */
    #[serde(rename = "type")]
    pub r#type: codes::RelatedArtifactType,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **Brief description of the related artifact**

 A brief description of the document or knowledge resource being referenced, suitable for display to a consumer.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "display")]
    pub display: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_display")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub display_ext: Option<FieldExtension>,
    /** **Bibliographic citation for the artifact**

 A bibliographic citation for the related artifact. This text SHOULD be formatted according to an accepted citation format.

 Additional structured information about citations should be captured as extensions. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "citation")]
    pub citation: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_citation")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub citation_ext: Option<FieldExtension>,
    /** **Where the artifact can be accessed**

 A url for the artifact that can be followed to access the actual content.

 If a document or resource element is present, this element SHALL NOT be provided (use the url or reference in the Attachment or resource reference). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "url")]
    pub url: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_url")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub url_ext: Option<FieldExtension>,
    /** **What document is being referenced**

 The document being referenced, represented as an attachment. This is exclusive with the resource element.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "document")]
    pub document: Option<Attachment>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_document")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub document_ext: Option<FieldExtension>,
    /** **What resource is being referenced**

 The related resource, such as a library, value set, profile, or other knowledge resource.

 If the type is predecessor, this is a reference to the succeeding knowledge resource. If the type is successor, this is a reference to the prior knowledge resource. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "resource")]
    pub resource: Option<Reference>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_resource")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub resource_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl RelatedArtifactBuilder {
    #[doc = concat!("Finalize building ", "RelatedArtifact", ".")]
    pub fn build(self) -> Result<RelatedArtifact, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<RelatedArtifactInner> for RelatedArtifact {
    fn from(inner: RelatedArtifactInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for RelatedArtifact {
    type Target = RelatedArtifactInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for RelatedArtifact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl RelatedArtifact {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> RelatedArtifactBuilder {
        RelatedArtifactBuilder::default()
    }
}
/** Base StructureDefinition for SampledData Type

 **[SampledData](http://hl7.org/fhir/StructureDefinition/SampledData) v3.0.2**

 A series of measurements taken by a device

 A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.

 The data is not interpretable without at least origin, period, and dimensions, but these are optional to allow a separation between the template of measurement and the actual measurement, such as between DeviceCapabilities and DeviceLog.  When providing a summary view (for example with Observation.value[x]) SampledData should be represented with a brief display text such as "Sampled Data". */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SampledData(pub Box<SampledDataInner>);
/** Base StructureDefinition for SampledData Type

 **[SampledData](http://hl7.org/fhir/StructureDefinition/SampledData) v3.0.2**

 A series of measurements taken by a device

 A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.

 The data is not interpretable without at least origin, period, and dimensions, but these are optional to allow a separation between the template of measurement and the actual measurement, such as between DeviceCapabilities and DeviceLog.  When providing a summary view (for example with Observation.value[x]) SampledData should be represented with a brief display text such as "Sampled Data". */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "SampledDataBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct SampledDataInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Zero value and units**

 The base quantity that a measured value of zero represents. In addition, this provides the units of the entire measurement series.

 */
    #[serde(rename = "origin")]
    pub origin: Quantity,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_origin")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub origin_ext: Option<FieldExtension>,
    /** **Number of milliseconds between samples**

 The length of time between sampling times, measured in milliseconds.

 This is usually a whole number. */
    #[serde(rename = "period")]
    pub period: f64,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_period")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_ext: Option<FieldExtension>,
    /** **Multiply data by this before adding to origin**

 A correction factor that is applied to the sampled data points before they are added to the origin.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "factor")]
    pub factor: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_factor")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub factor_ext: Option<FieldExtension>,
    /** **Lower limit of detection**

 The lower limit of detection of the measured points. This is needed if any of the data points have the value "L" (lower than detection limit).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "lowerLimit")]
    pub lower_limit: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_lowerLimit")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub lower_limit_ext: Option<FieldExtension>,
    /** **Upper limit of detection**

 The upper limit of detection of the measured points. This is needed if any of the data points have the value "U" (higher than detection limit).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "upperLimit")]
    pub upper_limit: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_upperLimit")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub upper_limit_ext: Option<FieldExtension>,
    /** **Number of sample points at each time point**

 The number of sample points at each time point. If this value is greater than one, then the dimensions will be interlaced - all the sample points for a point in time will be recorded at once.

 If there is more than one dimension, the code for the type of data will define the meaning of the dimensions (typically ECG data). */
    #[serde(rename = "dimensions")]
    pub dimensions: NonZeroU32,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_dimensions")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub dimensions_ext: Option<FieldExtension>,
    /** **Decimal values with spaces, or "E" | "U" | "L"**

 A series of data points which are decimal values separated by a single space (character u20). The special values "E" (error), "L" (below detection limit) and "U" (above detection limit) can also be used in place of a decimal value.

 */
    #[serde(rename = "data")]
    pub data: String,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_data")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub data_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl SampledDataBuilder {
    #[doc = concat!("Finalize building ", "SampledData", ".")]
    pub fn build(self) -> Result<SampledData, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<SampledDataInner> for SampledData {
    fn from(inner: SampledDataInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for SampledData {
    type Target = SampledDataInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for SampledData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl SampledData {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> SampledDataBuilder {
        SampledDataBuilder::default()
    }
}
/** Base StructureDefinition for Signature Type

 **[Signature](http://hl7.org/fhir/StructureDefinition/Signature) v3.0.2**

 A digital Signature - XML DigSig, JWT, Graphical image of signature, etc.

 A digital signature along with supporting context. The signature may be electronic/cryptographic in nature, or a graphical image representing a hand-written signature, or a signature process. Different signature approaches have different utilities.

 The elements of the Signature Resource are for ease of access of these elements. Foro digital signatures (Xml DigSig, JWT), the non-repudiation proof comes from the Signature  validation, which includes validation of the referenced objects (e.g. Resources) (a.k.a., Content) in the XML-Signature Detached form. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Signature(pub Box<SignatureInner>);
/** Base StructureDefinition for Signature Type

 **[Signature](http://hl7.org/fhir/StructureDefinition/Signature) v3.0.2**

 A digital Signature - XML DigSig, JWT, Graphical image of signature, etc.

 A digital signature along with supporting context. The signature may be electronic/cryptographic in nature, or a graphical image representing a hand-written signature, or a signature process. Different signature approaches have different utilities.

 The elements of the Signature Resource are for ease of access of these elements. Foro digital signatures (Xml DigSig, JWT), the non-repudiation proof comes from the Signature  validation, which includes validation of the referenced objects (e.g. Resources) (a.k.a., Content) in the XML-Signature Detached form. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "SignatureBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct SignatureInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[SignatureType](http://hl7.org/fhir/ValueSet/signature-type); Indication of the reason the entity signed the object(s)**

 An indication of the reason that the entity signed this document. This may be explicitly included as part of the signature information and can be used when determining accountability for various actions concerning the document.

 Examples include attesting to: authorship, correct transcription, and witness of specific event. Also known as a &quot;Commitment Type Indication&quot;. */
    #[serde(rename = "type")]
    pub r#type: Vec<Option<Coding>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub r#type_ext: Vec<Option<FieldExtension>>,
    /** **When the signature was created**

 When the digital signature was signed.

 This should agree with the information in the signature. */
    #[serde(rename = "when")]
    pub when: Instant,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_when")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub when_ext: Option<FieldExtension>,
    /** **Who signed**

 A reference to an application-usable description of the identity that signed  (e.g. the signature used their private key).

 This should agree with the information in the signature. */
    #[serde(flatten)]
    pub who: SignatureWho,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub who_ext: Option<SignatureWhoExtension>,
    /** **The party represented**

 A reference to an application-usable description of the identity that is represented by the signature.

 The party that can't sign. For example a child. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub on_behalf_of: Option<SignatureOnBehalfOf>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub on_behalf_of_ext: Option<SignatureOnBehalfOfExtension>,
    #[doc = " **[MimeType](http://www.rfc-editor.org/bcp/bcp13.txt); The technical format of the signature** \n\n A mime type that indicates the technical format of the signature. Important mime types are application/signature+xml for X ML DigSig, application/jwt for JWT, and image/* for a graphical image of a signature, etc. \n\n "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_contentType")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub content_type_ext: Option<FieldExtension>,
    /** **The actual signature content (XML DigSig. JWT, picture, etc.)**

 The base64 encoding of the Signature content. When signature is not recorded electronically this element would be empty.

 Where the signature type is an XML DigSig, the signed content is a FHIR Resource(s), the signature is of the XML form of the Resource(s) using  XML-Signature (XMLDIG) "Detached Signature" form. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "blob")]
    pub blob: Option<Base64Binary>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_blob")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub blob_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl SignatureBuilder {
    #[doc = concat!("Finalize building ", "Signature", ".")]
    pub fn build(self) -> Result<Signature, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<SignatureInner> for Signature {
    fn from(inner: SignatureInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Signature {
    type Target = SignatureInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Signature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Signature {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> SignatureBuilder {
        SignatureBuilder::default()
    }
}
/// Choice of types for the who[x] field in Signature
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SignatureWho {
    /// Variant accepting the Uri type.
    #[serde(rename = "whoUri")]
    Uri(String),
    /// Variant accepting the Reference type.
    #[serde(rename = "whoReference")]
    Reference(Reference),
}
/// Extension value for SignatureWho.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SignatureWhoExtension {
    /// Variant accepting the Uri extension.
    #[serde(rename = "_whoUri")]
    Uri(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_whoReference")]
    Reference(FieldExtension),
}
/// Choice of types for the onBehalfOf[x] field in Signature
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SignatureOnBehalfOf {
    /// Variant accepting the Uri type.
    #[serde(rename = "onBehalfOfUri")]
    Uri(String),
    /// Variant accepting the Reference type.
    #[serde(rename = "onBehalfOfReference")]
    Reference(Reference),
}
/// Extension value for SignatureOnBehalfOf.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SignatureOnBehalfOfExtension {
    /// Variant accepting the Uri extension.
    #[serde(rename = "_onBehalfOfUri")]
    Uri(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_onBehalfOfReference")]
    Reference(FieldExtension),
}
/** Base StructureDefinition for Timing Type

 **[Timing](http://hl7.org/fhir/StructureDefinition/Timing) v3.0.2**

 A timing schedule that specifies an event that may occur multiple times

 Specifies an event that may occur multiple times. Timing schedules are used to record when things are planned, expected or requested to occur. The most common usage is in dosage instructions for medications. They are also used when planning care of various kinds, and may be used for reporting the schedule to which past regular activities were carried out.

 A timing schedule can be either a list of events - intervals on which the event occurs, or a single event with repeating criteria or just repeating criteria with no actual event.  When both event and a repeating specification are provided, the list of events should be understood as an interpretation of the information in the repeat structure. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timing(pub Box<TimingInner>);
/** Base StructureDefinition for Timing Type

 **[Timing](http://hl7.org/fhir/StructureDefinition/Timing) v3.0.2**

 A timing schedule that specifies an event that may occur multiple times

 Specifies an event that may occur multiple times. Timing schedules are used to record when things are planned, expected or requested to occur. The most common usage is in dosage instructions for medications. They are also used when planning care of various kinds, and may be used for reporting the schedule to which past regular activities were carried out.

 A timing schedule can be either a list of events - intervals on which the event occurs, or a single event with repeating criteria or just repeating criteria with no actual event.  When both event and a repeating specification are provided, the list of events should be understood as an interpretation of the information in the repeat structure. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "TimingBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct TimingInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **When the event occurs**

 Identifies specific times when the event occurs.

 */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "event")]
    pub event: Vec<Option<DateTime>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_event")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub event_ext: Vec<Option<FieldExtension>>,
    /** **When the event is to occur**

 A set of rules that describe when the event is scheduled.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "repeat")]
    pub repeat: Option<TimingRepeat>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_repeat")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub repeat_ext: Option<FieldExtension>,
    /** **[TimingAbbreviation](http://hl7.org/fhir/ValueSet/timing-abbreviation); BID | TID | QID | AM | PM | QD | QOD | Q4H | Q6H +**

 A code for the timing schedule. Some codes such as BID are ubiquitous, but many institutions define their own additional codes. If a code is provided, the code is understood to be a complete statement of whatever is specified in the structured timing data, and either the code or the data may be used to interpret the Timing, with the exception that .repeat.bounds still applies over the code (and is not contained in the code).

 BID etc are defined as 'at institutionally specified times'. For example, an institution may choose that BID is "always at 7am and 6pm".  If it is inappropriate for this choice to be made, the code BID should not be used. Instead, a distinct organization-specific code should be used in place of the HL7-defined BID code and/or the a structured representation should be used (in this case, specifying the two event times). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<CodeableConcept>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl TimingBuilder {
    #[doc = concat!("Finalize building ", "Timing", ".")]
    pub fn build(self) -> Result<Timing, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<TimingInner> for Timing {
    fn from(inner: TimingInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for Timing {
    type Target = TimingInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Timing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Timing {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> TimingBuilder {
        TimingBuilder::default()
    }
}
/// Sub-fields of the repeat field in Timing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "TimingRepeatBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct TimingRepeat {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Length/Range of lengths, or (Start and/or end) limits**

 Either a duration for the length of the timing schedule, a range of possible length, or outer bounds for start and/or end limits of the timing schedule.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub bounds: Option<TimingRepeatBounds>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub bounds_ext: Option<TimingRepeatBoundsExtension>,
    /** **Number of times to repeat**

 A total count of the desired number of repetitions.

 If you have both bounds and count, then this should be understood as within the bounds period, until count times happens. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "count")]
    pub count: Option<i32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_count")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub count_ext: Option<FieldExtension>,
    /** **Maximum number of times to repeat**

 A maximum value for the count of the desired repetitions (e.g. do something 6-8 times).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "countMax")]
    pub count_max: Option<i32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_countMax")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub count_max_ext: Option<FieldExtension>,
    /** **How long when it happens**

 How long this thing happens for when it happens.

 For some events the duration is part of the definition of the event (e.g. IV infusions, where the duration is implicit in the specified quantity and rate). For others, it's part of the timing specification (e.g. exercise). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "duration")]
    pub duration: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_duration")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub duration_ext: Option<FieldExtension>,
    /** **How long when it happens (Max)**

 The upper limit of how long this thing happens for when it happens.

 For some events the duration is part of the definition of the event (e.g. IV infusions, where the duration is implicit in the specified quantity and rate). For others, it's part of the timing specification (e.g. exercise). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "durationMax")]
    pub duration_max: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_durationMax")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub duration_max_ext: Option<FieldExtension>,
    /** **[UnitsOfTime](http://hl7.org/fhir/ValueSet/units-of-time); s | min | h | d | wk | mo | a - unit of time (UCUM)**

 The units of time for the duration, in UCUM units.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "durationUnit")]
    pub duration_unit: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_durationUnit")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub duration_unit_ext: Option<FieldExtension>,
    /** **Event occurs frequency times per period**

 The number of times to repeat the action within the specified period / period range (i.e. both period and periodMax provided).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "frequency")]
    pub frequency: Option<i32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_frequency")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub frequency_ext: Option<FieldExtension>,
    /** **Event occurs up to frequencyMax times per period**

 If present, indicates that the frequency is a range - so to repeat between [frequency] and [frequencyMax] times within the period or period range.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "frequencyMax")]
    pub frequency_max: Option<i32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_frequencyMax")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub frequency_max_ext: Option<FieldExtension>,
    /** **Event occurs frequency times per period**

 Indicates the duration of time over which repetitions are to occur; e.g. to express "3 times per day", 3 would be the frequency and "1 day" would be the period.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "period")]
    pub period: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_period")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_ext: Option<FieldExtension>,
    /** **Upper limit of period (3-4 hours)**

 If present, indicates that the period is a range from [period] to [periodMax], allowing expressing concepts such as "do this once every 3-5 days.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "periodMax")]
    pub period_max: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_periodMax")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_max_ext: Option<FieldExtension>,
    /** **[UnitsOfTime](http://hl7.org/fhir/ValueSet/units-of-time); s | min | h | d | wk | mo | a - unit of time (UCUM)**

 The units of time for the period in UCUM units.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "periodUnit")]
    pub period_unit: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_periodUnit")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub period_unit_ext: Option<FieldExtension>,
    /** **[DayOfWeek](http://hl7.org/fhir/ValueSet/days-of-week); mon | tue | wed | thu | fri | sat | sun**

 If one or more days of week is provided, then the action happens only on the specified day(s).

 If no days are specified, the action is assumed to happen every day as otherwise specified. The elements frequency and period cannot be used as well as dayOfWeek. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "dayOfWeek")]
    pub day_of_week: Vec<Option<codes::DaysOfWeek>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_dayOfWeek")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub day_of_week_ext: Vec<Option<FieldExtension>>,
    /** **Time of day for action**

 Specified time of day for action to take place.

 When time of day is specified, it is inferred that the action happens every day (as filtered by dayofWeek) on the specified times. The elements when, frequency and period cannot be used as well as timeOfDay. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "timeOfDay")]
    pub time_of_day: Vec<Option<Time>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_timeOfDay")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub time_of_day_ext: Vec<Option<FieldExtension>>,
    /** **[EventTiming](http://hl7.org/fhir/ValueSet/event-timing); Regular life events the event is tied to**

 Real world events that the occurrence of the event should be tied to.

 When more than one event is listed, the event is tied to the union of the specified events. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "when")]
    pub when: Vec<Option<codes::EventTiming>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_when")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub when_ext: Vec<Option<FieldExtension>>,
    /** **Minutes from event (before or after)**

 The number of minutes from the event. If the event code does not indicate whether the minutes is before or after the event, then the offset is assumed to be after the event.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "offset")]
    pub offset: Option<u32>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_offset")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub offset_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl TimingRepeat {
    ///Start building a new instance
    #[must_use]
    pub fn builder() -> TimingRepeatBuilder {
        TimingRepeatBuilder::default()
    }
}
/// Choice of types for the bounds[x] field in TimingRepeat
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimingRepeatBounds {
    /// Variant accepting the Duration type.
    #[serde(rename = "boundsDuration")]
    Duration(Duration),
    /// Variant accepting the Range type.
    #[serde(rename = "boundsRange")]
    Range(Range),
    /// Variant accepting the Period type.
    #[serde(rename = "boundsPeriod")]
    Period(Period),
}
/// Extension value for TimingRepeatBounds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimingRepeatBoundsExtension {
    /// Variant accepting the Duration extension.
    #[serde(rename = "_boundsDuration")]
    Duration(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_boundsRange")]
    Range(FieldExtension),
    /// Variant accepting the Period extension.
    #[serde(rename = "_boundsPeriod")]
    Period(FieldExtension),
}
/** Base StructureDefinition for TriggerDefinition Type

 **[TriggerDefinition](http://hl7.org/fhir/StructureDefinition/TriggerDefinition) v3.0.2**

 Defines an expected trigger for a module

 A description of a triggering event.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TriggerDefinition(pub Box<TriggerDefinitionInner>);
/** Base StructureDefinition for TriggerDefinition Type

 **[TriggerDefinition](http://hl7.org/fhir/StructureDefinition/TriggerDefinition) v3.0.2**

 Defines an expected trigger for a module

 A description of a triggering event.

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "TriggerDefinitionBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct TriggerDefinitionInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[TriggerType](http://hl7.org/fhir/ValueSet/trigger-type); named-event | periodic | data-added | data-modified | data-removed | data-accessed | data-access-ended**

 The type of triggering event.

 */
    #[serde(rename = "type")]
    pub r#type: codes::TriggerType,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_type")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub r#type_ext: Option<FieldExtension>,
    /** **Triggering event name**

 The name of the event (if this is a named-event trigger).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "eventName")]
    pub event_name: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_eventName")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub event_name_ext: Option<FieldExtension>,
    /** **Timing of the event**

 The timing of the event (if this is a period trigger).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(flatten)]
    pub event_timing: Option<TriggerDefinitionEventTiming>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub event_timing_ext: Option<TriggerDefinitionEventTimingExtension>,
    /** **Triggering data of the event**

 The triggering data of the event (if this is a data trigger).

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "eventData")]
    pub event_data: Option<DataRequirement>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_eventData")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub event_data_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl TriggerDefinitionBuilder {
    #[doc = concat!("Finalize building ", "TriggerDefinition", ".")]
    pub fn build(self) -> Result<TriggerDefinition, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<TriggerDefinitionInner> for TriggerDefinition {
    fn from(inner: TriggerDefinitionInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for TriggerDefinition {
    type Target = TriggerDefinitionInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for TriggerDefinition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TriggerDefinition {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> TriggerDefinitionBuilder {
        TriggerDefinitionBuilder::default()
    }
}
/// Choice of types for the eventTiming[x] field in TriggerDefinition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TriggerDefinitionEventTiming {
    /// Variant accepting the Timing type.
    #[serde(rename = "eventTimingTiming")]
    Timing(Timing),
    /// Variant accepting the Reference type.
    #[serde(rename = "eventTimingReference")]
    Reference(Reference),
    /// Variant accepting the Date type.
    #[serde(rename = "eventTimingDate")]
    Date(Date),
    /// Variant accepting the DateTime type.
    #[serde(rename = "eventTimingDateTime")]
    DateTime(DateTime),
}
/// Extension value for TriggerDefinitionEventTiming.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TriggerDefinitionEventTimingExtension {
    /// Variant accepting the Timing extension.
    #[serde(rename = "_eventTimingTiming")]
    Timing(FieldExtension),
    /// Variant accepting the Reference extension.
    #[serde(rename = "_eventTimingReference")]
    Reference(FieldExtension),
    /// Variant accepting the Date extension.
    #[serde(rename = "_eventTimingDate")]
    Date(FieldExtension),
    /// Variant accepting the DateTime extension.
    #[serde(rename = "_eventTimingDateTime")]
    DateTime(FieldExtension),
}
/** Base StructureDefinition for UsageContext Type

 **[UsageContext](http://hl7.org/fhir/StructureDefinition/UsageContext) v3.0.2**

 Describes the context of use for a conformance or knowledge resource

 Specifies clinical/business/etc metadata that can be used to retrieve, index and/or categorize an artifact. This metadata can either be specific to the applicable population (e.g., age category, DRG) or the specific context of care (e.g., venue, care setting, provider of care).

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UsageContext(pub Box<UsageContextInner>);
/** Base StructureDefinition for UsageContext Type

 **[UsageContext](http://hl7.org/fhir/StructureDefinition/UsageContext) v3.0.2**

 Describes the context of use for a conformance or knowledge resource

 Specifies clinical/business/etc metadata that can be used to retrieve, index and/or categorize an artifact. This metadata can either be specific to the applicable population (e.g., age category, DRG) or the specific context of care (e.g., venue, care setting, provider of care).

 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "UsageContextBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct UsageContextInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **[UsageContextType](http://hl7.org/fhir/ValueSet/usage-context-type); Type of context being specified**

 A code that identifies the type of context being specified by this usage context.

 */
    #[serde(rename = "code")]
    pub code: Coding,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_ext: Option<FieldExtension>,
    /** **Value that defines the context**

 A value that defines the context specified in this context of use. The interpretation of the value is defined by the code.

 */
    #[serde(flatten)]
    pub value: UsageContextValue,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<UsageContextValueExtension>,
}
#[cfg(feature = "builders")]
impl UsageContextBuilder {
    #[doc = concat!("Finalize building ", "UsageContext", ".")]
    pub fn build(self) -> Result<UsageContext, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<UsageContextInner> for UsageContext {
    fn from(inner: UsageContextInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for UsageContext {
    type Target = UsageContextInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for UsageContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl UsageContext {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> UsageContextBuilder {
        UsageContextBuilder::default()
    }
}
/// Choice of types for the value[x] field in UsageContext
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UsageContextValue {
    /// Variant accepting the CodeableConcept type.
    #[serde(rename = "valueCodeableConcept")]
    CodeableConcept(CodeableConcept),
    /// Variant accepting the Quantity type.
    #[serde(rename = "valueQuantity")]
    Quantity(Quantity),
    /// Variant accepting the Range type.
    #[serde(rename = "valueRange")]
    Range(Range),
}
/// Extension value for UsageContextValue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UsageContextValueExtension {
    /// Variant accepting the CodeableConcept extension.
    #[serde(rename = "_valueCodeableConcept")]
    CodeableConcept(FieldExtension),
    /// Variant accepting the Quantity extension.
    #[serde(rename = "_valueQuantity")]
    Quantity(FieldExtension),
    /// Variant accepting the Range extension.
    #[serde(rename = "_valueRange")]
    Range(FieldExtension),
}
/** A fixed quantity (no comparator)

 **[SimpleQuantity](http://hl7.org/fhir/StructureDefinition/SimpleQuantity) v3.0.2**

 A fixed quantity (no comparator)

 The comparator is not used on a SimpleQuantity

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SimpleQuantity(pub Box<SimpleQuantityInner>);
/** A fixed quantity (no comparator)

 **[SimpleQuantity](http://hl7.org/fhir/StructureDefinition/SimpleQuantity) v3.0.2**

 A fixed quantity (no comparator)

 The comparator is not used on a SimpleQuantity

 The context of use may frequently define what kind of quantity this is and therefore what kind of units can be used. The context of use may also restrict the values for the comparator. */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "SimpleQuantityBuilder",
        build_fn(error = "crate::error::BuilderError", name = "build_inner"),
    )
)]
pub struct SimpleQuantityInner {
    /** **xml:id (or equivalent in JSON)**

 unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "id")]
    pub id: Option<String>,
    /** **Additional Content defined by implementations**

 May be used to represent additional information that is not part of the basic definition of the element. In order to make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension.

 There can be no stigma associated with the use of extensions by any application, project, or standard - regardless of the institution or jurisdiction that uses or defines the extensions.  The use of extensions is what allows the FHIR specification to retain a core level of simplicity for everyone. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "extension")]
    pub extension: Vec<Extension>,
    /** **Numerical value (with implicit precision)**

 The value of the measured amount. The value includes an implicit precision in the presentation of the value.

 The implicit precision in the value should always be honored. Monetary values have their own rules for handling precision (refer to standard accounting text books). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_value")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub value_ext: Option<FieldExtension>,
    /** **[QuantityComparator](http://hl7.org/fhir/ValueSet/quantity-comparator); < | <= | >= | > - how to understand the value**

 Not allowed to be used in this context

 This is labeled as "Is Modifier" because the comparator modifies the interpretation of the value significantly. If there is no comparator, then there is no modification of the value. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "comparator")]
    pub comparator: Vec<Option<codes::QuantityComparator>>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "_comparator")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub comparator_ext: Vec<Option<FieldExtension>>,
    /** **Unit representation**

 A human-readable form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "unit")]
    pub unit: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_unit")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub unit_ext: Option<FieldExtension>,
    /** **System that defines coded unit form**

 The identification of the system that provides the coded form of the unit.

 */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "system")]
    pub system: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_system")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub system_ext: Option<FieldExtension>,
    /** **Coded form of the unit**

 A computer processable form of the unit in some unit representation system.

 The preferred system is UCUM, but SNOMED CT can also be used (for customary units) or ISO 4217 for currency.  The context of use may additionally require a code from a particular system. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    #[serde(rename = "code")]
    pub code: Option<String>,
    /// Extension field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_code")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub code_ext: Option<FieldExtension>,
}
#[cfg(feature = "builders")]
impl SimpleQuantityBuilder {
    #[doc = concat!("Finalize building ", "SimpleQuantity", ".")]
    pub fn build(self) -> Result<SimpleQuantity, crate::error::BuilderError> {
        self.build_inner().map(Into::into)
    }
}
impl From<SimpleQuantityInner> for SimpleQuantity {
    fn from(inner: SimpleQuantityInner) -> Self {
        Self(Box::new(inner))
    }
}
impl ::core::ops::Deref for SimpleQuantity {
    type Target = SimpleQuantityInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for SimpleQuantity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl SimpleQuantity {
    /// Start building an instance.
    #[cfg(feature = "builders")]
    #[must_use]
    pub fn builder() -> SimpleQuantityBuilder {
        SimpleQuantityBuilder::default()
    }
}
/// Extension of a field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builders", derive(Builder))]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "builders",
    builder(
        pattern = "owned",
        name = "FieldExtensionBuilder",
        build_fn(error = "crate::error::BuilderError")
    )
)]
pub struct FieldExtension {
    /// Unique id for inter-element referencing
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builders", builder(default))]
    pub extension: Vec<Extension>,
}
#[cfg(feature = "builders")]
impl FieldExtension {
    ///Start building a new FieldExtension.
    #[must_use]
    pub fn builder() -> FieldExtensionBuilder {
        FieldExtensionBuilder::default()
    }
}
