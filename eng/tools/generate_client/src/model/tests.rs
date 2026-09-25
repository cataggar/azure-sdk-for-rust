// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::Package;
use serde_json::{json, Value};

const VALID: &str = r#"{
    "schema_version": 2,
    "clients": [{
        "name": "WidgetClient", "parent": null, "cross_language_id": null,
        "doc": null, "endpoint_name": "endpoint",
        "operations": [{
            "name": "get", "cross_language_id": null, "doc": null,
            "http_method": "GET", "path": "/widgets/{widgetId}",
            "parameters": [{"name":"widget_id","wire_name":"widgetId","location":"path",
                            "type":{"kind":"string"},"optional":false,"constant":null}],
            "status_codes": [200, 201], "response_type": {"kind": "model", "name": "Widget"}
        }],
        "children": []
    }],
    "models": [{
        "name": "Widget", "cross_language_id": null, "doc": null,
        "fields": [{
            "name": "label", "wire_name": "label", "type": {"kind": "string"},
            "optional": false, "read_only": false, "doc": null
        }]
    }],
    "enums": []
}"#;

#[test]
fn validates_forward_references() {
    let package: Package = serde_json::from_str(VALID).unwrap();
    package.validate().unwrap();
}

#[test]
fn validates_basic_http_fixture() {
    let package: Package =
        serde_json::from_str(include_str!("../../tests/fixtures/basic-http.json")).unwrap();
    package.validate().unwrap();
}

#[test]
fn validates_bodyless_http_fixture() {
    let package: Package =
        serde_json::from_str(include_str!("../../tests/fixtures/bodyless-http.json")).unwrap();
    package.validate().unwrap();
}

#[test]
fn validates_basic_json_body_fixture() {
    let package: Package =
        serde_json::from_str(include_str!("../../tests/fixtures/basic-json-body.json")).unwrap();
    package.validate().unwrap();
}

#[test]
fn validates_pinned_keyvault_secrets_model() {
    let package: Package = serde_json::from_str(include_str!(
        "../../tests/fixtures/keyvault-secrets-8d521358.json"
    ))
    .unwrap();
    package.validate().unwrap();
    assert_eq!(package.clients.len(), 1);
    assert_eq!(package.clients[0].operations.len(), 12);
    assert_eq!(package.models.len(), 13);
    assert_eq!(package.enums.len(), 3);
    assert_eq!(
        package.clients[0]
            .operations
            .iter()
            .filter(|operation| matches!(operation.kind, super::OperationKind::Paging))
            .count(),
        3
    );
}

#[test]
fn validates_paging_contract_and_rejects_drift() {
    let input: Value =
        serde_json::from_str(include_str!("../../tests/fixtures/basic-paging.json")).unwrap();
    let package: Package = serde_json::from_value(input.clone()).unwrap();
    package.validate().unwrap();

    let mut invalid = input.clone();
    invalid["clients"][0]["operations"][0]["paging"]["items"]["wire_name"] = json!("other");
    let package: Package = serde_json::from_value(invalid).unwrap();
    assert!(package
        .validate()
        .unwrap_err()
        .contains("unsupported paging contract"));

    let mut invalid = input;
    invalid["clients"][0]["operations"][0]["paging"]["next_link_verb"] = json!("POST");
    let package: Package = serde_json::from_value(invalid).unwrap();
    assert!(package
        .validate()
        .unwrap_err()
        .contains("unsupported paging contract"));
}

