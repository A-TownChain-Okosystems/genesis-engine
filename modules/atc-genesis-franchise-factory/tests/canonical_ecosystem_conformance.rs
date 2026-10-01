use atc_genesis_franchise_factory::{canonical_ad_filename, validate_canonical_spec_set, AtcSpec};

use std::{env, fs, path::PathBuf};

#[test]
fn canonical_ecosystem_specs_conform_when_root_is_provided() {
    let Some(root) = env::var_os("ATC_CANONICAL_FRANCHISE_SPEC_ROOT") else {
        eprintln!("ATC_CANONICAL_FRANCHISE_SPEC_ROOT not set; canonical external-source conformance is skipped");
        return;
    };

    let root = PathBuf::from(root);
    let mut inputs = Vec::new();

    for ad_id in 20..=43 {
        let filename = canonical_ad_filename(ad_id).expect("canonical AD filename");
        let path = root.join(filename);
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("AD-{ad_id} canonical source missing at {}: {e}", path.display()));
        inputs.push((filename.to_owned(), source));
    }

    let specs: Vec<AtcSpec> = inputs
        .iter()
        .map(|(filename, source)| {
            atc_genesis_franchise_factory::parse_spec(filename, source)
                .unwrap_or_else(|e| panic!("canonical {filename} failed Rust conformance: {e:?}"))
        })
        .collect();

    let validated = validate_canonical_spec_set(specs.into_iter())
        .expect("canonical AD-20..AD-43 set must validate");

    assert_eq!(validated.len(), 24);
    assert_eq!(validated.first().unwrap().ad_id, 20);
    assert_eq!(validated.last().unwrap().ad_id, 43);
}
