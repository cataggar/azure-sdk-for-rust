// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Package {
    pub(crate) schema_version: u32,
    pub(crate) clients: Vec<Client>,
    pub(crate) models: Vec<Model>,
    pub(crate) enums: Vec<Enum>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Client {
    pub(crate) name: String,
    parent: Option<String>,
    #[expect(dead_code, reason = "Preserved for future client emission")]
    cross_language_id: Option<String>,
    pub(crate) doc: Option<String>,
    #[serde(default)]
    pub(crate) authentication: Option<Authentication>,
    #[serde(default)]
    pub(crate) api_version: Option<ApiVersion>,
    endpoint_name: String,
    pub(crate) operations: Vec<Operation>,
    pub(crate) children: Vec<Client>,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase", deny_unknown_fields)]
pub(crate) enum Authentication {
    Bearer,
    OAuth2 {
        flow: OAuth2Flow,
        authorization_url: String,
        scopes: Vec<String>,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum OAuth2Flow {
    Implicit,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ApiVersion {
    pub(crate) name: String,
    pub(crate) default: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Operation {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) kind: OperationKind,
    #[expect(dead_code, reason = "Preserved for future operation emission")]
    cross_language_id: Option<String>,
    pub(crate) doc: Option<String>,
    pub(crate) http_method: String,
    pub(crate) path: String,
    pub(crate) parameters: Vec<Parameter>,
    #[serde(default)]
    pub(crate) body: Option<Body>,
    pub(crate) status_codes: Vec<u16>,
    pub(crate) response_type: Option<Type>,
    #[serde(default)]
    pub(crate) paging: Option<Paging>,
    #[serde(default)]
    pub(crate) exceptions: Vec<Exception>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum OperationKind {
    #[default]
    Basic,
    Paging,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Paging {
    pub(crate) next_link: PagingProperty,
    pub(crate) items: PagingItems,
    pub(crate) next_link_verb: String,
    pub(crate) reinject_api_version: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PagingProperty {
    pub(crate) name: String,
    pub(crate) wire_name: String,
    pub(crate) optional: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PagingItems {
    pub(crate) name: String,
    pub(crate) wire_name: String,
    pub(crate) optional: bool,
    #[serde(rename = "type")]
    pub(crate) item_type: Type,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Exception {
    status_codes: String,
    content_type: String,
    #[serde(rename = "type")]
    response_type: Type,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Body {
    pub(crate) name: String,
    #[serde(rename = "type")]
    pub(crate) body_type: Type,
    pub(crate) optional: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Parameter {
    pub(crate) name: String,
    pub(crate) wire_name: String,
    pub(crate) location: ParameterLocation,
    #[serde(rename = "type")]
    pub(crate) parameter_type: Type,
    pub(crate) optional: bool,
    pub(crate) constant: Option<String>,
    #[serde(default)]
    pub(crate) client_owned: bool,
}

#[derive(Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ParameterLocation {
    Path,
    Query,
    Header,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Model {
    pub(crate) name: String,
    #[expect(dead_code, reason = "Retained for parity checks")]
    cross_language_id: Option<String>,
    pub(crate) doc: Option<String>,
    pub(crate) fields: Vec<Field>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) wire_name: String,
    #[serde(rename = "type")]
    pub(crate) field_type: Type,
    pub(crate) optional: bool,
    pub(crate) read_only: bool,
    pub(crate) doc: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Enum {
    pub(crate) name: String,
    #[expect(dead_code, reason = "Retained for parity checks")]
    cross_language_id: Option<String>,
    pub(crate) extensible: bool,
    pub(crate) values: Vec<EnumValue>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EnumValue {
    pub(crate) name: String,
    pub(crate) wire_value: String,
}

#[derive(Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub(crate) enum Type {
    String,
    Boolean,
    Int32,
    Int64,
    Float64,
    Bytes {
        encoding: BytesEncoding,
    },
    UtcDateTime {
        format: DateTimeFormat,
        wire_type: DateTimeWireType,
    },
    Model {
        name: String,
    },
    Enum {
        name: String,
    },
    Array {
        value_type: Box<Type>,
    },
    Dict {
        value_type: Box<Type>,
    },
    Nullable {
        value_type: Box<Type>,
    },
}

#[derive(Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum BytesEncoding {
    Base64Url,
}

#[derive(Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum DateTimeFormat {
    UnixTime,
}

#[derive(Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum DateTimeWireType {
    Int32,
}

impl Package {
    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.schema_version != 2 {
            return Err(format!(
                "Unsupported TCGC model schema version {}",
                self.schema_version
            ));
        }
        let mut names = HashSet::new();
        let mut model_names = HashSet::new();
        let mut enum_names = HashSet::new();
        for model in &self.models {
            register(&mut names, &model.name, "model")?;
            model_names.insert(model.name.as_str());
        }
        for enum_value in &self.enums {
            register(&mut names, &enum_value.name, "enum")?;
            enum_names.insert(enum_value.name.as_str());
        }
        for model in &self.models {
            let mut fields = HashSet::new();
            let mut wire_names = HashSet::new();
            for field in &model.fields {
                register(&mut fields, &field.name, &model.name)?;
                register(
                    &mut wire_names,
                    &field.wire_name,
                    &format!("{} wire field", model.name),
                )?;
                field.field_type.validate(
                    &model_names,
                    &enum_names,
                    &format!("{}.{}", model.name, field.name),
                )?;
                if !matches!(field.field_type, Type::Bytes { .. }) && field.field_type.has_bytes() {
                    return Err(format!(
                        "{}.{}: encoded bytes require a direct model field",
                        model.name, field.name
                    ));
                }
                if !matches!(field.field_type, Type::UtcDateTime { .. })
                    && field.field_type.has_datetime()
                {
                    return Err(format!(
                        "{}.{}: encoded dates require a direct model field",
                        model.name, field.name
                    ));
                }
            }
        }
        for enum_value in &self.enums {
            let mut members = HashSet::new();
            let mut wire_values = HashSet::new();
            for value in &enum_value.values {
                register(&mut members, &value.name, &enum_value.name)?;
                if !wire_values.insert(value.wire_value.as_str()) {
                    return Err(format!(
                        "{}: duplicate wire enum value '{}'",
                        enum_value.name, value.wire_value
                    ));
                }
            }
        }
        let mut clients = HashSet::new();
        for client in &self.clients {
            validate_client(
                client,
                None,
                &mut clients,
                &model_names,
                &enum_names,
                &self.models,
            )?;
        }
        Ok(())
    }
}

fn register<'a>(names: &mut HashSet<&'a str>, name: &'a str, scope: &str) -> Result<(), String> {
    if name.is_empty() || !names.insert(name) {
        return Err(format!("{scope}: duplicate or empty name '{name}'"));
    }
    Ok(())
}

impl Type {
    fn has_bytes(&self) -> bool {
        match self {
            Self::Bytes { .. } => true,
            Self::Array { value_type }
            | Self::Dict { value_type }
            | Self::Nullable { value_type } => value_type.has_bytes(),
            _ => false,
        }
    }

    fn has_datetime(&self) -> bool {
        match self {
            Self::UtcDateTime { .. } => true,
            Self::Array { value_type }
            | Self::Dict { value_type }
            | Self::Nullable { value_type } => value_type.has_datetime(),
            _ => false,
        }
    }

    fn validate(
        &self,
        models: &HashSet<&str>,
        enums: &HashSet<&str>,
        scope: &str,
    ) -> Result<(), String> {
        match self {
            Self::Model { name } if !models.contains(name.as_str()) => {
                Err(format!("{scope}: unresolved model '{name}'"))
            }
            Self::Enum { name } if !enums.contains(name.as_str()) => {
                Err(format!("{scope}: unresolved model or enum '{name}'"))
            }
            Self::Array { value_type }
            | Self::Dict { value_type }
            | Self::Nullable { value_type } => value_type.validate(models, enums, scope),
            _ => Ok(()),
        }
    }
}

fn validate_client<'a>(
    client: &'a Client,
    parent: Option<&str>,
    clients: &mut HashSet<&'a str>,
    models: &HashSet<&str>,
    enums: &HashSet<&str>,
    all_models: &[Model],
) -> Result<(), String> {
    register(clients, &client.name, "client")?;
    if client.parent.as_deref() != parent {
        return Err(format!("client {}: invalid parent", client.name));
    }
    if client.endpoint_name.is_empty() {
        return Err(format!("client {}: missing endpoint", client.name));
    }
    if let Some(Authentication::OAuth2 {
        authorization_url,
        scopes,
        ..
    }) = &client.authentication
    {
        let mut unique = HashSet::new();
        if !authorization_url.starts_with("https://")
            || authorization_url.chars().any(char::is_whitespace)
            || scopes.is_empty()
            || scopes
                .iter()
                .any(|scope| scope.is_empty() || scope.trim() != scope || !unique.insert(scope))
        {
            return Err(format!(
                "client {}: OAuth2 scopes must be nonempty, unique, and trimmed",
                client.name
            ));
        }
        if let Some(version) = &client.api_version {
            if version.name.is_empty()
                || version.default.is_empty()
                || version.default.chars().any(char::is_control)
            {
                return Err(format!(
                    "client {}: invalid API version initializer",
                    client.name
                ));
            }
        }
    }
    let mut methods = HashSet::new();
    for operation in &client.operations {
        register(&mut methods, &operation.name, &client.name)?;
        if !["GET", "PUT", "POST", "PATCH", "DELETE", "HEAD"]
            .contains(&operation.http_method.as_str())
            || !operation.path.starts_with('/')
            || operation.status_codes.is_empty()
            || operation
                .status_codes
                .iter()
                .any(|code| !(200..400).contains(code))
        {
            return Err(format!(
                "{}.{}: invalid HTTP request or status",
                client.name, operation.name
            ));
        }
        let mut parameter_names = HashSet::new();
        let mut wire_bindings = HashSet::new();
        let mut path_bindings = HashSet::new();
        if operation.status_codes.iter().collect::<HashSet<_>>().len()
            != operation.status_codes.len()
        {
            return Err(format!(
                "{}.{}: duplicate success status",
                client.name, operation.name
            ));
        }
        if operation.response_type.is_some()
            && operation
                .status_codes
                .iter()
                .any(|code| matches!(*code, 204 | 205 | 304))
        {
            return Err(format!(
                "{}.{}: body response on a no-content success status",
                client.name, operation.name
            ));
        }
        for parameter in &operation.parameters {
            let scope = format!("{}.{}", client.name, operation.name);
            register(&mut parameter_names, &parameter.name, &scope)?;
            if parameter.client_owned
                && (client
                    .api_version
                    .as_ref()
                    .is_none_or(|v| v.name != parameter.name)
                    || parameter.location != ParameterLocation::Query
                    || !matches!(parameter.parameter_type, Type::String)
                    || parameter.optional
                    || parameter.constant.is_some())
            {
                return Err(format!(
                    "{scope}.{}: invalid client-owned binding",
                    parameter.name
                ));
            }
            if parameter.wire_name.is_empty()
                || !wire_bindings.insert((&parameter.location, parameter.wire_name.as_str()))
            {
                return Err(format!("{scope}: duplicate or empty wire binding"));
            }
            parameter.parameter_type.validate(
                models,
                enums,
                &format!("{scope}.{}", parameter.name),
            )?;
            if !matches!(
                parameter.parameter_type,
                Type::String
                    | Type::Boolean
                    | Type::Int32
                    | Type::Int64
                    | Type::Float64
                    | Type::Enum { .. }
            ) {
                return Err(format!(
                    "{scope}.{}: unsupported HTTP parameter type",
                    parameter.name
                ));
            }
            if matches!(parameter.parameter_type, Type::Enum { .. })
                && parameter.location != ParameterLocation::Query
            {
                return Err(format!(
                    "{scope}.{}: enum parameters require a query binding",
                    parameter.name
                ));
            }
            if parameter.location == ParameterLocation::Path
                && (!path_bindings.insert(parameter.wire_name.as_str())
                    || (parameter.optional
                        && (!matches!(parameter.parameter_type, Type::String)
                            || !operation
                                .path
                                .ends_with(&format!("/{{{}}}", parameter.wire_name)))))
            {
                return Err(format!("{scope}.{}: invalid path binding", parameter.name));
            }
            if parameter
                .constant
                .as_ref()
                .is_some_and(|value| value.contains(['\r', '\n']))
            {
                return Err(format!("{scope}.{}: invalid constant", parameter.name));
            }
        }
        let scope = format!("{}.{}", client.name, operation.name);
        if let Some(body) = &operation.body {
            register(&mut parameter_names, &body.name, &scope)?;
            body.body_type
                .validate(models, enums, &format!("{scope}.{}", body.name))?;
            if body.optional || !matches!(body.body_type, Type::Model { .. }) {
                return Err(format!("{scope}: unsupported JSON request body"));
            }
        }
        let template_bindings = path_bindings_in(&operation.path, &scope)?;
        if path_bindings != template_bindings {
            return Err(format!(
                "{scope}: path template and parameter bindings differ"
            ));
        }
        if let Some(response) = &operation.response_type {
            response.validate(
                models,
                enums,
                &format!("{}.{}", client.name, operation.name),
            )?;
            if response.has_bytes() || response.has_datetime() {
                return Err(format!("{scope}: encoded types require a model response"));
            }
        }
        match (&operation.kind, &operation.paging) {
            (OperationKind::Basic, None) => {}
            (OperationKind::Paging, Some(paging)) => {
                validate_paging(client, operation, paging, all_models, &scope)?;
            }
            _ => return Err(format!("{scope}: missing or unexpected paging metadata")),
        }
        for exception in &operation.exceptions {
            exception.response_type.validate(models, enums, &scope)?;
            if operation.exceptions.len() != 1
                || exception.status_codes != "*"
                || exception.content_type != "application/json"
                || !matches!(&exception.response_type, Type::Model { name } if name == "KeyVaultError")
                || !is_standard_keyvault_error(all_models)
            {
                return Err(format!("{scope}: unsupported explicit exception response"));
            }
        }
    }
    for child in &client.children {
        validate_client(
            child,
            Some(&client.name),
            clients,
            models,
            enums,
            all_models,
        )?;
    }
    Ok(())
}

fn validate_paging(
    client: &Client,
    operation: &Operation,
    paging: &Paging,
    models: &[Model],
    scope: &str,
) -> Result<(), String> {
    let Type::Model { name } = operation
        .response_type
        .as_ref()
        .ok_or_else(|| format!("{scope}: paging response model is required"))?
    else {
        return Err(format!("{scope}: paging response must be a model"));
    };
    let wrapper = models
        .iter()
        .find(|model| model.name == *name)
        .ok_or_else(|| format!("{scope}: unresolved paging response model"))?;
    let next = wrapper.fields.iter().find(|field| {
        field.name == paging.next_link.name && field.wire_name == paging.next_link.wire_name
    });
    let items = wrapper
        .fields
        .iter()
        .find(|field| field.name == paging.items.name && field.wire_name == paging.items.wire_name);
    if operation.http_method != "GET"
        || operation.status_codes != [200]
        || operation.body.is_some()
        || paging.next_link_verb != "GET"
        || !paging.reinject_api_version
        || client.api_version.is_none()
        || !operation
            .parameters
            .iter()
            .any(|param| param.client_owned && param.wire_name == "api-version")
        || next.is_none_or(|field| {
            !field.optional || !paging.next_link.optional || field.field_type != Type::String
        })
        || items.is_none_or(|field| {
            !field.optional || !paging.items.optional || field.field_type != paging.items.item_type
        })
        || !matches!(
            &paging.items.item_type,
            Type::Array { value_type } if matches!(**value_type, Type::Model { .. })
        )
    {
        return Err(format!("{scope}: unsupported paging contract"));
    }
    Ok(())
}

fn is_standard_keyvault_error(models: &[Model]) -> bool {
    let Some(root) = models.iter().find(|model| model.name == "KeyVaultError") else {
        return false;
    };
    let Some(error) = root.fields.iter().find(|field| field.wire_name == "error") else {
        return false;
    };
    let Type::Model { name } = &error.field_type else {
        return false;
    };
    let Some(detail) = models.iter().find(|model| model.name == *name) else {
        return false;
    };
    ["code", "message"].iter().all(|wire| {
        detail
            .fields
            .iter()
            .any(|field| field.wire_name == *wire && matches!(field.field_type, Type::String))
    })
}

fn path_bindings_in<'a>(path: &'a str, scope: &str) -> Result<HashSet<&'a str>, String> {
    if path.contains(['?', '#']) {
        return Err(format!("{scope}: query and fragment in operation path"));
    }
    let mut bindings = HashSet::new();
    let mut remaining = path;
    while let Some(open) = remaining.find('{') {
        if remaining[..open].contains('}') {
            return Err(format!("{scope}: malformed path template"));
        }
        let suffix = &remaining[open + 1..];
        let close = suffix
            .find('}')
            .ok_or_else(|| format!("{scope}: malformed path template"))?;
        let name = &suffix[..close];
        if name.is_empty() || name.contains('{') || !bindings.insert(name) {
            return Err(format!("{scope}: invalid path template binding"));
        }
        remaining = &suffix[close + 1..];
    }
    if remaining.contains('}') {
        return Err(format!("{scope}: malformed path template"));
    }
    Ok(bindings)
}

#[cfg(test)]
mod tests;