#[test]
fn accepts_only_known_standard_keyvault_exception_shape() {
    let mut input: Value = serde_json::from_str(VALID).unwrap();
    input["models"].as_array_mut().unwrap().extend([
        json!({
            "name":"KeyVaultError","cross_language_id":null,"doc":null,
            "fields":[{"name":"error","wire_name":"error",
                       "type":{"kind":"model","name":"KeyVaultErrorError"},
                       "optional":true,"read_only":false,"doc":null}]
        }),
        json!({
            "name":"KeyVaultErrorError","cross_language_id":null,"doc":null,
            "fields":[
                {"name":"code","wire_name":"code","type":{"kind":"string"},
                 "optional":true,"read_only":false,"doc":null},
                {"name":"message","wire_name":"message","type":{"kind":"string"},
                 "optional":true,"read_only":false,"doc":null}]
        }),
    ]);
    input["clients"][0]["operations"][0]["exceptions"] = json!([{
        "status_codes":"*","content_type":"application/json",
        "type":{"kind":"model","name":"KeyVaultError"}
    }]);
    let package: Package = serde_json::from_value(input.clone()).unwrap();
    package.validate().unwrap();

    input["clients"][0]["operations"][0]["exceptions"][0]["status_codes"] = json!("404");
    let package: Package = serde_json::from_value(input.clone()).unwrap();
    assert!(package
        .validate()
        .unwrap_err()
        .contains("unsupported explicit exception"));

    input["clients"][0]["operations"][0]["exceptions"][0]["status_codes"] = json!("*");
    input["models"][2]["fields"][1]["wire_name"] = json!("detail");
    let package: Package = serde_json::from_value(input).unwrap();
    assert!(package
        .validate()
        .unwrap_err()
        .contains("unsupported explicit exception"));
}

#[test]
fn validates_supported_authentication_and_rejects_unknown_schemes() {
    let source = VALID.replace(
        "\"endpoint_name\": \"endpoint\"",
        "\"authentication\": {\"kind\":\"bearer\"}, \"endpoint_name\": \"endpoint\"",
    );
    let package: Package = serde_json::from_str(&source).unwrap();
    package.validate().unwrap();

    let unsupported = source.replace("\"kind\":\"bearer\"", "\"kind\":\"apiKey\"");
    assert!(serde_json::from_str::<Package>(&unsupported).is_err());

    let oauth2 = source.replace(
        "\"kind\":\"bearer\"",
        "\"kind\":\"oauth2\", \"flow\":\"implicit\", \
         \"authorization_url\":\"https://login.microsoftonline.com/common/oauth2/authorize\", \
         \"scopes\":[\"https://vault.azure.net/.default\"]",
    );
    let package: Package = serde_json::from_str(&oauth2).unwrap();
    package.validate().unwrap();

    let missing_scopes = oauth2.replace("\"https://vault.azure.net/.default\"", "\" \"");
    let package: Package = serde_json::from_str(&missing_scopes).unwrap();
    assert!(package.validate().unwrap_err().contains("OAuth2 scopes"));

    let duplicate_scopes = oauth2.replace(
        "\"https://vault.azure.net/.default\"]",
        "\"https://vault.azure.net/.default\", \"https://vault.azure.net/.default\"]",
    );
    let package: Package = serde_json::from_str(&duplicate_scopes).unwrap();
    assert!(package.validate().unwrap_err().contains("OAuth2 scopes"));
}

#[test]
fn validates_client_owned_api_version_bindings() {
    let source = VALID.replace(
        "\"endpoint_name\": \"endpoint\"",
        "\"api_version\":{\"name\":\"apiVersion\",\"default\":\"2026-05-01-preview\"}, \
         \"endpoint_name\": \"endpoint\"",
    );
    let source = source.replace(
        "\"optional\":false,\"constant\":null}",
        "\"optional\":false,\"constant\":null}, \
         {\"name\":\"apiVersion\",\"wire_name\":\"api-version\",\"location\":\"query\",\
          \"type\":{\"kind\":\"string\"},\"optional\":false,\"constant\":null,\"client_owned\":true}",
    );
    let package: Package = serde_json::from_str(&source).unwrap();
    package.validate().unwrap();

    let invalid = source.replace(
        "\"optional\":false,\"constant\":null,\"client_owned\":true}",
        "\"optional\":true,\"constant\":null,\"client_owned\":true}",
    );
    let package: Package = serde_json::from_str(&invalid).unwrap();
    assert!(package
        .validate()
        .unwrap_err()
        .contains("client-owned binding"));
    let invalid = source.replace(
        "\"name\":\"apiVersion\",\"wire_name\"",
        "\"name\":\"version\",\"wire_name\"",
    );
    let package: Package = serde_json::from_str(&invalid).unwrap();
    assert!(package
        .validate()
        .unwrap_err()
        .contains("client-owned binding"));
}

