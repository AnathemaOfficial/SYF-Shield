#[test]
fn compile_fail_sealed_path_absence() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/sealed_cannot_engage.rs");
}

#[test]
fn compile_fail_capacity_not_writable() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/capacity_not_writable.rs");
}
