//! Home of the ResourceWrite trait.

use std::future::Future;

use fhir_model::for_all_versions;
use reqwest::StatusCode;
use serde::Serialize;

use super::super::{error::Error, Client};
use crate::{
	extensions::{AnyResource, GenericResource},
	version::{fhir_version, FhirVersion},
};

/// A trait to write resources to the FHIR server, mutating the interior id and
/// version_id so that the resource is up to date for future update requests.
pub trait ResourceWrite<Version>: Serialize + Send + Sync {
	/// Update the current version of the resource on the server. Returns
	/// whether the resource was created.
	fn update(
		&mut self,
		conditional: bool,
		client: &Client<Version>,
	) -> impl Future<Output = Result<bool, Error>> + Send;
	/// Create this resource on the server. Returns the resource ID.
	fn create(
		&mut self,
		client: &Client<Version>,
	) -> impl Future<Output = Result<String, Error>> + Send;
	/// Delete this resource on the FHIR server.
	fn delete(self, client: &Client<Version>) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<R, V> ResourceWrite<V> for R
where
	R: AnyResource<V> + Serialize + Send + Sync,
	V: FhirVersion,
	Error: From<(StatusCode, V::OperationOutcome)>,
{
	async fn update(&mut self, conditional: bool, client: &Client<V>) -> Result<bool, Error> {
		let id = self.id().ok_or(Error::MissingId)?;
		let version_id =
			conditional.then(|| self.version_id().ok_or(Error::MissingVersionId)).transpose()?;
		let (created, version_id) =
			client.update_generic(R::TYPE_STR, id, self, version_id).await?;
		self.set_version_id(version_id);
		Ok(created)
	}

	async fn create(&mut self, client: &Client<V>) -> Result<String, Error> {
		let (id, version_id) = client.create_generic(R::TYPE_STR, self).await?;
		self.set_id(id.clone());
		if let Some(version) = version_id {
			self.set_version_id(version);
		}
		Ok(id)
	}

	async fn delete(self, client: &Client<V>) -> Result<(), Error> {
		let id = self.id().ok_or(Error::MissingId)?;
		client.delete(R::TYPE, id).await?;
		Ok(())
	}
}

/// A trait to write the resource enum to the FHIR server, mutating the interior
/// id and version_id so that the resource is up to date for future update
/// requests.
/// This trait sadly needs to be separate to [`ResourceWrite`],
/// because we cannot both implement the generic trait as well as implementing
/// it on a type that fhir-model could implement the required traits for.
pub trait AnyResourceWrite: Serialize + Send + Sync {
	/// FHIR client version type.
	type Version: FhirVersion;

	/// Update the current version of the resource on the server. Returns
	/// whether the resource was created.
	fn update(
		&mut self,
		conditional: bool,
		client: &Client<Self::Version>,
	) -> impl Future<Output = Result<bool, Error>> + Send;
	/// Create this resource on the server. Returns the resource ID.
	fn create(
		&mut self,
		client: &Client<Self::Version>,
	) -> impl Future<Output = Result<String, Error>> + Send;
	/// Delete this resource on the FHIR server.
	fn delete(
		self,
		client: &Client<Self::Version>,
	) -> impl Future<Output = Result<(), Error>> + Send;
}

/// Implement `ResourceWrite` on generic resource enums. Sadly needs to be
/// separate currently, see [AnyResource] (`Resource` can only implement
/// `AnyResource<GenericResource>`).
macro_rules! impl_generic_resource_write {
	($version:ident) => {
		use fhir_model::$version;

		impl AnyResourceWrite for $version::resources::Resource {
			type Version = fhir_version!($version);

			async fn update(
				&mut self,
				conditional: bool,
				client: &Client<Self::Version>,
			) -> Result<bool, Error> {
				let id = self.as_base_resource().id().as_deref().ok_or(Error::MissingId)?;
				let version_id = conditional
					.then(|| self.version_id().ok_or(Error::MissingVersionId))
					.transpose()?;
				let (created, version_id) = client
					.update_generic(self.resource_type().as_str(), id, self, version_id)
					.await?;
				self.set_version_id(version_id);
				Ok(created)
			}

			async fn create(&mut self, client: &Client<Self::Version>) -> Result<String, Error> {
				let (id, version_id) =
					client.create_generic(self.resource_type().as_str(), self).await?;
				self.as_base_resource_mut().set_id(Some(id.clone()));
				if let Some(version) = version_id {
					self.set_version_id(version);
				}
				Ok(id)
			}

			async fn delete(self, client: &Client<Self::Version>) -> Result<(), Error> {
				let id = self.as_base_resource().id().as_deref().ok_or(Error::MissingId)?;
				client.delete(self.resource_type(), id).await?;
				Ok(())
			}
		}
	};
}
for_all_versions!(impl_generic_resource_write);