#[test]
fn optional_path_requires_final_string_segment() {
    let mut input: Value = serde_json::from_str(VALID).unwrap();
    input["clients"][0]["operations"][0]["path"] = json!("/widgets/{widgetId}/{version}");
    input["clients"][0]["operations"][0]["parameters"]
        .as_array_mut()
        .unwrap()
        .push(
            json!({"name":"version","wire_name":"version","location":"path",
                    "type":{"kind":"string"},"optional":true,"constant":null}),
        );
    let package: Package = serde_json::from_value(input.clone()).unwrap();
    package.validate().unwrap();

    input["clients"][0]["operations"][0]["path"] = json!("/widgets/{version}/{widgetId}");
    let package: Package = serde_json::from_value(input).unwrap();
    assert!(package
        .validate()
        .unwrap_err()
        .contains("invalid path binding"));
}

#[test]
fn rejects_unknown_schema_version() {
    let source = VALID.replace("\"schema_version\": 2", "\"schema_version\": 3");
    let package: Package = serde_json::from_str(&source).unwrap();
    assert!(package.validate().unwrap_err().contains("schema version 3"));
}

#[test]
fn requires_matching_path_bindings_and_success_codes() {
    let source = VALID.replace("\"wire_name\":\"widgetId\"", "\"wire_name\":\"wrong\"");
    let package: Package = serde_json::from_str(&source).unwrap();
    assert!(package
        .validate()
        .unwrap_err()
        .contains("path template and parameter bindings differ"));

    let source = VALID.replace(
        "\"status_codes\": [200, 201]",
        "\"status_codes\": [200, 200]",
    );
    let package: Package = serde_json::from_str(&source).unwrap();
    assert!(package
        .validate()
        .unwrap_err()
        .contains("duplicate success status"));

    let source = VALID.replace(
        "\"status_codes\": [200, 201]",
        "\"status_codes\": [200, 204]",
    );
    let package: Package = serde_json::from_str(&source).unwrap();
    assert!(package
        .validate()
        .unwrap_err()
        .contains("no-content success"));

    let source = VALID.replace("/widgets/{widgetId}", "/widgets/{widgetId");
    let package: Package = serde_json::from_str(&source).unwrap();
    assert!(package
        .validate()
        .unwrap_err()
        .contains("malformed path template"));

    let source = VALID.replace("/widgets/{widgetId}", "/widgets/{widgetId}?debug=1");
    let package: Package = serde_json::from_str(&source).unwrap();
    assert!(package
        .validate()
        .unwrap_err()
        .contains("query and fragment"));
}

#[test]
fn rejects_unresolved_model() {
    let source = VALID.replace("\"name\": \"Widget\"}", "\"name\": \"Missing\"}");
    let package: Package = serde_json::from_str(&source).unwrap();
    assert!(package.validate().unwrap_err().contains("unresolved model"));
}

#[test]
fn rejects_unknown_type() {
    let source = VALID.replace("\"kind\": \"string\"", "\"kind\": \"unrecognized\"");
    assert!(serde_json::from_str::<Package>(&source).is_err());
}

#[test]
fn rejects_wrong_reference_kind_and_duplicate_wire_names() {
    let source = VALID.replace(
        "\"kind\": \"string\"",
        "\"kind\": \"enum\", \"name\": \"Widget\"",
    );
    let package: Package = serde_json::from_str(&source).unwrap();
    assert!(package
        .validate()
        .unwrap_err()
        .contains("unresolved model or enum"));

    let source = VALID.replace(
        "\"optional\": false, \"read_only\": false, \"doc\": null",
        "\"optional\": false, \"read_only\": false, \"doc\": null}, {\
         \"name\": \"other\", \"wire_name\": \"label\", \"type\": {\"kind\": \"string\"},\
         \"optional\": false, \"read_only\": false, \"doc\": null",
    );
    let package: Package = serde_json::from_str(&source).unwrap();
    assert!(package.validate().unwrap_err().contains("duplicate"));
}
