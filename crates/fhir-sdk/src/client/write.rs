//! Home of the ResourceWrite trait.

use std::future::Future;

#[cfg(feature = "r4b")]
use fhir_model::r4b;
#[cfg(feature = "r5")]
use fhir_model::r5;
#[cfg(feature = "stu3")]
use fhir_model::stu3;
use serde::Serialize;

use super::{error::Error, Client};

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

#[cfg(feature = "stu3")]
impl<R> ResourceWrite<super::FhirStu3> for R
where
	R: stu3::resources::NamedResource + stu3::resources::BaseResource + Serialize + Send + Sync,
{
	async fn update(
		&mut self,
		conditional: bool,
		client: &Client<super::FhirStu3>,
	) -> Result<bool, Error> {
		let (created, version_id) = client.update(self, conditional).await?;
		if let Some(meta) = self.meta_mut() {
			meta.version_id = Some(version_id);
		} else {
			// Meta does not require any field and will succeed building.
			#[allow(clippy::unwrap_used)]
			self.set_meta(Some(
				stu3::types::Meta::builder().version_id(version_id).build().unwrap(),
			));
		}
		Ok(created)
	}

	async fn create(&mut self, client: &Client<super::FhirStu3>) -> Result<String, Error> {
		let (id, version_id) = client.create(self).await?;
		self.set_id(Some(id.clone()));
		if let Some(meta) = self.meta_mut() {
			meta.version_id = version_id;
		} else if let Some(version_id) = version_id {
			// Meta does not require any field and will succeed building.
			#[allow(clippy::unwrap_used)]
			self.set_meta(Some(
				stu3::types::Meta::builder().version_id(version_id).build().unwrap(),
			));
		}
		Ok(id)
	}

	async fn delete(self, client: &Client<super::FhirStu3>) -> Result<(), Error> {
		let id = self.id().as_ref().ok_or(Error::MissingId)?;
		client.delete(R::TYPE, id).await?;
		Ok(())
	}
}

#[cfg(feature = "r4b")]
impl<R> ResourceWrite<super::FhirR4B> for R
where
	R: r4b::resources::NamedResource + r4b::resources::BaseResource + Serialize + Send + Sync,
{
	async fn update(
		&mut self,
		conditional: bool,
		client: &Client<super::FhirR4B>,
	) -> Result<bool, Error> {
		let (created, version_id) = client.update(self, conditional).await?;
		if let Some(meta) = self.meta_mut() {
			meta.version_id = Some(version_id);
		} else {
			// Meta does not require any field and will succeed building.
			#[allow(clippy::unwrap_used)]
			self.set_meta(Some(
				r4b::types::Meta::builder().version_id(version_id).build().unwrap(),
			));
		}
		Ok(created)
	}

	async fn create(&mut self, client: &Client<super::FhirR4B>) -> Result<String, Error> {
		let (id, version_id) = client.create(self).await?;
		self.set_id(Some(id.clone()));
		if let Some(meta) = self.meta_mut() {
			meta.version_id = version_id;
		} else if let Some(version_id) = version_id {
			// Meta does not require any field and will succeed building.
			#[allow(clippy::unwrap_used)]
			self.set_meta(Some(
				r4b::types::Meta::builder().version_id(version_id).build().unwrap(),
			));
		}
		Ok(id)
	}

	async fn delete(self, client: &Client<super::FhirR4B>) -> Result<(), Error> {
		let id = self.id().as_ref().ok_or(Error::MissingId)?;
		client.delete(R::TYPE, id).await?;
		Ok(())
	}
}

