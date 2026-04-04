// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Code model types for the TypeSpec Rust code generator.
//!
//! These types mirror the TCGC (TypeSpec Client Generator Core) code model
//! and serve as the intermediate representation between TypeSpec input and
//! generated Rust output.

use serde::{Deserialize, Serialize};

/// A complete Rust crate to be generated.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Crate {
    pub name: String,
    pub version: String,
    pub service_type: ServiceType,
    pub clients: Vec<Client>,
    pub models: Vec<Model>,
    pub enums: Vec<Enum>,
    pub unions: Vec<Union>,
}

/// Whether this is an ARM management or data-plane crate.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ServiceType {
    AzureArm,
    DataPlane,
}

/// A service client with operations grouped by resource.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Client {
    pub name: String,
    pub doc: Option<String>,
    pub endpoint: EndpointParameter,
    pub parameters: Vec<ClientParameter>,
    pub methods: Vec<Method>,
    pub sub_clients: Vec<SubClient>,
    pub credential_scopes: Vec<String>,
}

/// Reference to a sub-client accessible from the parent client.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubClient {
    pub name: String,
    pub accessor_name: String,
    pub client_name: String,
}

/// Client constructor parameter.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientParameter {
    pub name: String,
    pub doc: Option<String>,
    pub param_type: TypeRef,
    pub optional: bool,
}

/// Endpoint parameter for the client constructor.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EndpointParameter {
    pub name: String,
    pub default_value: Option<String>,
}

/// A client method (operation).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Method {
    pub name: String,
    pub doc: Option<String>,
    pub http_method: HttpMethod,
    pub path: String,
    pub parameters: Vec<MethodParameter>,
    pub response: ResponseType,
    pub paging: Option<PagingInfo>,
    pub lro: Option<LroInfo>,
}

/// HTTP method.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Put,
    Post,
    Patch,
    Delete,
    Head,
}

/// Method parameter with location info.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MethodParameter {
    pub name: String,
    pub doc: Option<String>,
    pub param_type: TypeRef,
    pub location: ParameterLocation,
    pub optional: bool,
    pub format: Option<String>,
}

/// Where a parameter is sent in the HTTP request.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ParameterLocation {
    Path,
    Query,
    Header,
    Body,
}

/// Response type for a method.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResponseType {
    pub body: Option<TypeRef>,
    pub headers: Vec<ResponseHeader>,
    pub success_codes: Vec<u16>,
}

/// A response header.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResponseHeader {
    pub name: String,
    pub header_name: String,
    pub header_type: TypeRef,
}

/// Pagination metadata.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PagingInfo {
    pub items_path: String,
    pub next_link_path: Option<String>,
}

/// Long-running operation metadata.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LroInfo {
    pub polling_strategy: String,
    pub final_state_via: Option<String>,
}

/// A model (struct) definition.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Model {
    pub name: String,
    pub doc: Option<String>,
    pub fields: Vec<ModelField>,
    pub parents: Vec<String>,
    pub is_input: bool,
    pub is_output: bool,
}

/// A field within a model.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModelField {
    pub name: String,
    pub serialized_name: String,
    pub doc: Option<String>,
    pub field_type: TypeRef,
    pub optional: bool,
    pub read_only: bool,
    pub flatten: bool,
}

/// An enum definition.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Enum {
    pub name: String,
    pub doc: Option<String>,
    pub values: Vec<EnumValue>,
    pub value_type: ScalarType,
    pub extensible: bool,
}

/// A single enum variant.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnumValue {
    pub name: String,
    pub value: serde_json::Value,
    pub doc: Option<String>,
}

/// A discriminated or untagged union.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Union {
    pub name: String,
    pub doc: Option<String>,
    pub discriminant: Option<String>,
    pub variants: Vec<UnionVariant>,
}

/// A variant within a union.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnionVariant {
    pub name: String,
    pub variant_type: TypeRef,
    pub discriminator_value: Option<String>,
}

/// A reference to a type, possibly with generics.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", content = "value")]
pub enum TypeRef {
    Scalar(ScalarType),
    Model(String),
    Enum(String),
    Union(String),
    Array(Box<TypeRef>),
    Map {
        key: Box<TypeRef>,
        value: Box<TypeRef>,
    },
    Option(Box<TypeRef>),
}

/// Built-in scalar types.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ScalarType {
    Bool,
    I32,
    I64,
    F32,
    F64,
    String,
    Bytes,
    DateTime,
    Url,
    Uuid,
}
