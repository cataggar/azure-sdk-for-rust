// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{git_output, Location};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT_TEST: AtomicUsize = AtomicUsize::new(0);

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "generate-client-location-{}-{}",
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
fn rejects_unpinned_and_parent_paths() {
    let dir = TestDirectory::new();
    let file = dir.0.join("tsp-location.yaml");
    fs::write(
        &file,
        "repo: Azure/azure-rest-api-specs\ncommit: latest\ndirectory: specification/keyvault\n",
    )
    .unwrap();
    assert!(Location::load(&dir.0)
        .err()
        .unwrap()
        .contains("Invalid TypeSpec commit"));
    fs::write(
        &file,
        format!(
            "repo: Azure/azure-rest-api-specs\ncommit: {}\ndirectory: ../elsewhere\n",
            "a".repeat(40)
        ),
    )
    .unwrap();
    assert!(Location::load(&dir.0)
        .err()
        .unwrap()
        .contains("Invalid TypeSpec directory"));
}

#[test]
fn reads_pinned_location() {
    let dir = TestDirectory::new();
    fs::write(dir.0.join("tsp-location.yaml"), format!(
        "repo: Azure/azure-rest-api-specs\ncommit: {}\ndirectory: specification/keyvault/data-plane/Secrets\n",
        "a".repeat(40)
    )).unwrap();
    assert!(Location::load(&dir.0).unwrap().is_some());
    fs::write(
        dir.0.join("tsp-location.yaml"),
        format!(
            "repo: Azure/azure-rest-api-specs\ncommit: {}\ndirectory: specification/storage/data-plane/QueueStorage\nadditionalDirectories:\n",
            "a".repeat(40)
        ),
    )
    .unwrap();
    assert!(Location::load(&dir.0).unwrap().is_some());
}

#[test]
fn verifies_revision_and_rejects_dirty_checkout() {
    let dir = TestDirectory::new();
    let project = dir.0.join("specification").join("keyvault");
    fs::create_dir_all(&project).unwrap();
    fs::write(project.join("main.tsp"), "namespace Example;").unwrap();
    git_output(&dir.0, &["init", "-q", "."]).unwrap();
    git_output(&dir.0, &["add", "."]).unwrap();
    git_output(
        &dir.0,
        &[
            "-c",
            "user.name=SDK Test",
            "-c",
            "user.email=sdk-test@example.invalid",
            "commit",
            "-qm",
            "Initial fixture",
        ],
    )
    .unwrap();
    let commit = git_output(&dir.0, &["rev-parse", "HEAD"]).unwrap();
    fs::write(
        dir.0.join("tsp-location.yaml"),
        format!(
            "repo: Azure/azure-rest-api-specs\ncommit: {}\ndirectory: specification/keyvault\n",
            commit.trim()
        ),
    )
    .unwrap();
    let location = Location::load(&dir.0).unwrap().unwrap();
    assert!(location.verify(&project).unwrap_err().contains("untracked"));
    fs::remove_file(dir.0.join("tsp-location.yaml")).unwrap();
    location.verify(&project).unwrap();
    fs::write(project.join("main.tsp"), "namespace Changed;").unwrap();
    assert!(location
        .verify(&project)
        .unwrap_err()
        .contains("uncommitted"));
}
