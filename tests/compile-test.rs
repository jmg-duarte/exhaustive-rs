#[test]
fn test() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile/01-empty-iter.rs");
    t.compile_fail("tests/compile/02-once-iter.rs");
    t.pass("tests/compile/03-for-loop.rs");
}