use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_grouped_imports_that_keep_statement_count_low() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let content = std::fs::read_to_string(root.join(rel)).expect("read source");

    write_file(
        root,
        rel,
        &format!("use crate::{{a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u}};\n{content}"),
    );

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-10"), BTreeSet::new());
}
