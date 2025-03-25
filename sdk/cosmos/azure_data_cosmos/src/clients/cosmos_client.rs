// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::DatabaseClient,
    models::{DatabaseProperties, DatabaseQueryResults},
    pipeline::{AuthorizationPolicy, CosmosPipeline},
    resource_context::{ResourceLink, ResourceType},
    CosmosClientOptions, CreateDatabaseOptions, Query, QueryDatabasesOptions,
};
use azure_core::{
    credentials::TokenCredential,
    http::{request::Request, response::Response, Method, Pager, Url},
};
use serde::Serialize;
use std::sync::Arc;

#[cfg(feature = "key_auth")]
use azure_core::credentials::Secret;

/// Client for Azure Cosmos DB.
#[derive(Debug, Clone)]
pub struct CosmosClient {
    databases_link: ResourceLink,
    pipeline: CosmosPipeline,
}

impl CosmosClient {
    /// Creates a new CosmosClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Cosmos DB account, for example `https://myaccount.documents.azure.com/`.
    /// * `credential` - An implementation of [`TokenCredential`](azure_core::credentials::TokenCredential) that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use std::sync::Arc;
    /// use azure_data_cosmos::CosmosClient;
    ///
    /// let credential = azure_identity::DefaultAzureCredential::new().unwrap();
    /// let client = CosmosClient::new("https://myaccount.documents.azure.com/", credential, None).unwrap();
    /// ```
    pub fn new(
        endpoint: &str,
        credential: Arc<dyn TokenCredential>,
        options: Option<CosmosClientOptions>,
    ) -> azure_core::Result<Self> {
        let options = options.unwrap_or_default();
        Ok(Self {
            databases_link: ResourceLink::root(ResourceType::Databases),
            pipeline: CosmosPipeline::new(
                endpoint.parse()?,
                AuthorizationPolicy::from_token_credential(credential),
                options.client_options,
            ),
        })
    }

    /// Creates a new CosmosClient, using key authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Cosmos DB account, for example `https://myaccount.documents.azure.com/`.
    /// * `key` - The key to use when authenticating.
    /// * `options` - Optional configuration for the client.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::CosmosClient;
    /// use azure_core::credentials::Secret;
    ///
    /// let client = CosmosClient::with_key("https://myaccount.documents.azure.com/", Secret::from("my_key"), None).unwrap();
    /// ```
    #[cfg(feature = "key_auth")]
    pub fn with_key(
        endpoint: &str,
        key: Secret,
        options: Option<CosmosClientOptions>,
    ) -> azure_core::Result<Self> {
        let options = options.unwrap_or_default();
        Ok(Self {
            databases_link: ResourceLink::root(ResourceType::Databases),
            pipeline: CosmosPipeline::new(
                endpoint.parse()?,
                AuthorizationPolicy::from_shared_key(key),
                options.client_options,
            ),
        })
    }

    /// Gets a [`DatabaseClient`] that can be used to access the database with the specified ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the database.
    pub fn database_client(&self, id: &str) -> DatabaseClient {
        DatabaseClient::new(self.pipeline.clone(), id)
    }

    /// Gets the endpoint of the database account this client is connected to.
    pub fn endpoint(&self) -> &Url {
        &self.pipeline.endpoint
    }

    /// Executes a query against databases in the account.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to execute.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// The `query` parameter accepts anything that can be transformed [`Into`] a [`Query`].
    /// This allows simple queries without parameters to be expressed easily:
    ///
    /// ```rust,no_run
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// # use azure_data_cosmos::CosmosClient;
    /// # let client: CosmosClient = panic!("this is a non-running example");
    /// let dbs = client.query_databases(
    ///     "SELECT * FROM dbs",
    ///     None)?;
    /// # }
    /// ```
    ///
    /// See [`Query`] for more information on how to specify a query.
    pub fn query_databases(
        &self,
        query: impl Into<Query>,
        options: Option<QueryDatabasesOptions<'_>>,
    ) -> azure_core::Result<Pager<DatabaseQueryResults>> {
        let options = options.unwrap_or_default();
        let url = self.pipeline.url(&self.databases_link);
        let base_request = Request::new(url, Method::Post);

        self.pipeline.send_query_request(
            options.method_options.context,
            query.into(),
            base_request,
            self.databases_link.clone(),
        )
    }

    /// Creates a new database.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    /// # Arguments
    /// * `id` - The ID of the new database.
    /// * `options` - Optional parameters for the request.
    pub async fn create_database(
        &self,
        id: &str,
        options: Option<CreateDatabaseOptions<'_>>,
    ) -> azure_core::Result<Response<DatabaseProperties>> {
        let options = options.unwrap_or_default();

        #[derive(Serialize)]
        struct RequestBody<'a> {
            id: &'a str,
        }

        let url = self.pipeline.url(&self.databases_link);
        let mut req = Request::new(url, Method::Post);
        req.insert_headers(&options.throughput)?;
        req.set_json(&RequestBody { id })?;

        self.pipeline
            .send(
                options.method_options.context,
                &mut req,
                self.databases_link.clone(),
            )
            .await
    }
}
