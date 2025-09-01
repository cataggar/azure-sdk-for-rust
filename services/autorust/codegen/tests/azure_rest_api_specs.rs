// cargo test --test azure_rest_api_specs
// These tests require cloning azure-rest-api-specs.
// git clone git@github.com:Azure/azure-rest-api-specs.git ../azure-rest-api-specs

use autorust_codegen::*;
use autorust_openapi::Reference;
use camino::Utf8PathBuf;
use spec::TypedReference;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn specs_root() -> Utf8PathBuf {
    fn canonicalize_utf8(p: Utf8PathBuf) -> Utf8PathBuf {
        match std::fs::canonicalize(&p) {
            Ok(abs) => Utf8PathBuf::from_path_buf(abs).unwrap_or(p),
            Err(_) => p,
        }
    }
    if let Ok(root) = std::env::var("AZURE_REST_API_SPECS") {
        return canonicalize_utf8(Utf8PathBuf::from(root));
    }
    // Fallback to the historical relative location from this crate
    let joined = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../azure-rest-api-specs");
    canonicalize_utf8(joined)
}

fn common_types_spec() -> Utf8PathBuf {
    specs_root().join("specification/security/resource-manager/common/v1/types.json")
}

fn vmware_spec() -> Utf8PathBuf {
    specs_root().join("specification/vmware/resource-manager/Microsoft.AVS/AVS/stable/2020-03-20/vmware.json")
}

#[test]
fn refs_count_security_common() -> Result<()> {
    let doc_file = common_types_spec();
    let api = &spec::openapi::parse(&doc_file)?;
    let refs = spec::openapi::get_references(&doc_file, api);
    assert_eq!(15, refs.len());
    Ok(())
}

#[test]
fn refs_count_avs() -> Result<()> {
    let doc_file = vmware_spec();
    let api = &spec::openapi::parse(&doc_file)?;
    let refs = spec::openapi::get_references(&doc_file, api);
    assert_eq!(199, refs.len());
    Ok(())
}

#[test]
fn ref_files() -> Result<()> {
    let doc_file = vmware_spec();
    let api = &spec::openapi::parse(&doc_file)?;
    let files = spec::openapi::get_reference_file_paths(&doc_file, api);
    assert_eq!(1, files.len());
    // Join the discovered reference with the doc file dir and compare to the canonical common-types path
    let joined: Vec<Utf8PathBuf> = files
        .iter()
        .map(|f| autorust_codegen::io::join(&doc_file, f).expect("join should succeed"))
        .collect();
    let expected = specs_root().join("specification/common-types/resource-management/v1/types.json");
    assert!(
        joined.iter().any(|p| p == &expected),
        "expected reference to {} not found in {:?}",
        expected,
        joined
    );
    Ok(())
}

#[test]
fn read_spec_avs() -> Result<()> {
    let vmware = vmware_spec();
    let spec = &Spec::read_files(&[&vmware])?;
    assert_eq!(2, spec.docs().len());
    let common = specs_root().join("specification/common-types/resource-management/v1/types.json");
    assert!(spec.docs().contains_key(common.as_path()));
    Ok(())
}

#[test]
fn test_resolve_schema_ref() -> Result<()> {
    let file = vmware_spec();
    let spec = &Spec::read_files(&[&file])?;
    spec.resolve_schema_ref(&file, &Reference::parse("#/definitions/OperationList").unwrap())?;
    // Resolve a schema from the common-types file using its absolute path to avoid brittle ../../ joins
    let common = specs_root().join("specification/common-types/resource-management/v1/types.json");
    spec.resolve_schema_ref(&common, &Reference::parse("#/definitions/ErrorResponse").unwrap())?;
    Ok(())
}

#[test]
fn test_resolve_parameter_ref() -> Result<()> {
    let file = vmware_spec();
    let spec = &Spec::read_files(&[&file])?;
    // Resolve a parameter from the common-types file using its absolute path
    let common = specs_root().join("specification/common-types/resource-management/v1/types.json");
    spec.resolve_parameter_ref(&common, Reference::parse("#/parameters/ApiVersionParameter").unwrap())?;
    Ok(())
}

#[test]
fn test_resolve_all_refs() -> Result<()> {
    let doc_file = vmware_spec();
    let spec = &Spec::read_files(&[&doc_file])?;
    for (doc_file, api) in spec.docs() {
        let refs = spec::openapi::get_references(doc_file, api);
        for rs in refs {
            match rs {
                TypedReference::PathItem(_) => {}
                TypedReference::Example(_) => {}
                TypedReference::Parameter(reference) => {
                    spec.resolve_parameter_ref(doc_file, reference)?;
                }
                TypedReference::Schema(reference) => {
                    spec.resolve_schema_ref(doc_file, &reference)?;
                }
            }
        }
    }
    Ok(())
}
