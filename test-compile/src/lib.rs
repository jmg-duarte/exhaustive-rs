#[test]
fn test() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/tests/01-empty-iter.rs");
    // t.pass("tests/02-once-iter.rs");
}