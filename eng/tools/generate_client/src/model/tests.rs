// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::Package;

const VALID: &str = r#"{
    "schema_version": 1,
    "clients": [{
        "name": "WidgetClient", "parent": null, "cross_language_id": null,
        "doc": null, "endpoint_name": "endpoint",
        "operations": [{
            "name": "get", "cross_language_id": null, "doc": null,
            "http_method": "GET", "path": "/widgets", "headers": [],
            "status_code": 200, "response_type": {"kind": "model", "name": "Widget"}
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
fn rejects_unknown_schema_version() {
    let source = VALID.replace("\"schema_version\": 1", "\"schema_version\": 2");
    let package: Package = serde_json::from_str(&source).unwrap();
    assert!(package.validate().unwrap_err().contains("schema version 2"));
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
