// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{git_output, Location};
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

fn local_repository() -> (TestDirectory, Location, PathBuf, PathBuf) {
    let dir = TestDirectory::new();
    let repo = dir.0.join("repository");
    let cache = dir.0.join("cache");
    let project = repo
        .join("specification")
        .join("keyvault")
        .join("data-plane")
        .join("Secrets");
    let shared = repo.join("specification").join("common-types");
    fs::create_dir_all(&project).unwrap();
    fs::create_dir_all(&shared).unwrap();
    fs::create_dir_all(repo.join("specification").join("unrelated")).unwrap();
    fs::write(repo.join("package.json"), "{}").unwrap();
    fs::write(
        repo.join("specification").join("tspconfig.yaml"),
        "emit: []",
    )
    .unwrap();
    fs::write(
        project.join("main.tsp"),
        "import \"../../../common-types/shared.tsp\";",
    )
    .unwrap();
    fs::write(shared.join("shared.tsp"), "namespace Shared;").unwrap();
    fs::write(
        repo.join("specification")
            .join("unrelated")
            .join("large.tsp"),
        "namespace Unrelated;",
    )
    .unwrap();
    git_output(&repo, &["init", "-q", "."]).unwrap();
    git_output(&repo, &["add", "."]).unwrap();
    git_output(
        &repo,
        &[
            "-c",
            "user.name=SDK Test",
            "-c",
            "user.email=sdk-test@example.invalid",
            "commit",
            "-qm",
            "Pinned fixture",
        ],
    )
    .unwrap();
    let commit = git_output(&repo, &["rev-parse", "HEAD"]).unwrap();
    let location = Location {
        directory: PathBuf::from("specification/keyvault/data-plane/Secrets"),
        commit: commit.trim().to_string(),
        repo: "Azure/azure-rest-api-specs".to_string(),
        additional_directories: Some(vec![PathBuf::from("specification/common-types")]),
    };
    (dir, location, repo, cache)
}

fn assert_sparse_checkout(cache: &Path, location: &Location) {
    location.verify(&cache.join(&location.directory)).unwrap();
    assert!(cache.join("package.json").is_file());
    assert!(cache.join("specification").join("tspconfig.yaml").is_file());
    assert!(cache
        .join("specification")
        .join("common-types")
        .join("shared.tsp")
        .is_file());
    assert!(!cache.join("specification").join("unrelated").exists());
    assert_eq!(
        git_output(cache, &["sparse-checkout", "list"])
            .unwrap()
            .lines()
            .collect::<Vec<_>>(),
        [
            "specification/common-types",
            "specification/keyvault/data-plane/Secrets"
        ]
    );
}

#[test]
fn fresh_sync_is_sparse_and_repeatable() {
    let (_dir, location, repo, cache) = local_repository();
    let url = repo.to_str().unwrap();
    assert_eq!(
        location.sync_from(&cache, url).unwrap(),
        cache.join(&location.directory)
    );
    assert_sparse_checkout(&cache, &location);
    location.sync_from(&cache, url).unwrap();
    assert_sparse_checkout(&cache, &location);
}

#[test]
fn existing_full_checkout_becomes_sparse() {
    let (_dir, location, repo, cache) = local_repository();
    fs::create_dir(&cache).unwrap();
    git_output(&cache, &["init", "-q", "."]).unwrap();
    git_output(
        &cache,
        &[
            "fetch",
            "--depth=1",
            repo.to_str().unwrap(),
            &location.commit,
        ],
    )
    .unwrap();
    git_output(&cache, &["checkout", "--detach", "FETCH_HEAD"]).unwrap();
    assert!(cache.join("specification").join("unrelated").exists());
    location.sync_from(&cache, repo.to_str().unwrap()).unwrap();
    assert_sparse_checkout(&cache, &location);
}

#[test]
fn existing_checkout_moves_to_exact_pinned_commit() {
    let (_dir, mut location, repo, cache) = local_repository();
    location.sync_from(&cache, repo.to_str().unwrap()).unwrap();
    fs::write(
        repo.join("specification")
            .join("keyvault")
            .join("data-plane")
            .join("Secrets")
            .join("main.tsp"),
        "namespace Updated;",
    )
    .unwrap();
    git_output(&repo, &["add", "."]).unwrap();
    git_output(
        &repo,
        &[
            "-c",
            "user.name=SDK Test",
            "-c",
            "user.email=sdk-test@example.invalid",
            "commit",
            "-qm",
            "Updated fixture",
        ],
    )
    .unwrap();
    location.commit = git_output(&repo, &["rev-parse", "HEAD"])
        .unwrap()
        .trim()
        .to_string();
    location.sync_from(&cache, repo.to_str().unwrap()).unwrap();
    assert_sparse_checkout(&cache, &location);
    assert_eq!(
        fs::read_to_string(cache.join(&location.directory).join("main.tsp")).unwrap(),
        "namespace Updated;"
    );
}

#[test]
fn dirty_cached_checkout_is_not_changed() {
    let (_dir, location, repo, cache) = local_repository();
    location.sync_from(&cache, repo.to_str().unwrap()).unwrap();
    fs::write(cache.join("package.json"), "{\"changed\":true}").unwrap();
    assert!(location
        .sync_from(&cache, repo.to_str().unwrap())
        .unwrap_err()
        .contains("uncommitted"));
    assert_eq!(
        fs::read_to_string(cache.join("package.json")).unwrap(),
        "{\"changed\":true}"
    );
}

#[test]
fn absent_additional_directory_fails_explicitly() {
    let (_dir, mut location, repo, cache) = local_repository();
    location.additional_directories = Some(vec![PathBuf::from("specification/missing")]);
    assert!(location
        .sync_from(&cache, repo.to_str().unwrap())
        .unwrap_err()
        .contains("Missing pinned additional TypeSpec directory"));
}