#[cfg(feature = "r5")]
impl<R> ResourceWrite<super::FhirR5> for R
where
	R: r5::resources::NamedResource + r5::resources::BaseResource + Serialize + Send + Sync,
{
	async fn update(
		&mut self,
		conditional: bool,
		client: &Client<super::FhirR5>,
	) -> Result<bool, Error> {
		let (created, version_id) = client.update(self, conditional).await?;
		if let Some(meta) = self.meta_mut() {
			meta.version_id = Some(version_id);
		} else {
			// Meta does not require any field and will succeed building.
			#[allow(clippy::unwrap_used)]
			self.set_meta(Some(r5::types::Meta::builder().version_id(version_id).build().unwrap()));
		}
		Ok(created)
	}

	async fn create(&mut self, client: &Client<super::FhirR5>) -> Result<String, Error> {
		let (id, version_id) = client.create(self).await?;
		self.set_id(Some(id.clone()));
		if let Some(meta) = self.meta_mut() {
			meta.version_id = version_id;
		} else if let Some(version_id) = version_id {
			// Meta does not require any field and will succeed building.
			#[allow(clippy::unwrap_used)]
			self.set_meta(Some(r5::types::Meta::builder().version_id(version_id).build().unwrap()));
		}
		Ok(id)
	}

	async fn delete(self, client: &Client<super::FhirR5>) -> Result<(), Error> {
		let id = self.id().as_ref().ok_or(Error::MissingId)?;
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
	type Version;

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

#[cfg(feature = "stu3")]
impl AnyResourceWrite for stu3::resources::Resource {
	type Version = super::FhirStu3;

	async fn update(
		&mut self,
		conditional: bool,
		client: &Client<super::FhirStu3>,
	) -> Result<bool, Error> {
		let id = self.as_base_resource().id().as_deref().ok_or(Error::MissingId)?;
		let version_id = conditional
			.then(|| {
				self.as_base_resource()
					.meta()
					.as_ref()
					.and_then(|meta| meta.version_id.as_deref())
					.ok_or(Error::MissingVersionId)
			})
			.transpose()?;
		let (created, version_id) =
			client.update_generic(self.resource_type(), id, self, version_id).await?;
		if let Some(meta) = self.as_base_resource_mut().meta_mut() {
			meta.version_id = Some(version_id);
		} else {
			// Meta does not require any field and will succeed building.
			#[allow(clippy::unwrap_used)]
			self.as_base_resource_mut().set_meta(Some(
				stu3::types::Meta::builder().version_id(version_id).build().unwrap(),
			));
		}
		Ok(created)
	}

	async fn create(&mut self, client: &Client<super::FhirStu3>) -> Result<String, Error> {
		let (id, version_id) = client.create_generic(self.resource_type(), self).await?;
		self.as_base_resource_mut().set_id(Some(id.clone()));
		if let Some(meta) = self.as_base_resource_mut().meta_mut() {
			meta.version_id = version_id;
		} else if let Some(version_id) = version_id {
			// Meta does not require any field and will succeed building.
			#[allow(clippy::unwrap_used)]
			self.as_base_resource_mut().set_meta(Some(
				stu3::types::Meta::builder().version_id(version_id).build().unwrap(),
			));
		}
		Ok(id)
	}

	async fn delete(self, client: &Client<super::FhirStu3>) -> Result<(), Error> {
		let id = self.as_base_resource().id().as_ref().ok_or(Error::MissingId)?;
		client.delete(self.resource_type(), id).await?;
		Ok(())
	}
}

#[cfg(feature = "r4b")]
impl AnyResourceWrite for r4b::resources::Resource {
	type Version = super::FhirR4B;

	async fn update(
		&mut self,
		conditional: bool,
		client: &Client<super::FhirR4B>,
	) -> Result<bool, Error> {
		let id = self.as_base_resource().id().as_deref().ok_or(Error::MissingId)?;
		let version_id = conditional
			.then(|| {
				self.as_base_resource()
					.meta()
					.as_ref()
					.and_then(|meta| meta.version_id.as_deref())
					.ok_or(Error::MissingVersionId)
			})
			.transpose()?;
		let (created, version_id) =
			client.update_generic(self.resource_type(), id, self, version_id).await?;
		if let Some(meta) = self.as_base_resource_mut().meta_mut() {
			meta.version_id = Some(version_id);
		} else {
			// Meta does not require any field and will succeed building.
			#[allow(clippy::unwrap_used)]
			self.as_base_resource_mut().set_meta(Some(
				r4b::types::Meta::builder().version_id(version_id).build().unwrap(),
			));
		}
		Ok(created)
	}

	async fn create(&mut self, client: &Client<super::FhirR4B>) -> Result<String, Error> {
		let (id, version_id) = client.create_generic(self.resource_type(), self).await?;
		self.as_base_resource_mut().set_id(Some(id.clone()));
		if let Some(meta) = self.as_base_resource_mut().meta_mut() {
			meta.version_id = version_id;
		} else if let Some(version_id) = version_id {
			// Meta does not require any field and will succeed building.
			#[allow(clippy::unwrap_used)]
			self.as_base_resource_mut().set_meta(Some(
				r4b::types::Meta::builder().version_id(version_id).build().unwrap(),
			));
		}
		Ok(id)
	}

	async fn delete(self, client: &Client<super::FhirR4B>) -> Result<(), Error> {
		let id = self.as_base_resource().id().as_ref().ok_or(Error::MissingId)?;
		client.delete(self.resource_type(), id).await?;
		Ok(())
	}
}

#[cfg(feature = "r5")]
impl AnyResourceWrite for r5::resources::Resource {
	type Version = super::FhirR5;

	async fn update(
		&mut self,
		conditional: bool,
		client: &Client<super::FhirR5>,
	) -> Result<bool, Error> {
		let id = self.as_base_resource().id().as_deref().ok_or(Error::MissingId)?;
		let version_id = conditional
			.then(|| {
				self.as_base_resource()
					.meta()
					.as_ref()
					.and_then(|meta| meta.version_id.as_deref())
					.ok_or(Error::MissingVersionId)
			})
			.transpose()?;
		let (created, version_id) =
			client.update_generic(self.resource_type(), id, self, version_id).await?;
		if let Some(meta) = self.as_base_resource_mut().meta_mut() {
			meta.version_id = Some(version_id);
		} else {
			// Meta does not require any field and will succeed building.
			#[allow(clippy::unwrap_used)]
			self.as_base_resource_mut()
				.set_meta(Some(r5::types::Meta::builder().version_id(version_id).build().unwrap()));
		}
		Ok(created)
	}

	async fn create(&mut self, client: &Client<super::FhirR5>) -> Result<String, Error> {
		let (id, version_id) = client.create_generic(self.resource_type(), self).await?;
		self.as_base_resource_mut().set_id(Some(id.clone()));
		if let Some(meta) = self.as_base_resource_mut().meta_mut() {
			meta.version_id = version_id;
		} else if let Some(version_id) = version_id {
			// Meta does not require any field and will succeed building.
			#[allow(clippy::unwrap_used)]
			self.as_base_resource_mut()
				.set_meta(Some(r5::types::Meta::builder().version_id(version_id).build().unwrap()));
		}
		Ok(id)
	}

	async fn delete(self, client: &Client<super::FhirR5>) -> Result<(), Error> {
		let id = self.as_base_resource().id().as_ref().ok_or(Error::MissingId)?;
		client.delete(self.resource_type(), id).await?;
		Ok(())
	}
}
