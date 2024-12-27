// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.
// Code generated by Microsoft (R) Rust Code Generator. DO NOT EDIT.

use crate::models::{ScriptExecution, ScriptExecutionsList, ScriptOutputStreamType};
use azure_core::{
    ClientMethodOptions, Context, Method, Pager, Pipeline, Request, RequestContent, Response,
    Result, Url,
};
use typespec_client_core::http::PagerResult;
use typespec_client_core::json;

pub struct AVSScriptExecutionsClient {
    pub(crate) api_version: String,
    pub(crate) endpoint: Url,
    pub(crate) pipeline: Pipeline,
    pub(crate) subscription_id: String,
}

impl AVSScriptExecutionsClient {
    /// Returns the Url associated with this client.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Create a ScriptExecution
    pub async fn create_or_update(
        &self,
        resource_group_name: String,
        private_cloud_name: String,
        script_execution_name: String,
        script_execution: RequestContent<ScriptExecution>,
        options: Option<AVSScriptExecutionsClientCreateOrUpdateOptions<'_>>,
    ) -> Result<Response<ScriptExecution>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        let mut path = String::from("subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.AVS/privateClouds/{privateCloudName}/scriptExecutions/{scriptExecutionName}");
        path = path.replace("{privateCloudName}", &private_cloud_name);
        path = path.replace("{resourceGroupName}", &resource_group_name);
        path = path.replace("{scriptExecutionName}", &script_execution_name);
        path = path.replace("{subscriptionId}", &self.subscription_id);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/json");
        request.set_body(script_execution);
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Delete a ScriptExecution
    pub async fn delete(
        &self,
        resource_group_name: String,
        private_cloud_name: String,
        script_execution_name: String,
        options: Option<AVSScriptExecutionsClientDeleteOptions<'_>>,
    ) -> Result<Response<()>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        let mut path = String::from("subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.AVS/privateClouds/{privateCloudName}/scriptExecutions/{scriptExecutionName}");
        path = path.replace("{privateCloudName}", &private_cloud_name);
        path = path.replace("{resourceGroupName}", &resource_group_name);
        path = path.replace("{scriptExecutionName}", &script_execution_name);
        path = path.replace("{subscriptionId}", &self.subscription_id);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Delete);
        request.insert_header("accept", "application/json");
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Get a ScriptExecution
    pub async fn get(
        &self,
        resource_group_name: String,
        private_cloud_name: String,
        script_execution_name: String,
        options: Option<AVSScriptExecutionsClientGetOptions<'_>>,
    ) -> Result<Response<ScriptExecution>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        let mut path = String::from("subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.AVS/privateClouds/{privateCloudName}/scriptExecutions/{scriptExecutionName}");
        path = path.replace("{privateCloudName}", &private_cloud_name);
        path = path.replace("{resourceGroupName}", &resource_group_name);
        path = path.replace("{scriptExecutionName}", &script_execution_name);
        path = path.replace("{subscriptionId}", &self.subscription_id);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Get);
        request.insert_header("accept", "application/json");
        self.pipeline.send(&ctx, &mut request).await
    }

    /// Return the logs for a script execution resource
    pub async fn get_execution_logs(
        &self,
        resource_group_name: String,
        private_cloud_name: String,
        script_execution_name: String,
        options: Option<AVSScriptExecutionsClientGetExecutionLogsOptions<'_>>,
    ) -> Result<Response<ScriptExecution>> {
        let options = options.unwrap_or_default();
        let ctx = Context::with_context(&options.method_options.context);
        let mut url = self.endpoint.clone();
        let mut path = String::from("subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.AVS/privateClouds/{privateCloudName}/scriptExecutions/{scriptExecutionName}/getExecutionLogs");
        path = path.replace("{privateCloudName}", &private_cloud_name);
        path = path.replace("{resourceGroupName}", &resource_group_name);
        path = path.replace("{scriptExecutionName}", &script_execution_name);
        path = path.replace("{subscriptionId}", &self.subscription_id);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Post);
        request.insert_header("accept", "application/json");
        if let Some(content_type) = options.content_type {
            request.insert_header("content-type", "application/json");
        }
        if let Some(script_output_stream_type) = options.script_output_stream_type {
            request.set_body(script_output_stream_type);
        }
        self.pipeline.send(&ctx, &mut request).await
    }

    /// List ScriptExecution resources by PrivateCloud
    pub fn list(
        &self,
        resource_group_name: String,
        private_cloud_name: String,
        options: Option<AVSScriptExecutionsClientListOptions<'_>>,
    ) -> Result<Pager<ScriptExecutionsList>> {
        let options = options.unwrap_or_default().into_owned();
        let pipeline = self.pipeline.clone();
        let mut first_url = self.endpoint.clone();
        let mut path = String::from("subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.AVS/privateClouds/{privateCloudName}/scriptExecutions");
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
                let rsp: Response<ScriptExecutionsList> = pipeline.send(&ctx, &mut request).await?;
                let (status, headers, body) = rsp.deconstruct();
                let bytes = body.collect().await?;
                let res: ScriptExecutionsList = json::from_json(bytes.clone())?;
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
pub struct AVSScriptExecutionsClientCreateOrUpdateOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Debug, Default)]
pub struct AVSScriptExecutionsClientDeleteOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Debug, Default)]
pub struct AVSScriptExecutionsClientGetOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Debug, Default)]
pub struct AVSScriptExecutionsClientGetExecutionLogsOptions<'a> {
    pub content_type: undefined,
    pub method_options: ClientMethodOptions<'a>,
    pub script_output_stream_type: Option<RequestContent<Vec<ScriptOutputStreamType>>>,
}

#[derive(Clone, Debug, Default)]
pub struct AVSScriptExecutionsClientListOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

impl<'a> AVSScriptExecutionsClientListOptions<'a> {
    pub fn into_owned(self) -> AVSScriptExecutionsClientListOptions<'static> {
        AVSScriptExecutionsClientListOptions {
            method_options: ClientMethodOptions {
                context: self.method_options.context.into_owned(),
            },
        }
    }
}
