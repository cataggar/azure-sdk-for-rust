#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: azure_core::http::Url,
    credential: std::sync::Arc<dyn azure_core::credentials::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::http::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::credentials::TokenCredential>,
    endpoint: Option<azure_core::http::Url>,
    scopes: Option<Vec<String>>,
    options: azure_core::http::ClientOptions,
}
pub use azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD as DEFAULT_ENDPOINT;
impl ClientBuilder {
    #[doc = "Create a new instance of `ClientBuilder`."]
    #[must_use]
    pub fn new(credential: std::sync::Arc<dyn azure_core::credentials::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
            options: azure_core::http::ClientOptions::default(),
        }
    }
    #[doc = "Set the endpoint."]
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<azure_core::http::Url>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    #[doc = "Set the scopes."]
    #[must_use]
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    #[doc = "Convert the builder into a `Client` instance."]
    pub fn build(self) -> azure_core::Result<Client> {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = if let Some(scopes) = self.scopes {
            scopes
        } else {
            vec![endpoint.join(azure_core::credentials::DEFAULT_SCOPE_SUFFIX)?.to_string()]
        };
        Ok(Client::new(endpoint, self.credential, scopes, self.options))
    }
}
impl Client {
    pub(crate) async fn bearer_token(&self) -> azure_core::Result<azure_core::credentials::Secret> {
        let credential = self.token_credential();
        let response = credential.get_token(&self.scopes(), None).await?;
        Ok(response.token)
    }
    pub(crate) fn endpoint(&self) -> &azure_core::http::Url {
        &self.endpoint
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::credentials::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(
        &self,
        request: &mut typespec_client_core::http::request::Request,
    ) -> azure_core::Result<typespec_client_core::http::response::RawResponse> {
        let context = typespec_client_core::http::Context::default();
        self.pipeline.send(&context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: std::sync::Arc<dyn azure_core::credentials::TokenCredential>) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<azure_core::http::Url>,
        credential: std::sync::Arc<dyn azure_core::credentials::TokenCredential>,
        scopes: Vec<String>,
        options: azure_core::http::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn addons_client(&self) -> addons::Client {
        addons::Client(self.clone())
    }
    pub fn authorizations_client(&self) -> authorizations::Client {
        authorizations::Client(self.clone())
    }
    pub fn cloud_links_client(&self) -> cloud_links::Client {
        cloud_links::Client(self.clone())
    }
    pub fn clusters_client(&self) -> clusters::Client {
        clusters::Client(self.clone())
    }
    pub fn datastores_client(&self) -> datastores::Client {
        datastores::Client(self.clone())
    }
    pub fn global_reach_connections_client(&self) -> global_reach_connections::Client {
        global_reach_connections::Client(self.clone())
    }
    pub fn hcx_enterprise_sites_client(&self) -> hcx_enterprise_sites::Client {
        hcx_enterprise_sites::Client(self.clone())
    }
    pub fn hosts_client(&self) -> hosts::Client {
        hosts::Client(self.clone())
    }
    pub fn iscsi_paths_client(&self) -> iscsi_paths::Client {
        iscsi_paths::Client(self.clone())
    }
    pub fn locations_client(&self) -> locations::Client {
        locations::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn placement_policies_client(&self) -> placement_policies::Client {
        placement_policies::Client(self.clone())
    }
    pub fn private_clouds_client(&self) -> private_clouds::Client {
        private_clouds::Client(self.clone())
    }
    pub fn provisioned_networks_client(&self) -> provisioned_networks::Client {
        provisioned_networks::Client(self.clone())
    }
    pub fn pure_storage_policies_client(&self) -> pure_storage_policies::Client {
        pure_storage_policies::Client(self.clone())
    }
    pub fn script_cmdlets_client(&self) -> script_cmdlets::Client {
        script_cmdlets::Client(self.clone())
    }
    pub fn script_executions_client(&self) -> script_executions::Client {
        script_executions::Client(self.clone())
    }
    pub fn script_packages_client(&self) -> script_packages::Client {
        script_packages::Client(self.clone())
    }
    pub fn skus_client(&self) -> skus::Client {
        skus::Client(self.clone())
    }
    pub fn virtual_machines_client(&self) -> virtual_machines::Client {
        virtual_machines::Client(self.clone())
    }
    pub fn workload_networks_client(&self) -> workload_networks::Client {
        workload_networks::Client(self.clone())
    }
}
pub mod operations {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List the operations for the provider"]
        pub fn list(&self) -> list::RequestBuilder {
            list::RequestBuilder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::OperationListResult> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::OperationListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path("/providers/Microsoft.AVS/operations");
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::OperationListResult>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::OperationListResult = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<models::OperationListResult, azure_core::http::JsonFormat> =
                                raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
}
pub mod locations {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Return quota for subscription by region"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `location`: The name of the Azure region."]
        pub fn check_quota_availability(
            &self,
            subscription_id: impl Into<String>,
            location: impl Into<String>,
        ) -> check_quota_availability::RequestBuilder {
            check_quota_availability::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                location: location.into(),
            }
        }
        #[doc = "Return trial status for subscription by region"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `location`: The name of the Azure region."]
        pub fn check_trial_availability(
            &self,
            subscription_id: impl Into<String>,
            location: impl Into<String>,
        ) -> check_trial_availability::RequestBuilder {
            check_trial_availability::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                location: location.into(),
                sku: None,
            }
        }
    }
    pub mod check_quota_availability {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Quota> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::Quota = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) location: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/providers/Microsoft.AVS/locations/{}/checkQuotaAvailability",
                    &self.subscription_id, &self.location
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Post);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.insert_header(azure_core::http::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod check_trial_availability {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Trial> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::Trial = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) location: String,
            pub(crate) sku: Option<models::Sku>,
        }
        impl RequestBuilder {
            #[doc = "Optionally, check for a specific SKU"]
            pub fn sku(mut self, sku: impl Into<models::Sku>) -> Self {
                self.sku = Some(sku.into());
                self
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/providers/Microsoft.AVS/locations/{}/checkTrialAvailability",
                    &self.subscription_id, &self.location
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Post);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = if let Some(sku) = &this.sku {
                            req.insert_header("content-type", "application/json");
                            azure_core::json::to_json(sku)?
                        } else {
                            azure_openapi_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod private_clouds {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List PrivateCloud resources by subscription ID"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        pub fn list_in_subscription(&self, subscription_id: impl Into<String>) -> list_in_subscription::RequestBuilder {
            list_in_subscription::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "List PrivateCloud resources by resource group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        pub fn list(&self, subscription_id: impl Into<String>, resource_group_name: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
            }
        }
        #[doc = "Get a PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Create a PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `private_cloud`: Resource create parameters."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            private_cloud: impl Into<models::PrivateCloud>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                private_cloud: private_cloud.into(),
            }
        }
        #[doc = "Update a PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `private_cloud_update`: The resource properties to be updated."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            private_cloud_update: impl Into<models::PrivateCloudUpdate>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                private_cloud_update: private_cloud_update.into(),
            }
        }
        #[doc = "Delete a PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "List the admin credentials for the private cloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list_admin_credentials(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list_admin_credentials::RequestBuilder {
            list_admin_credentials::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Rotate the NSX-T Manager password"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn rotate_nsxt_password(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> rotate_nsxt_password::RequestBuilder {
            rotate_nsxt_password::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Rotate the vCenter password"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn rotate_vcenter_password(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> rotate_vcenter_password::RequestBuilder {
            rotate_vcenter_password::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
    }
    pub mod list_in_subscription {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PrivateCloudList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::PrivateCloudList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/providers/Microsoft.AVS/privateClouds",
                    &self.subscription_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::PrivateCloudList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::PrivateCloudList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<models::PrivateCloudList, azure_core::http::JsonFormat> =
                                raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PrivateCloudList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::PrivateCloudList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds",
                    &self.subscription_id, &self.resource_group_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::PrivateCloudList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::PrivateCloudList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<models::PrivateCloudList, azure_core::http::JsonFormat> =
                                raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PrivateCloud> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::PrivateCloud = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PrivateCloud> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::PrivateCloud = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::PrivateCloud);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::PrivateCloud;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) private_cloud: models::PrivateCloud,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.private_cloud)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PrivateCloud> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::PrivateCloud = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::PrivateCloud);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::PrivateCloud;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) private_cloud_update: models::PrivateCloudUpdate,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Patch);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.private_cloud_update)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_admin_credentials {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AdminCredentials> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::AdminCredentials = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/listAdminCredentials",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Post);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.insert_header(azure_core::http::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod rotate_nsxt_password {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/rotateNsxtPassword",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Post);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.insert_header(azure_core::http::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod rotate_vcenter_password {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/rotateVcenterPassword",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Post);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.insert_header(azure_core::http::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod skus {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "A list of SKUs."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PagedResourceSku> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::PagedResourceSku = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!("/subscriptions/{}/providers/Microsoft.AVS/skus", &self.subscription_id));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::PagedResourceSku>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::PagedResourceSku = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<models::PagedResourceSku, azure_core::http::JsonFormat> =
                                raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
}
pub mod addons {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List Addon resources by PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a Addon"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `addon_name`: Name of the addon."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            addon_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                addon_name: addon_name.into(),
            }
        }
        #[doc = "Create a Addon"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `addon_name`: Name of the addon."]
        #[doc = "* `addon`: Resource create parameters."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            addon_name: impl Into<String>,
            addon: impl Into<models::Addon>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                addon_name: addon_name.into(),
                addon: addon.into(),
            }
        }
        #[doc = "Delete a Addon"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `addon_name`: Name of the addon."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            addon_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                addon_name: addon_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::AddonList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::AddonList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/addons",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::AddonList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::AddonList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<models::AddonList, azure_core::http::JsonFormat> =
                                raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Addon> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::Addon = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) addon_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/addons/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.addon_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Addon> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::Addon = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::Addon);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::Addon;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) addon_name: String,
            pub(crate) addon: models::Addon,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/addons/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.addon_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.addon)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) addon_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/addons/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.addon_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod authorizations {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List ExpressRouteAuthorization resources by PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a ExpressRouteAuthorization"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `authorization_name`: Name of the ExpressRoute Circuit Authorization"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            authorization_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                authorization_name: authorization_name.into(),
            }
        }
        #[doc = "Create a ExpressRouteAuthorization"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `authorization_name`: Name of the ExpressRoute Circuit Authorization"]
        #[doc = "* `authorization`: Resource create parameters."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            authorization_name: impl Into<String>,
            authorization: impl Into<models::ExpressRouteAuthorization>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                authorization_name: authorization_name.into(),
                authorization: authorization.into(),
            }
        }
        #[doc = "Delete a ExpressRouteAuthorization"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `authorization_name`: Name of the ExpressRoute Circuit Authorization"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            authorization_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                authorization_name: authorization_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ExpressRouteAuthorizationList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::ExpressRouteAuthorizationList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/authorizations",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::ExpressRouteAuthorizationList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::ExpressRouteAuthorizationList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<
                                models::ExpressRouteAuthorizationList,
                                azure_core::http::JsonFormat,
                            > = raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ExpressRouteAuthorization> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::ExpressRouteAuthorization = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) authorization_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/authorizations/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.authorization_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ExpressRouteAuthorization> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::ExpressRouteAuthorization = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::ExpressRouteAuthorization);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::ExpressRouteAuthorization;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) authorization_name: String,
            pub(crate) authorization: models::ExpressRouteAuthorization,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/authorizations/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.authorization_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.authorization)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) authorization_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/authorizations/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.authorization_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod cloud_links {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List CloudLink resources by PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a CloudLink"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cloud_link_name`: Name of the cloud link."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cloud_link_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cloud_link_name: cloud_link_name.into(),
            }
        }
        #[doc = "Create a CloudLink"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cloud_link_name`: Name of the cloud link."]
        #[doc = "* `cloud_link`: Resource create parameters."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cloud_link_name: impl Into<String>,
            cloud_link: impl Into<models::CloudLink>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cloud_link_name: cloud_link_name.into(),
                cloud_link: cloud_link.into(),
            }
        }
        #[doc = "Delete a CloudLink"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cloud_link_name`: Name of the cloud link."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cloud_link_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cloud_link_name: cloud_link_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CloudLinkList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::CloudLinkList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/cloudLinks",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::CloudLinkList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::CloudLinkList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<models::CloudLinkList, azure_core::http::JsonFormat> =
                                raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CloudLink> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::CloudLink = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cloud_link_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/cloudLinks/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cloud_link_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CloudLink> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::CloudLink = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::CloudLink);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::CloudLink;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cloud_link_name: String,
            pub(crate) cloud_link: models::CloudLink,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/cloudLinks/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cloud_link_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.cloud_link)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cloud_link_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/cloudLinks/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cloud_link_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod clusters {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List Cluster resources by PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a Cluster"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
            }
        }
        #[doc = "Create a Cluster"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        #[doc = "* `cluster`: Resource create parameters."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
            cluster: impl Into<models::Cluster>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
                cluster: cluster.into(),
            }
        }
        #[doc = "Update a Cluster"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        #[doc = "* `cluster_update`: The resource properties to be updated."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
            cluster_update: impl Into<models::ClusterUpdate>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
                cluster_update: cluster_update.into(),
            }
        }
        #[doc = "Delete a Cluster"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
            }
        }
        #[doc = "List hosts by zone in a cluster"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        pub fn list_zones(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
        ) -> list_zones::RequestBuilder {
            list_zones::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ClusterList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::ClusterList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::ClusterList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::ClusterList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<models::ClusterList, azure_core::http::JsonFormat> =
                                raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Cluster> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::Cluster = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cluster_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Cluster> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::Cluster = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::Cluster);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::Cluster;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
            pub(crate) cluster: models::Cluster,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cluster_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.cluster)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Cluster> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::Cluster = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::Cluster);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::Cluster;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
            pub(crate) cluster_update: models::ClusterUpdate,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cluster_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Patch);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.cluster_update)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cluster_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_zones {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ClusterZoneList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::ClusterZoneList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}/listZones",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cluster_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Post);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.insert_header(azure_core::http::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod datastores {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List Datastore resources by Cluster"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
            }
        }
        #[doc = "Get a Datastore"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        #[doc = "* `datastore_name`: Name of the datastore"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
            datastore_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
                datastore_name: datastore_name.into(),
            }
        }
        #[doc = "Create a Datastore"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        #[doc = "* `datastore_name`: Name of the datastore"]
        #[doc = "* `datastore`: Resource create parameters."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
            datastore_name: impl Into<String>,
            datastore: impl Into<models::Datastore>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
                datastore_name: datastore_name.into(),
                datastore: datastore.into(),
            }
        }
        #[doc = "Delete a Datastore"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        #[doc = "* `datastore_name`: Name of the datastore"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
            datastore_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
                datastore_name: datastore_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DatastoreList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::DatastoreList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}/datastores",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cluster_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::DatastoreList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::DatastoreList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<models::DatastoreList, azure_core::http::JsonFormat> =
                                raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Datastore> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::Datastore = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
            pub(crate) datastore_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}/datastores/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cluster_name, &self.datastore_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Datastore> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::Datastore = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::Datastore);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::Datastore;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
            pub(crate) datastore_name: String,
            pub(crate) datastore: models::Datastore,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}/datastores/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cluster_name, &self.datastore_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.datastore)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
            pub(crate) datastore_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}/datastores/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cluster_name, &self.datastore_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod hosts {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List Host resources by Cluster"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
            }
        }
        #[doc = "Get a Host"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        #[doc = "* `host_id`: The host identifier."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
            host_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
                host_id: host_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HostListResult> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::HostListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}/hosts",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cluster_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::HostListResult>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::HostListResult = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<models::HostListResult, azure_core::http::JsonFormat> =
                                raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::Host> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::Host = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
            pub(crate) host_id: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}/hosts/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cluster_name, &self.host_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod placement_policies {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List PlacementPolicy resources by Cluster"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
            }
        }
        #[doc = "Get a PlacementPolicy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        #[doc = "* `placement_policy_name`: Name of the placement policy."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
            placement_policy_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
                placement_policy_name: placement_policy_name.into(),
            }
        }
        #[doc = "Create a PlacementPolicy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        #[doc = "* `placement_policy_name`: Name of the placement policy."]
        #[doc = "* `placement_policy`: Resource create parameters."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
            placement_policy_name: impl Into<String>,
            placement_policy: impl Into<models::PlacementPolicy>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
                placement_policy_name: placement_policy_name.into(),
                placement_policy: placement_policy.into(),
            }
        }
        #[doc = "Update a PlacementPolicy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        #[doc = "* `placement_policy_name`: Name of the placement policy."]
        #[doc = "* `placement_policy_update`: The resource properties to be updated."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
            placement_policy_name: impl Into<String>,
            placement_policy_update: impl Into<models::PlacementPolicyUpdate>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
                placement_policy_name: placement_policy_name.into(),
                placement_policy_update: placement_policy_update.into(),
            }
        }
        #[doc = "Delete a PlacementPolicy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        #[doc = "* `placement_policy_name`: Name of the placement policy."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
            placement_policy_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
                placement_policy_name: placement_policy_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PlacementPoliciesList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::PlacementPoliciesList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}/placementPolicies",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cluster_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::PlacementPoliciesList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::PlacementPoliciesList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<
                                models::PlacementPoliciesList,
                                azure_core::http::JsonFormat,
                            > = raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PlacementPolicy> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::PlacementPolicy = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
            pub(crate) placement_policy_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}/placementPolicies/{}",
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.private_cloud_name,
                    &self.cluster_name,
                    &self.placement_policy_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PlacementPolicy> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::PlacementPolicy = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::PlacementPolicy);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::PlacementPolicy;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
            pub(crate) placement_policy_name: String,
            pub(crate) placement_policy: models::PlacementPolicy,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}/placementPolicies/{}",
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.private_cloud_name,
                    &self.cluster_name,
                    &self.placement_policy_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.placement_policy)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PlacementPolicy> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::PlacementPolicy = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::PlacementPolicy);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::PlacementPolicy;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
            pub(crate) placement_policy_name: String,
            pub(crate) placement_policy_update: models::PlacementPolicyUpdate,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}/placementPolicies/{}",
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.private_cloud_name,
                    &self.cluster_name,
                    &self.placement_policy_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Patch);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.placement_policy_update)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
            pub(crate) placement_policy_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}/placementPolicies/{}",
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.private_cloud_name,
                    &self.cluster_name,
                    &self.placement_policy_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod virtual_machines {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List VirtualMachine resources by Cluster"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
            }
        }
        #[doc = "Get a VirtualMachine"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        #[doc = "* `virtual_machine_id`: ID of the virtual machine."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
            virtual_machine_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
                virtual_machine_id: virtual_machine_id.into(),
            }
        }
        #[doc = "Enable or disable DRS-driven VM movement restriction"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `cluster_name`: Name of the cluster"]
        #[doc = "* `virtual_machine_id`: ID of the virtual machine."]
        #[doc = "* `restrict_movement`: The content of the action request"]
        pub fn restrict_movement(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            cluster_name: impl Into<String>,
            virtual_machine_id: impl Into<String>,
            restrict_movement: impl Into<models::VirtualMachineRestrictMovement>,
        ) -> restrict_movement::RequestBuilder {
            restrict_movement::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                cluster_name: cluster_name.into(),
                virtual_machine_id: virtual_machine_id.into(),
                restrict_movement: restrict_movement.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VirtualMachinesList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::VirtualMachinesList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}/virtualMachines",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.cluster_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::VirtualMachinesList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::VirtualMachinesList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<models::VirtualMachinesList, azure_core::http::JsonFormat> =
                                raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::VirtualMachine> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::VirtualMachine = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
            pub(crate) virtual_machine_id: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}/virtualMachines/{}",
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.private_cloud_name,
                    &self.cluster_name,
                    &self.virtual_machine_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod restrict_movement {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) cluster_name: String,
            pub(crate) virtual_machine_id: String,
            pub(crate) restrict_movement: models::VirtualMachineRestrictMovement,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url . set_path (& format ! ("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/clusters/{}/virtualMachines/{}/restrictMovement" , & self . subscription_id , & self . resource_group_name , & self . private_cloud_name , & self . cluster_name , & self . virtual_machine_id)) ;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Post);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.restrict_movement)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod global_reach_connections {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List GlobalReachConnection resources by PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a GlobalReachConnection"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `global_reach_connection_name`: Name of the global reach connection"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            global_reach_connection_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                global_reach_connection_name: global_reach_connection_name.into(),
            }
        }
        #[doc = "Create a GlobalReachConnection"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `global_reach_connection_name`: Name of the global reach connection"]
        #[doc = "* `global_reach_connection`: Resource create parameters."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            global_reach_connection_name: impl Into<String>,
            global_reach_connection: impl Into<models::GlobalReachConnection>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                global_reach_connection_name: global_reach_connection_name.into(),
                global_reach_connection: global_reach_connection.into(),
            }
        }
        #[doc = "Delete a GlobalReachConnection"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `global_reach_connection_name`: Name of the global reach connection"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            global_reach_connection_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                global_reach_connection_name: global_reach_connection_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::GlobalReachConnectionList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::GlobalReachConnectionList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/globalReachConnections",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::GlobalReachConnectionList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::GlobalReachConnectionList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<
                                models::GlobalReachConnectionList,
                                azure_core::http::JsonFormat,
                            > = raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::GlobalReachConnection> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::GlobalReachConnection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) global_reach_connection_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/globalReachConnections/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.global_reach_connection_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::GlobalReachConnection> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::GlobalReachConnection = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::GlobalReachConnection);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::GlobalReachConnection;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) global_reach_connection_name: String,
            pub(crate) global_reach_connection: models::GlobalReachConnection,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/globalReachConnections/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.global_reach_connection_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.global_reach_connection)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) global_reach_connection_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/globalReachConnections/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.global_reach_connection_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod hcx_enterprise_sites {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List HcxEnterpriseSite resources by PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a HcxEnterpriseSite"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `hcx_enterprise_site_name`: Name of the HCX Enterprise Site"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            hcx_enterprise_site_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                hcx_enterprise_site_name: hcx_enterprise_site_name.into(),
            }
        }
        #[doc = "Create a HcxEnterpriseSite"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `hcx_enterprise_site_name`: Name of the HCX Enterprise Site"]
        #[doc = "* `hcx_enterprise_site`: Resource create parameters."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            hcx_enterprise_site_name: impl Into<String>,
            hcx_enterprise_site: impl Into<models::HcxEnterpriseSite>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                hcx_enterprise_site_name: hcx_enterprise_site_name.into(),
                hcx_enterprise_site: hcx_enterprise_site.into(),
            }
        }
        #[doc = "Delete a HcxEnterpriseSite"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `hcx_enterprise_site_name`: Name of the HCX Enterprise Site"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            hcx_enterprise_site_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                hcx_enterprise_site_name: hcx_enterprise_site_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HcxEnterpriseSiteList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::HcxEnterpriseSiteList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/hcxEnterpriseSites",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::HcxEnterpriseSiteList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::HcxEnterpriseSiteList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<
                                models::HcxEnterpriseSiteList,
                                azure_core::http::JsonFormat,
                            > = raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HcxEnterpriseSite> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::HcxEnterpriseSite = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) hcx_enterprise_site_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/hcxEnterpriseSites/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.hcx_enterprise_site_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::HcxEnterpriseSite> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::HcxEnterpriseSite = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) hcx_enterprise_site_name: String,
            pub(crate) hcx_enterprise_site: models::HcxEnterpriseSite,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/hcxEnterpriseSites/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.hcx_enterprise_site_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.hcx_enterprise_site)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) hcx_enterprise_site_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/hcxEnterpriseSites/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.hcx_enterprise_site_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod iscsi_paths {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List IscsiPath resources by PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list_by_private_cloud(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list_by_private_cloud::RequestBuilder {
            list_by_private_cloud::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a IscsiPath"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Create a IscsiPath"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `resource`: Resource create parameters."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            resource: impl Into<models::IscsiPath>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                resource: resource.into(),
            }
        }
        #[doc = "Delete a IscsiPath"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
    }
    pub mod list_by_private_cloud {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IscsiPathListResult> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::IscsiPathListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/iscsiPaths",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::IscsiPathListResult>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::IscsiPathListResult = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<models::IscsiPathListResult, azure_core::http::JsonFormat> =
                                raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IscsiPath> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::IscsiPath = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/iscsiPaths/default",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::IscsiPath> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::IscsiPath = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::IscsiPath);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::IscsiPath;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) resource: models::IscsiPath,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/iscsiPaths/default",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.resource)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/iscsiPaths/default",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod provisioned_networks {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List ProvisionedNetwork resources by PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a ProvisionedNetwork"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `provisioned_network_name`: Name of the cloud link."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            provisioned_network_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                provisioned_network_name: provisioned_network_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProvisionedNetworkListResult> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::ProvisionedNetworkListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/provisionedNetworks",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::ProvisionedNetworkListResult>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::ProvisionedNetworkListResult = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<
                                models::ProvisionedNetworkListResult,
                                azure_core::http::JsonFormat,
                            > = raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProvisionedNetwork> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::ProvisionedNetwork = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) provisioned_network_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/provisionedNetworks/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.provisioned_network_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod pure_storage_policies {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List PureStoragePolicy resources by PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a PureStoragePolicy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `storage_policy_name`: Name of the storage policy."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            storage_policy_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                storage_policy_name: storage_policy_name.into(),
            }
        }
        #[doc = "Create a PureStoragePolicy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `storage_policy_name`: Name of the storage policy."]
        #[doc = "* `resource`: Resource create parameters."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            storage_policy_name: impl Into<String>,
            resource: impl Into<models::PureStoragePolicy>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                storage_policy_name: storage_policy_name.into(),
                resource: resource.into(),
            }
        }
        #[doc = "Delete a PureStoragePolicy"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `storage_policy_name`: Name of the storage policy."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            storage_policy_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                storage_policy_name: storage_policy_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PureStoragePolicyListResult> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::PureStoragePolicyListResult = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/pureStoragePolicies",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::PureStoragePolicyListResult>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::PureStoragePolicyListResult = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<
                                models::PureStoragePolicyListResult,
                                azure_core::http::JsonFormat,
                            > = raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PureStoragePolicy> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::PureStoragePolicy = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) storage_policy_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/pureStoragePolicies/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.storage_policy_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PureStoragePolicy> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::PureStoragePolicy = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "A link to the status monitor"]
            pub fn azure_async_operation(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::http::headers::HeaderName::from_static("azure-asyncoperation"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::PureStoragePolicy);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::PureStoragePolicy;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) storage_policy_name: String,
            pub(crate) resource: models::PureStoragePolicy,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/pureStoragePolicies/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.storage_policy_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.resource)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) storage_policy_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/pureStoragePolicies/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.storage_policy_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod script_executions {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List ScriptExecution resources by PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a ScriptExecution"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `script_execution_name`: Name of the script cmdlet."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            script_execution_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                script_execution_name: script_execution_name.into(),
            }
        }
        #[doc = "Create a ScriptExecution"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `script_execution_name`: Name of the script cmdlet."]
        #[doc = "* `script_execution`: Resource create parameters."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            script_execution_name: impl Into<String>,
            script_execution: impl Into<models::ScriptExecution>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                script_execution_name: script_execution_name.into(),
                script_execution: script_execution.into(),
            }
        }
        #[doc = "Delete a ScriptExecution"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `script_execution_name`: Name of the script cmdlet."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            script_execution_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                script_execution_name: script_execution_name.into(),
            }
        }
        #[doc = "Return the logs for a script execution resource"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `script_execution_name`: Name of the script cmdlet."]
        pub fn get_execution_logs(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            script_execution_name: impl Into<String>,
        ) -> get_execution_logs::RequestBuilder {
            get_execution_logs::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                script_execution_name: script_execution_name.into(),
                script_output_stream_type: Vec::new(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ScriptExecutionsList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::ScriptExecutionsList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/scriptExecutions",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::ScriptExecutionsList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::ScriptExecutionsList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<models::ScriptExecutionsList, azure_core::http::JsonFormat> =
                                raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ScriptExecution> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::ScriptExecution = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) script_execution_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/scriptExecutions/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.script_execution_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ScriptExecution> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::ScriptExecution = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::ScriptExecution);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::ScriptExecution;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) script_execution_name: String,
            pub(crate) script_execution: models::ScriptExecution,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/scriptExecutions/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.script_execution_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.script_execution)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) script_execution_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/scriptExecutions/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.script_execution_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_execution_logs {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ScriptExecution> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::ScriptExecution = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) script_execution_name: String,
            pub(crate) script_output_stream_type: Vec<models::ScriptOutputStreamType>,
        }
        impl RequestBuilder {
            #[doc = "Name of the desired output stream to return. If not provided, will return all. An empty array will return nothing."]
            pub fn script_output_stream_type(mut self, script_output_stream_type: Vec<models::ScriptOutputStreamType>) -> Self {
                self.script_output_stream_type = script_output_stream_type;
                self
            }
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/scriptExecutions/{}/getExecutionLogs",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.script_execution_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Post);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.script_output_stream_type)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod script_packages {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List ScriptPackage resources by PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a ScriptPackage"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `script_package_name`: Name of the script package."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            script_package_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                script_package_name: script_package_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ScriptPackagesList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::ScriptPackagesList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/scriptPackages",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::ScriptPackagesList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::ScriptPackagesList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<models::ScriptPackagesList, azure_core::http::JsonFormat> =
                                raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ScriptPackage> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::ScriptPackage = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) script_package_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/scriptPackages/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.script_package_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod script_cmdlets {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List ScriptCmdlet resources by ScriptPackage"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `script_package_name`: Name of the script package."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            script_package_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                script_package_name: script_package_name.into(),
            }
        }
        #[doc = "Get a ScriptCmdlet"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `script_package_name`: Name of the script package."]
        #[doc = "* `script_cmdlet_name`: Name of the script cmdlet."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            script_package_name: impl Into<String>,
            script_cmdlet_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                script_package_name: script_package_name.into(),
                script_cmdlet_name: script_cmdlet_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ScriptCmdletsList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::ScriptCmdletsList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) script_package_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/scriptPackages/{}/scriptCmdlets",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.script_package_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::ScriptCmdletsList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::ScriptCmdletsList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<models::ScriptCmdletsList, azure_core::http::JsonFormat> =
                                raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ScriptCmdlet> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::ScriptCmdlet = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) script_package_name: String,
            pub(crate) script_cmdlet_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/scriptPackages/{}/scriptCmdlets/{}",
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.private_cloud_name,
                    &self.script_package_name,
                    &self.script_cmdlet_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod workload_networks {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List WorkloadNetwork resources by PrivateCloud"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a WorkloadNetwork"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "List WorkloadNetworkDhcp resources by WorkloadNetwork"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list_dhcp(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list_dhcp::RequestBuilder {
            list_dhcp::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a WorkloadNetworkDhcp"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `dhcp_id`: The ID of the DHCP configuration"]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn get_dhcp(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            dhcp_id: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> get_dhcp::RequestBuilder {
            get_dhcp::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                dhcp_id: dhcp_id.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Create a WorkloadNetworkDhcp"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `dhcp_id`: The ID of the DHCP configuration"]
        #[doc = "* `workload_network_dhcp`: Resource create parameters."]
        pub fn create_dhcp(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            dhcp_id: impl Into<String>,
            workload_network_dhcp: impl Into<models::WorkloadNetworkDhcp>,
        ) -> create_dhcp::RequestBuilder {
            create_dhcp::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                dhcp_id: dhcp_id.into(),
                workload_network_dhcp: workload_network_dhcp.into(),
            }
        }
        #[doc = "Update a WorkloadNetworkDhcp"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `dhcp_id`: The ID of the DHCP configuration"]
        #[doc = "* `workload_network_dhcp`: The resource properties to be updated."]
        pub fn update_dhcp(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            dhcp_id: impl Into<String>,
            workload_network_dhcp: impl Into<models::WorkloadNetworkDhcp>,
        ) -> update_dhcp::RequestBuilder {
            update_dhcp::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                dhcp_id: dhcp_id.into(),
                workload_network_dhcp: workload_network_dhcp.into(),
            }
        }
        #[doc = "Delete a WorkloadNetworkDhcp"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `dhcp_id`: The ID of the DHCP configuration"]
        pub fn delete_dhcp(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            dhcp_id: impl Into<String>,
        ) -> delete_dhcp::RequestBuilder {
            delete_dhcp::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                dhcp_id: dhcp_id.into(),
            }
        }
        #[doc = "List WorkloadNetworkDnsService resources by WorkloadNetwork"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list_dns_services(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list_dns_services::RequestBuilder {
            list_dns_services::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a WorkloadNetworkDnsService"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `dns_service_id`: ID of the DNS service."]
        pub fn get_dns_service(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            dns_service_id: impl Into<String>,
        ) -> get_dns_service::RequestBuilder {
            get_dns_service::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                dns_service_id: dns_service_id.into(),
            }
        }
        #[doc = "Create a WorkloadNetworkDnsService"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `dns_service_id`: ID of the DNS service."]
        #[doc = "* `workload_network_dns_service`: Resource create parameters."]
        pub fn create_dns_service(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            dns_service_id: impl Into<String>,
            workload_network_dns_service: impl Into<models::WorkloadNetworkDnsService>,
        ) -> create_dns_service::RequestBuilder {
            create_dns_service::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                dns_service_id: dns_service_id.into(),
                workload_network_dns_service: workload_network_dns_service.into(),
            }
        }
        #[doc = "Update a WorkloadNetworkDnsService"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `dns_service_id`: ID of the DNS service."]
        #[doc = "* `workload_network_dns_service`: The resource properties to be updated."]
        pub fn update_dns_service(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            dns_service_id: impl Into<String>,
            workload_network_dns_service: impl Into<models::WorkloadNetworkDnsService>,
        ) -> update_dns_service::RequestBuilder {
            update_dns_service::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                dns_service_id: dns_service_id.into(),
                workload_network_dns_service: workload_network_dns_service.into(),
            }
        }
        #[doc = "Delete a WorkloadNetworkDnsService"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `dns_service_id`: ID of the DNS service."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn delete_dns_service(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            dns_service_id: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> delete_dns_service::RequestBuilder {
            delete_dns_service::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                dns_service_id: dns_service_id.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "List WorkloadNetworkDnsZone resources by WorkloadNetwork"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list_dns_zones(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list_dns_zones::RequestBuilder {
            list_dns_zones::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a WorkloadNetworkDnsZone"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `dns_zone_id`: ID of the DNS zone."]
        pub fn get_dns_zone(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            dns_zone_id: impl Into<String>,
        ) -> get_dns_zone::RequestBuilder {
            get_dns_zone::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                dns_zone_id: dns_zone_id.into(),
            }
        }
        #[doc = "Create a WorkloadNetworkDnsZone"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `dns_zone_id`: ID of the DNS zone."]
        #[doc = "* `workload_network_dns_zone`: Resource create parameters."]
        pub fn create_dns_zone(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            dns_zone_id: impl Into<String>,
            workload_network_dns_zone: impl Into<models::WorkloadNetworkDnsZone>,
        ) -> create_dns_zone::RequestBuilder {
            create_dns_zone::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                dns_zone_id: dns_zone_id.into(),
                workload_network_dns_zone: workload_network_dns_zone.into(),
            }
        }
        #[doc = "Update a WorkloadNetworkDnsZone"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `dns_zone_id`: ID of the DNS zone."]
        #[doc = "* `workload_network_dns_zone`: The resource properties to be updated."]
        pub fn update_dns_zone(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            dns_zone_id: impl Into<String>,
            workload_network_dns_zone: impl Into<models::WorkloadNetworkDnsZone>,
        ) -> update_dns_zone::RequestBuilder {
            update_dns_zone::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                dns_zone_id: dns_zone_id.into(),
                workload_network_dns_zone: workload_network_dns_zone.into(),
            }
        }
        #[doc = "Delete a WorkloadNetworkDnsZone"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `dns_zone_id`: ID of the DNS zone."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn delete_dns_zone(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            dns_zone_id: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> delete_dns_zone::RequestBuilder {
            delete_dns_zone::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                dns_zone_id: dns_zone_id.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "List WorkloadNetworkGateway resources by WorkloadNetwork"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list_gateways(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list_gateways::RequestBuilder {
            list_gateways::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a WorkloadNetworkGateway"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `gateway_id`: The ID of the NSX Gateway"]
        pub fn get_gateway(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            gateway_id: impl Into<String>,
        ) -> get_gateway::RequestBuilder {
            get_gateway::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                gateway_id: gateway_id.into(),
            }
        }
        #[doc = "List WorkloadNetworkPortMirroring resources by WorkloadNetwork"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list_port_mirroring(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list_port_mirroring::RequestBuilder {
            list_port_mirroring::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a WorkloadNetworkPortMirroring"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `port_mirroring_id`: ID of the NSX port mirroring profile."]
        pub fn get_port_mirroring(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            port_mirroring_id: impl Into<String>,
        ) -> get_port_mirroring::RequestBuilder {
            get_port_mirroring::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                port_mirroring_id: port_mirroring_id.into(),
            }
        }
        #[doc = "Create a WorkloadNetworkPortMirroring"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `port_mirroring_id`: ID of the NSX port mirroring profile."]
        #[doc = "* `workload_network_port_mirroring`: Resource create parameters."]
        pub fn create_port_mirroring(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            port_mirroring_id: impl Into<String>,
            workload_network_port_mirroring: impl Into<models::WorkloadNetworkPortMirroring>,
        ) -> create_port_mirroring::RequestBuilder {
            create_port_mirroring::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                port_mirroring_id: port_mirroring_id.into(),
                workload_network_port_mirroring: workload_network_port_mirroring.into(),
            }
        }
        #[doc = "Update a WorkloadNetworkPortMirroring"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `port_mirroring_id`: ID of the NSX port mirroring profile."]
        #[doc = "* `workload_network_port_mirroring`: The resource properties to be updated."]
        pub fn update_port_mirroring(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            port_mirroring_id: impl Into<String>,
            workload_network_port_mirroring: impl Into<models::WorkloadNetworkPortMirroring>,
        ) -> update_port_mirroring::RequestBuilder {
            update_port_mirroring::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                port_mirroring_id: port_mirroring_id.into(),
                workload_network_port_mirroring: workload_network_port_mirroring.into(),
            }
        }
        #[doc = "Delete a WorkloadNetworkPortMirroring"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `port_mirroring_id`: ID of the NSX port mirroring profile."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn delete_port_mirroring(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            port_mirroring_id: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> delete_port_mirroring::RequestBuilder {
            delete_port_mirroring::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                port_mirroring_id: port_mirroring_id.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "List WorkloadNetworkPublicIP resources by WorkloadNetwork"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list_public_i_ps(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list_public_i_ps::RequestBuilder {
            list_public_i_ps::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a WorkloadNetworkPublicIP"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `public_ip_id`: ID of the DNS zone."]
        pub fn get_public_ip(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            public_ip_id: impl Into<String>,
        ) -> get_public_ip::RequestBuilder {
            get_public_ip::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                public_ip_id: public_ip_id.into(),
            }
        }
        #[doc = "Create a WorkloadNetworkPublicIP"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `public_ip_id`: ID of the DNS zone."]
        #[doc = "* `workload_network_public_ip`: Resource create parameters."]
        pub fn create_public_ip(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            public_ip_id: impl Into<String>,
            workload_network_public_ip: impl Into<models::WorkloadNetworkPublicIp>,
        ) -> create_public_ip::RequestBuilder {
            create_public_ip::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                public_ip_id: public_ip_id.into(),
                workload_network_public_ip: workload_network_public_ip.into(),
            }
        }
        #[doc = "Delete a WorkloadNetworkPublicIP"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `public_ip_id`: ID of the DNS zone."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn delete_public_ip(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            public_ip_id: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> delete_public_ip::RequestBuilder {
            delete_public_ip::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                public_ip_id: public_ip_id.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "List WorkloadNetworkSegment resources by WorkloadNetwork"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list_segments(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list_segments::RequestBuilder {
            list_segments::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a WorkloadNetworkSegment"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `segment_id`: The ID of the NSX Segment"]
        pub fn get_segment(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            segment_id: impl Into<String>,
        ) -> get_segment::RequestBuilder {
            get_segment::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                segment_id: segment_id.into(),
            }
        }
        #[doc = "Create a WorkloadNetworkSegment"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `segment_id`: The ID of the NSX Segment"]
        #[doc = "* `workload_network_segment`: Resource create parameters."]
        pub fn create_segments(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            segment_id: impl Into<String>,
            workload_network_segment: impl Into<models::WorkloadNetworkSegment>,
        ) -> create_segments::RequestBuilder {
            create_segments::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                segment_id: segment_id.into(),
                workload_network_segment: workload_network_segment.into(),
            }
        }
        #[doc = "Update a WorkloadNetworkSegment"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `segment_id`: The ID of the NSX Segment"]
        #[doc = "* `workload_network_segment`: The resource properties to be updated."]
        pub fn update_segments(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            segment_id: impl Into<String>,
            workload_network_segment: impl Into<models::WorkloadNetworkSegment>,
        ) -> update_segments::RequestBuilder {
            update_segments::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                segment_id: segment_id.into(),
                workload_network_segment: workload_network_segment.into(),
            }
        }
        #[doc = "Delete a WorkloadNetworkSegment"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `segment_id`: The ID of the NSX Segment"]
        pub fn delete_segment(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            segment_id: impl Into<String>,
        ) -> delete_segment::RequestBuilder {
            delete_segment::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                segment_id: segment_id.into(),
            }
        }
        #[doc = "List WorkloadNetworkVirtualMachine resources by WorkloadNetwork"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list_virtual_machines(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list_virtual_machines::RequestBuilder {
            list_virtual_machines::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a WorkloadNetworkVirtualMachine"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `virtual_machine_id`: ID of the virtual machine."]
        pub fn get_virtual_machine(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            virtual_machine_id: impl Into<String>,
        ) -> get_virtual_machine::RequestBuilder {
            get_virtual_machine::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                virtual_machine_id: virtual_machine_id.into(),
            }
        }
        #[doc = "List WorkloadNetworkVMGroup resources by WorkloadNetwork"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn list_vm_groups(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> list_vm_groups::RequestBuilder {
            list_vm_groups::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
        #[doc = "Get a WorkloadNetworkVMGroup"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `vm_group_id`: ID of the VM group."]
        pub fn get_vm_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            vm_group_id: impl Into<String>,
        ) -> get_vm_group::RequestBuilder {
            get_vm_group::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                vm_group_id: vm_group_id.into(),
            }
        }
        #[doc = "Create a WorkloadNetworkVMGroup"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `vm_group_id`: ID of the VM group."]
        #[doc = "* `workload_network_vm_group`: Resource create parameters."]
        pub fn create_vm_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            vm_group_id: impl Into<String>,
            workload_network_vm_group: impl Into<models::WorkloadNetworkVmGroup>,
        ) -> create_vm_group::RequestBuilder {
            create_vm_group::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                vm_group_id: vm_group_id.into(),
                workload_network_vm_group: workload_network_vm_group.into(),
            }
        }
        #[doc = "Update a WorkloadNetworkVMGroup"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        #[doc = "* `vm_group_id`: ID of the VM group."]
        #[doc = "* `workload_network_vm_group`: The resource properties to be updated."]
        pub fn update_vm_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            private_cloud_name: impl Into<String>,
            vm_group_id: impl Into<String>,
            workload_network_vm_group: impl Into<models::WorkloadNetworkVmGroup>,
        ) -> update_vm_group::RequestBuilder {
            update_vm_group::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                private_cloud_name: private_cloud_name.into(),
                vm_group_id: vm_group_id.into(),
                workload_network_vm_group: workload_network_vm_group.into(),
            }
        }
        #[doc = "Delete a WorkloadNetworkVMGroup"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription. The value must be an UUID."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `vm_group_id`: ID of the VM group."]
        #[doc = "* `private_cloud_name`: Name of the private cloud"]
        pub fn delete_vm_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            vm_group_id: impl Into<String>,
            private_cloud_name: impl Into<String>,
        ) -> delete_vm_group::RequestBuilder {
            delete_vm_group::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                vm_group_id: vm_group_id.into(),
                private_cloud_name: private_cloud_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::WorkloadNetworkList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::WorkloadNetworkList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<models::WorkloadNetworkList, azure_core::http::JsonFormat> =
                                raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetwork> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetwork = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_dhcp {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkDhcpList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkDhcpList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url . set_path (& format ! ("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/dhcpConfigurations" , & self . subscription_id , & self . resource_group_name , & self . private_cloud_name)) ;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::WorkloadNetworkDhcpList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::WorkloadNetworkDhcpList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<
                                models::WorkloadNetworkDhcpList,
                                azure_core::http::JsonFormat,
                            > = raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get_dhcp {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkDhcp> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkDhcp = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) dhcp_id: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url . set_path (& format ! ("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/dhcpConfigurations/{}" , & self . subscription_id , & self . resource_group_name , & self . private_cloud_name , & self . dhcp_id)) ;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_dhcp {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkDhcp> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkDhcp = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::WorkloadNetworkDhcp);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::WorkloadNetworkDhcp;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) dhcp_id: String,
            pub(crate) workload_network_dhcp: models::WorkloadNetworkDhcp,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url . set_path (& format ! ("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/dhcpConfigurations/{}" , & self . subscription_id , & self . resource_group_name , & self . private_cloud_name , & self . dhcp_id)) ;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.workload_network_dhcp)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod update_dhcp {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkDhcp> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkDhcp = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::WorkloadNetworkDhcp);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::WorkloadNetworkDhcp;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) dhcp_id: String,
            pub(crate) workload_network_dhcp: models::WorkloadNetworkDhcp,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url . set_path (& format ! ("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/dhcpConfigurations/{}" , & self . subscription_id , & self . resource_group_name , & self . private_cloud_name , & self . dhcp_id)) ;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Patch);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.workload_network_dhcp)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete_dhcp {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) dhcp_id: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url . set_path (& format ! ("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/dhcpConfigurations/{}" , & self . subscription_id , & self . resource_group_name , & self . private_cloud_name , & self . dhcp_id)) ;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_dns_services {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkDnsServicesList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkDnsServicesList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/dnsServices",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::WorkloadNetworkDnsServicesList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::WorkloadNetworkDnsServicesList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<
                                models::WorkloadNetworkDnsServicesList,
                                azure_core::http::JsonFormat,
                            > = raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get_dns_service {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkDnsService> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkDnsService = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) dns_service_id: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/dnsServices/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.dns_service_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_dns_service {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkDnsService> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkDnsService = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::WorkloadNetworkDnsService);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::WorkloadNetworkDnsService;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) dns_service_id: String,
            pub(crate) workload_network_dns_service: models::WorkloadNetworkDnsService,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/dnsServices/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.dns_service_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.workload_network_dns_service)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod update_dns_service {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkDnsService> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkDnsService = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::WorkloadNetworkDnsService);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::WorkloadNetworkDnsService;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) dns_service_id: String,
            pub(crate) workload_network_dns_service: models::WorkloadNetworkDnsService,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/dnsServices/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.dns_service_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Patch);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.workload_network_dns_service)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete_dns_service {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) dns_service_id: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/dnsServices/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.dns_service_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_dns_zones {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkDnsZonesList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkDnsZonesList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/dnsZones",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::WorkloadNetworkDnsZonesList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::WorkloadNetworkDnsZonesList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<
                                models::WorkloadNetworkDnsZonesList,
                                azure_core::http::JsonFormat,
                            > = raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get_dns_zone {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkDnsZone> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkDnsZone = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) dns_zone_id: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/dnsZones/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.dns_zone_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_dns_zone {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkDnsZone> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkDnsZone = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::WorkloadNetworkDnsZone);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::WorkloadNetworkDnsZone;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) dns_zone_id: String,
            pub(crate) workload_network_dns_zone: models::WorkloadNetworkDnsZone,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/dnsZones/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.dns_zone_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.workload_network_dns_zone)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod update_dns_zone {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkDnsZone> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkDnsZone = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::WorkloadNetworkDnsZone);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::WorkloadNetworkDnsZone;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) dns_zone_id: String,
            pub(crate) workload_network_dns_zone: models::WorkloadNetworkDnsZone,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/dnsZones/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.dns_zone_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Patch);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.workload_network_dns_zone)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete_dns_zone {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) dns_zone_id: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/dnsZones/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.dns_zone_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_gateways {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkGatewayList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkGatewayList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/gateways",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::WorkloadNetworkGatewayList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::WorkloadNetworkGatewayList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<
                                models::WorkloadNetworkGatewayList,
                                azure_core::http::JsonFormat,
                            > = raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get_gateway {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkGateway> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkGateway = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) gateway_id: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/gateways/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.gateway_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_port_mirroring {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkPortMirroringList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkPortMirroringList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url . set_path (& format ! ("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/portMirroringProfiles" , & self . subscription_id , & self . resource_group_name , & self . private_cloud_name)) ;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::WorkloadNetworkPortMirroringList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::WorkloadNetworkPortMirroringList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<
                                models::WorkloadNetworkPortMirroringList,
                                azure_core::http::JsonFormat,
                            > = raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get_port_mirroring {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkPortMirroring> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkPortMirroring = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) port_mirroring_id: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url . set_path (& format ! ("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/portMirroringProfiles/{}" , & self . subscription_id , & self . resource_group_name , & self . private_cloud_name , & self . port_mirroring_id)) ;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_port_mirroring {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkPortMirroring> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkPortMirroring = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::WorkloadNetworkPortMirroring);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::WorkloadNetworkPortMirroring;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) port_mirroring_id: String,
            pub(crate) workload_network_port_mirroring: models::WorkloadNetworkPortMirroring,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url . set_path (& format ! ("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/portMirroringProfiles/{}" , & self . subscription_id , & self . resource_group_name , & self . private_cloud_name , & self . port_mirroring_id)) ;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.workload_network_port_mirroring)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod update_port_mirroring {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkPortMirroring> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkPortMirroring = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::WorkloadNetworkPortMirroring);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::WorkloadNetworkPortMirroring;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) port_mirroring_id: String,
            pub(crate) workload_network_port_mirroring: models::WorkloadNetworkPortMirroring,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url . set_path (& format ! ("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/portMirroringProfiles/{}" , & self . subscription_id , & self . resource_group_name , & self . private_cloud_name , & self . port_mirroring_id)) ;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Patch);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.workload_network_port_mirroring)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete_port_mirroring {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) port_mirroring_id: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url . set_path (& format ! ("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/portMirroringProfiles/{}" , & self . subscription_id , & self . resource_group_name , & self . private_cloud_name , & self . port_mirroring_id)) ;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_public_i_ps {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkPublicIPsList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkPublicIPsList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/publicIPs",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::WorkloadNetworkPublicIPsList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::WorkloadNetworkPublicIPsList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<
                                models::WorkloadNetworkPublicIPsList,
                                azure_core::http::JsonFormat,
                            > = raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get_public_ip {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkPublicIp> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkPublicIp = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) public_ip_id: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/publicIPs/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.public_ip_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_public_ip {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkPublicIp> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkPublicIp = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::WorkloadNetworkPublicIp);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::WorkloadNetworkPublicIp;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) public_ip_id: String,
            pub(crate) workload_network_public_ip: models::WorkloadNetworkPublicIp,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/publicIPs/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.public_ip_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.workload_network_public_ip)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete_public_ip {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) public_ip_id: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/publicIPs/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.public_ip_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_segments {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkSegmentsList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkSegmentsList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/segments",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::WorkloadNetworkSegmentsList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::WorkloadNetworkSegmentsList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<
                                models::WorkloadNetworkSegmentsList,
                                azure_core::http::JsonFormat,
                            > = raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get_segment {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkSegment> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkSegment = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) segment_id: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/segments/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.segment_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_segments {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkSegment> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkSegment = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::WorkloadNetworkSegment);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::WorkloadNetworkSegment;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) segment_id: String,
            pub(crate) workload_network_segment: models::WorkloadNetworkSegment,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/segments/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.segment_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.workload_network_segment)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod update_segments {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkSegment> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkSegment = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::WorkloadNetworkSegment);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::WorkloadNetworkSegment;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) segment_id: String,
            pub(crate) workload_network_segment: models::WorkloadNetworkSegment,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/segments/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.segment_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Patch);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.workload_network_segment)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete_segment {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) segment_id: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/segments/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.segment_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_virtual_machines {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkVirtualMachinesList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkVirtualMachinesList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/virtualMachines",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::WorkloadNetworkVirtualMachinesList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::WorkloadNetworkVirtualMachinesList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<
                                models::WorkloadNetworkVirtualMachinesList,
                                azure_core::http::JsonFormat,
                            > = raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get_virtual_machine {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkVirtualMachine> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkVirtualMachine = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) virtual_machine_id: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url . set_path (& format ! ("/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/virtualMachines/{}" , & self . subscription_id , & self . resource_group_name , & self . private_cloud_name , & self . virtual_machine_id)) ;
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_vm_groups {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkVmGroupsList> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkVmGroupsList = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/vmGroups",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Pager over pages"]
            pub fn pager(self) -> azure_core::Result<azure_core::http::pager::Pager<models::WorkloadNetworkVmGroupsList>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::pager::Pager::from_callback(
                    move |state: azure_core::http::pager::PagerState<String>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            let rsp = match state {
                                azure_core::http::pager::PagerState::Initial => initial.clone().send().await?.into_raw_response(),
                                azure_core::http::pager::PagerState::More(next_url) => {
                                    let mut url = client.endpoint().clone();
                                    url.set_path("");
                                    let url = url.join(next_url.as_ref())?;
                                    let mut req = typespec_client_core::http::request::Request::new(url, azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    let has_api_version_already = req
                                        .url_mut()
                                        .query_pairs()
                                        .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                                    if !has_api_version_already {
                                        req.url_mut()
                                            .query_pairs_mut()
                                            .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                                    }
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    client.send(&mut req).await?
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let bytes = body.collect().await?;
                            let page: models::WorkloadNetworkVmGroupsList = serde_json::from_slice(&bytes)?;
                            let bytes_for_json = bytes.clone();
                            let raw = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes);
                            let response: azure_core::http::response::Response<
                                models::WorkloadNetworkVmGroupsList,
                                azure_core::http::JsonFormat,
                            > = raw.into();
                            let continuation = serde_json::from_slice::<serde_json::Value>(&bytes_for_json)
                                .ok()
                                .and_then(|v| v.get("nextLink").and_then(|x| x.as_str()).map(|s| s.to_string()))
                                .filter(|s| !s.is_empty());
                            Ok(match continuation {
                                Some(continuation) => azure_core::http::pager::PagerResult::More { response, continuation },
                                None => azure_core::http::pager::PagerResult::Done { response },
                            })
                        }
                    },
                ))
            }
        }
    }
    pub mod get_vm_group {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkVmGroup> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkVmGroup = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) vm_group_id: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/vmGroups/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.vm_group_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_vm_group {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkVmGroup> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkVmGroup = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::WorkloadNetworkVmGroup);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::WorkloadNetworkVmGroup;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) vm_group_id: String,
            pub(crate) workload_network_vm_group: models::WorkloadNetworkVmGroup,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/vmGroups/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.vm_group_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.workload_network_vm_group)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod update_vm_group {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::WorkloadNetworkVmGroup> {
                let (_, _, body) = self.0.deconstruct();
                let bytes = body.collect().await?;
                let body: models::WorkloadNetworkVmGroup = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
        #[serde(transparent)]
        pub struct Operation(pub models::WorkloadNetworkVmGroup);
        impl azure_core::http::poller::StatusMonitor for Operation {
            type Output = models::WorkloadNetworkVmGroup;
            fn status(&self) -> azure_core::http::poller::PollerStatus {
                fn map_status(s: &str) -> azure_core::http::poller::PollerStatus {
                    match s.to_ascii_lowercase().as_str() {
                        "succeeded" => azure_core::http::poller::PollerStatus::Succeeded,
                        "failed" => azure_core::http::poller::PollerStatus::Failed,
                        "canceled" | "cancelled" => azure_core::http::poller::PollerStatus::Canceled,
                        _ => azure_core::http::poller::PollerStatus::InProgress,
                    }
                }
                fn find_status(value: &serde_json::Value) -> Option<azure_core::http::poller::PollerStatus> {
                    if let Some(ps) = value
                        .get("properties")
                        .and_then(|p| p.get("provisioningState"))
                        .and_then(|v| v.as_str())
                    {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("provisioningState").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    if let Some(ps) = value.get("status").and_then(|v| v.as_str()) {
                        return Some(map_status(ps));
                    }
                    None
                }
                match serde_json::to_value(&self.0).ok().and_then(|v| find_status(&v)) {
                    Some(s) => s,
                    None => azure_core::http::poller::PollerStatus::InProgress,
                }
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) private_cloud_name: String,
            pub(crate) vm_group_id: String,
            pub(crate) workload_network_vm_group: models::WorkloadNetworkVmGroup,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/vmGroups/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.vm_group_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Patch);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::json::to_json(&this.workload_network_vm_group)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            #[doc = "Return a Poller over an LRO"]
            pub fn poller(self) -> azure_core::Result<azure_core::http::poller::Poller<Operation>> {
                let client = self.client.clone();
                let initial = self.clone();
                Ok(azure_core::http::poller::Poller::from_callback(
                    move |state: azure_core::http::poller::PollerState<azure_core::http::Url>| {
                        let client = client.clone();
                        let initial = initial.clone();
                        async move {
                            use azure_core::http::poller::{PollerResult, PollerState, StatusMonitor as _};
                            use azure_core::json;
                            let (rsp, next_link) = match state {
                                PollerState::Initial => {
                                    let rsp = initial.clone().send().await?.into_raw_response();
                                    let next = initial.clone().url()?;
                                    (rsp, next)
                                }
                                PollerState::More(next_url) => {
                                    let mut req =
                                        typespec_client_core::http::request::Request::new(next_url.clone(), azure_core::http::Method::Get);
                                    let bearer_token = client.bearer_token().await?;
                                    req.insert_header(
                                        azure_core::http::headers::AUTHORIZATION,
                                        format!("Bearer {}", bearer_token.secret()),
                                    );
                                    req.set_body(azure_openapi_core::EMPTY_BODY);
                                    let rsp = client.send(&mut req).await?;
                                    (rsp, next_url.clone())
                                }
                            };
                            if !rsp.status().is_success() {
                                return Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                    status: rsp.status(),
                                    error_code: None,
                                }));
                            }
                            let (status, headers, body) = rsp.deconstruct();
                            let retry_after =
                                azure_core::http::poller::get_retry_after(&headers, &azure_core::http::poller::PollerOptions::default());
                            let bytes = body.collect().await?;
                            let op: Operation = json::from_json(&bytes)?;
                            let response = azure_core::http::response::RawResponse::from_bytes(status, headers, bytes).into();
                            Ok(match op.status() {
                                azure_core::http::poller::PollerStatus::InProgress => PollerResult::InProgress {
                                    response,
                                    retry_after,
                                    next: next_link,
                                },
                                _ => PollerResult::Done { response },
                            })
                        }
                    },
                    None,
                ))
            }
        }
    }
    pub mod delete_vm_group {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(typespec_client_core::http::response::RawResponse);
        impl Response {
            pub fn into_raw_response(self) -> typespec_client_core::http::response::RawResponse {
                self.0
            }
            pub fn as_raw_response(&self) -> &typespec_client_core::http::response::RawResponse {
                &self.0
            }
            pub fn headers(&self) -> Headers<'_> {
                Headers(self.0.headers())
            }
        }
        impl From<Response> for typespec_client_core::http::response::RawResponse {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<typespec_client_core::http::response::RawResponse> for Response {
            fn as_ref(&self) -> &typespec_client_core::http::response::RawResponse {
                self.as_raw_response()
            }
        }
        pub struct Headers<'a>(&'a azure_core::http::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "The Location header contains the URL where the status of the long running operation can be checked."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::http::headers::HeaderName::from_static("location"))
            }
            #[doc = "The Retry-After header can indicate how long the client should wait before polling the operation status."]
            pub fn retry_after(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::http::headers::HeaderName::from_static("retry-after"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) vm_group_id: String,
            pub(crate) private_cloud_name: String,
        }
        impl RequestBuilder {
            fn url(&self) -> azure_core::Result<azure_core::http::Url> {
                let mut url = self.client.endpoint().clone();
                url.set_path(&format!(
                    "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AVS/privateClouds/{}/workloadNetworks/default/vmGroups/{}",
                    &self.subscription_id, &self.resource_group_name, &self.private_cloud_name, &self.vm_group_id
                ));
                let has_api_version_already = url
                    .query_pairs()
                    .any(|(k, _)| k == azure_core::http::headers::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::http::headers::query_param::API_VERSION, "2024-09-01");
                }
                Ok(url)
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::http::Request::new(url, azure_core::http::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(
                            azure_core::http::headers::AUTHORIZATION,
                            format!("Bearer {}", bearer_token.secret()),
                        );
                        let req_body = azure_openapi_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
