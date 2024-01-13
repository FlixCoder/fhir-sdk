//! Home of the ResourceWrite trait.

use async_trait::async_trait;
use model::{
	resources::{BaseResource, NamedResource},
	types::Meta,
};
use serde::Serialize;

use super::{error::Error, model, Client};

/// A trait to write resources to the FHIR server, mutating the interior id and
/// version_id so that the resource is up to date for future update requests.
#[async_trait]
pub trait ResourceWrite: NamedResource + BaseResource + Serialize + Send + Sync {
	/// Update the current version of the resource on the server. Returns
	/// whether the resource was created.
	async fn update(&mut self, conditional: bool, client: &Client) -> Result<bool, Error>;
	/// Create this resource on the server. Returns the resource ID.
	async fn create(&mut self, client: &Client) -> Result<String, Error>;
	/// Delete this resource on the FHIR server.
	async fn delete(self, client: &Client) -> Result<(), Error>;
}

#[async_trait]
impl<R: NamedResource + BaseResource + Serialize + Send + Sync> ResourceWrite for R {
	async fn update(&mut self, conditional: bool, client: &Client) -> Result<bool, Error> {
		let (created, version_id) = client.update(self, conditional).await?;
		if let Some(meta) = self.meta_mut() {
			meta.version_id = Some(version_id);
		} else {
			// Meta does not require any field and will succeed building.
			#[allow(clippy::unwrap_used)]
			self.set_meta(Some(Meta::builder().version_id(version_id).build().unwrap()));
		}
		Ok(created)
	}

	async fn create(&mut self, client: &Client) -> Result<String, Error> {
		let (id, version_id) = client.create(self).await?;
		self.set_id(Some(id.clone()));
		if let Some(meta) = self.meta_mut() {
			meta.version_id = version_id;
		} else if let Some(version_id) = version_id {
			// Meta does not require any field and will succeed building.
			#[allow(clippy::unwrap_used)]
			self.set_meta(Some(Meta::builder().version_id(version_id).build().unwrap()));
		}
		Ok(id)
	}

	async fn delete(self, client: &Client) -> Result<(), Error> {
		let id = self.id().as_ref().ok_or(Error::MissingId)?;
		client.delete(R::TYPE, id).await?;
		Ok(())
	}
}
