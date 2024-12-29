// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.
// Code generated by Microsoft (R) Rust Code Generator. DO NOT EDIT.

use crate::models::{CloudLink, CloudLinkList};
use azure_core::{
    ClientMethodOptions, Context, Method, Pager, Pipeline, Request, RequestContent, Response,
    Result, Url,
};
use typespec_client_core::http::PagerResult;
use typespec_client_core::json;

pub struct AVSCloudLinksClient {
    pub(crate) api_version: String,
    pub(crate) endpoint: Url,
    pub(crate) pipeline: Pipeline,
    pub(crate) subscription_id: String,
}

impl AVSCloudLinksClient {
    /// Returns the Url associated with this client.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Create a CloudLink
    pub async fn create_or_update(
        &self,
        resource_group_name: String,
        private_cloud_name: String,
        cloud_link_name: String,
        cloud_link: RequestContent<CloudLink>,
        options: Option<AVSCloudLinksClientCreateOrUpdateOptions<'_>>,
    ) -> Result<Response<CloudLink>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        let mut path = String::from("subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.AVS/privateClouds/{privateCloudName}/cloudLinks/{cloudLinkName}");
        path = path.replace("{cloudLinkName}", &cloud_link_name);
        path = path.replace("{privateCloudName}", &private_cloud_name);
        path = path.replace("{resourceGroupName}", &resource_group_name);
        path = path.replace("{subscriptionId}", &self.subscription_id);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/json");
        request.set_body(cloud_link);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Delete a CloudLink
    pub async fn delete(
        &self,
        resource_group_name: String,
        private_cloud_name: String,
        cloud_link_name: String,
        options: Option<AVSCloudLinksClientDeleteOptions<'_>>,
    ) -> Result<Response<()>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        let mut path = String::from("subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.AVS/privateClouds/{privateCloudName}/cloudLinks/{cloudLinkName}");
        path = path.replace("{cloudLinkName}", &cloud_link_name);
        path = path.replace("{privateCloudName}", &private_cloud_name);
        path = path.replace("{resourceGroupName}", &resource_group_name);
        path = path.replace("{subscriptionId}", &self.subscription_id);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Delete);
        request.insert_header("accept", "application/json");
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Get a CloudLink
    pub async fn get(
        &self,
        resource_group_name: String,
        private_cloud_name: String,
        cloud_link_name: String,
        options: Option<AVSCloudLinksClientGetOptions<'_>>,
    ) -> Result<Response<CloudLink>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        let mut path = String::from("subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.AVS/privateClouds/{privateCloudName}/cloudLinks/{cloudLinkName}");
        path = path.replace("{cloudLinkName}", &cloud_link_name);
        path = path.replace("{privateCloudName}", &private_cloud_name);
        path = path.replace("{resourceGroupName}", &resource_group_name);
        path = path.replace("{subscriptionId}", &self.subscription_id);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Get);
        request.insert_header("accept", "application/json");
        self.pipeline.send(&ctx, &mut request).await
    }

    /// List CloudLink resources by PrivateCloud
    pub fn list(
        &self,
        resource_group_name: String,
        private_cloud_name: String,
        options: Option<AVSCloudLinksClientListOptions<'_>>,
    ) -> Result<Pager<CloudLinkList>> {
        let options = options.unwrap_or_default().into_owned();
        let pipeline = self.pipeline.clone();
        let mut first_url = self.endpoint.clone();
        let mut path = String::from("subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.AVS/privateClouds/{privateCloudName}/cloudLinks");
        path = path.replace("{privateCloudName}", &private_cloud_name);
        path = path.replace("{resourceGroupName}", &resource_group_name);
        path = path.replace("{subscriptionId}", &self.subscription_id);
        first_url = first_url.join(&path)?;
        first_url
            .query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        Ok(Pager::from_callback(move |next_link: Option<Url>| {
            let url = match next_link {
                Some(next_link) => next_link,
                None => first_url.clone(),
            };
            let mut request = Request::new(url, Method::Get);
            request.insert_header("accept", "application/json");
            let ctx = options.method_options.context.clone();
            let pipeline = pipeline.clone();
            async move {
                let rsp: Response<CloudLinkList> = pipeline.send(&ctx, &mut request).await?;
                let (status, headers, body) = rsp.deconstruct();
                let bytes = body.collect().await?;
                let res: CloudLinkList = json::from_json(bytes.clone())?;
                let rsp = Response::from_bytes(status, headers, bytes);
                Ok(match res.next_link {
                    Some(next_link) => PagerResult::Continue {
                        response: rsp,
                        continuation: next_link.parse()?,
                    },
                    None => PagerResult::Complete { response: rsp },
                })
            }
        }))
    }
}

#[derive(Clone, Debug, Default)]
pub struct AVSCloudLinksClientCreateOrUpdateOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Debug, Default)]
pub struct AVSCloudLinksClientDeleteOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Debug, Default)]
pub struct AVSCloudLinksClientGetOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Debug, Default)]
pub struct AVSCloudLinksClientListOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

impl<'a> AVSCloudLinksClientListOptions<'a> {
    pub fn into_owned(self) -> AVSCloudLinksClientListOptions<'static> {
        AVSCloudLinksClientListOptions {
            method_options: ClientMethodOptions {
                context: self.method_options.context.into_owned(),
            },
        }
    }
}
