// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::collect;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT_TEST: AtomicUsize = AtomicUsize::new(0);

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "generate-client-source-{}-{}",
            std::process::id(),
            NEXT_TEST.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn collects_project_and_resources_at_stable_virtual_paths() {
    let dir = TestDirectory::new();
    let project = dir.0.join("spec");
    let resources = dir.0.join("packages");
    let library = resources
        .join("node_modules")
        .join("@typespec")
        .join("compiler");
    fs::create_dir_all(&project).unwrap();
    fs::create_dir_all(&library).unwrap();
    fs::write(project.join("main.tsp"), "namespace Example;").unwrap();
    fs::write(project.join("tspconfig.yaml"), "emit: []").unwrap();
    fs::write(project.join("ignored.rs"), "not TypeSpec").unwrap();
    fs::write(library.join("package.json"), "{}").unwrap();
    fs::write(library.join("library.tsp"), "namespace TypeSpec;").unwrap();
    fs::write(dir.0.join("resources.json"), r#"{"schema_version":1,"root":"resources/node_modules","files":["/node_modules/@typespec/compiler/package.json","/node_modules/@typespec/compiler/library.tsp"]}"#).unwrap();

    let files = collect(&project, Some(&resources), None).unwrap();
    assert_eq!(files.get("/spec/main.tsp").unwrap(), "namespace Example;");
    assert_eq!(files.get("/spec/tspconfig.yaml").unwrap(), "emit: []");
    assert_eq!(
        files
            .get("/node_modules/@typespec/compiler/package.json")
            .unwrap(),
        "{}"
    );
    assert!(!files.contains_key("/spec/ignored.rs"));
}

#[test]
fn missing_entrypoint_is_an_error() {
    let dir = TestDirectory::new();
    let resources = dir.0.join("resources");
    fs::create_dir_all(resources.join("node_modules")).unwrap();
    fs::write(
        dir.0.join("resources.json"),
        r#"{"schema_version":1,"root":"resources/node_modules","files":[]}"#,
    )
    .unwrap();
    assert!(collect(&dir.0, Some(&resources), None)
        .unwrap_err()
        .contains("main.tsp or client.tsp"));
}

#[test]
fn retains_sibling_imports_in_pinned_checkout() {
    let dir = TestDirectory::new();
    let root = dir.0.join("checkout");
    let project = root.join("specification").join("keyvault").join("Secrets");
    let common = root.join("specification").join("common-types");
    let resources = dir.0.join("resources");
    fs::create_dir_all(&project).unwrap();
    fs::create_dir_all(&common).unwrap();
    fs::create_dir_all(resources.join("node_modules")).unwrap();
    fs::write(
        project.join("main.tsp"),
        "import \"../../common-types/shared.tsp\";",
    )
    .unwrap();
    fs::write(common.join("shared.tsp"), "namespace Shared;").unwrap();
    fs::write(
        dir.0.join("resources.json"),
        r#"{"schema_version":1,"root":"resources/node_modules","files":[]}"#,
    )
    .unwrap();

    let files = collect(
        &root,
        Some(&resources),
        Some(Path::new("specification/keyvault/Secrets")),
    )
    .unwrap();
    assert!(files.contains_key("/spec/specification/keyvault/Secrets/main.tsp"));
    assert_eq!(
        files
            .get("/spec/specification/common-types/shared.tsp")
            .unwrap(),
        "namespace Shared;"
    );
}
