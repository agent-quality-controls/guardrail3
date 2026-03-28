use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_10_use_count_error::assert_no_hits;
use test_support::write_file;

#[test]
fn skips_grouped_imports_that_keep_statement_count_low() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let content = test_support::read_file(root, rel);

    write_file(
        root,
        rel,
        &format!("use crate::{{a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u}};\n{content}"),
    );

    let results = run_family(root);
    assert_no_hits(&results);
}
