// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Package {
    schema_version: u32,
    clients: Vec<Client>,
    models: Vec<Model>,
    enums: Vec<Enum>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Client {
    name: String,
    parent: Option<String>,
    cross_language_id: Option<String>,
    doc: Option<String>,
    endpoint_name: String,
    operations: Vec<Operation>,
    children: Vec<Client>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Operation {
    name: String,
    cross_language_id: Option<String>,
    doc: Option<String>,
    http_method: String,
    path: String,
    headers: Vec<Header>,
    status_code: u16,
    response_type: Option<Type>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Header {
    wire_name: String,
    value: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Model {
    name: String,
    cross_language_id: Option<String>,
    doc: Option<String>,
    fields: Vec<Field>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Field {
    name: String,
    wire_name: String,
    #[serde(rename = "type")]
    field_type: Type,
    optional: bool,
    read_only: bool,
    doc: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Enum {
    name: String,
    cross_language_id: Option<String>,
    extensible: bool,
    values: Vec<EnumValue>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EnumValue {
    name: String,
    wire_value: String,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
enum Type {
    String,
    Boolean,
    Int32,
    Int64,
    Float64,
    Bytes,
    UtcDateTime,
    Model { name: String },
    Enum { name: String },
    Array { value_type: Box<Type> },
    Dict { value_type: Box<Type> },
    Nullable { value_type: Box<Type> },
}

impl Package {
    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "Unsupported TCGC model schema version {}",
                self.schema_version
            ));
        }
        let mut names = HashSet::new();
        for model in &self.models {
            register(&mut names, &model.name, "model")?;
        }
        for enum_value in &self.enums {
            register(&mut names, &enum_value.name, "enum")?;
        }
        for model in &self.models {
            let mut fields = HashSet::new();
            for field in &model.fields {
                register(&mut fields, &field.name, &model.name)?;
                field
                    .field_type
                    .validate(&names, &format!("{}.{}", model.name, field.name))?;
            }
        }
        for enum_value in &self.enums {
            let mut members = HashSet::new();
            for value in &enum_value.values {
                register(&mut members, &value.name, &enum_value.name)?;
            }
        }
        let mut clients = HashSet::new();
        for client in &self.clients {
            validate_client(client, None, &mut clients, &names)?;
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
    fn validate(&self, names: &HashSet<&str>, scope: &str) -> Result<(), String> {
        match self {
            Self::Model { name } | Self::Enum { name } if !names.contains(name.as_str()) => {
                Err(format!("{scope}: unresolved model or enum '{name}'"))
            }
            Self::Array { value_type }
            | Self::Dict { value_type }
            | Self::Nullable { value_type } => value_type.validate(names, scope),
            _ => Ok(()),
        }
    }
}

fn validate_client<'a>(
    client: &'a Client,
    parent: Option<&str>,
    clients: &mut HashSet<&'a str>,
    names: &HashSet<&str>,
) -> Result<(), String> {
    register(clients, &client.name, "client")?;
    if client.parent.as_deref() != parent {
        return Err(format!("client {}: invalid parent", client.name));
    }
    if client.endpoint_name.is_empty() {
        return Err(format!("client {}: missing endpoint", client.name));
    }
    let mut methods = HashSet::new();
    for operation in &client.operations {
        register(&mut methods, &operation.name, &client.name)?;
        if !["GET", "PUT", "POST", "PATCH", "DELETE", "HEAD"]
            .contains(&operation.http_method.as_str())
            || operation.path.is_empty()
            || !(100..600).contains(&operation.status_code)
        {
            return Err(format!(
                "{}.{}: invalid HTTP request or status",
                client.name, operation.name
            ));
        }
        if let Some(response) = &operation.response_type {
            response.validate(names, &format!("{}.{}", client.name, operation.name))?;
        }
    }
    for child in &client.children {
        validate_client(child, Some(&client.name), clients, names)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
