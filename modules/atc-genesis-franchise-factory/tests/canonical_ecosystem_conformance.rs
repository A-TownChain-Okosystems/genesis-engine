use atc_genesis_franchise_factory::{canonical_ad_filename, validate_canonical_spec_set};

use std::{env, fs, path::PathBuf};

#[test]
fn canonical_ecosystem_specs_conform() {
    let root = env::var_os("ATC_CANONICAL_FRANCHISE_SPEC_ROOT")
        .expect("ATC_CANONICAL_FRANCHISE_SPEC_ROOT must be set for canonical external-source conformance");

    let root = PathBuf::from(root);
    let mut inputs = Vec::new();

    for ad_id in 20..=43 {
        let filename = canonical_ad_filename(ad_id).expect("canonical AD filename");
        let path = root.join(filename);
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("AD-{ad_id} canonical source missing at {}: {e}", path.display()));
        inputs.push((filename.to_owned(), source));
    }

    let validated = validate_canonical_spec_set(
        inputs.iter().map(|(filename, source)| (filename.as_str(), source.as_str())),
    )
    .expect("canonical AD-20..AD-43 set must validate");

    assert_eq!(validated.len(), 24);
    assert_eq!(validated.first().unwrap().ad_id, 20);
    assert_eq!(validated.last().unwrap().ad_id, 43);
}
