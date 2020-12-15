use exhaustive::c;

#[test]
fn compile_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile/01-empty-iter.rs");
    t.compile_fail("tests/compile/02-once-iter.rs");
    t.pass("tests/compile/03-for-loop.rs");
    t.pass("tests/compile/04-for-if.rs");
}

#[test]
fn test_for() {
    let i = c![n for n in 0..10];
    let values: Vec<i64> = i.collect();
    assert_eq!(values, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}


#[test]
fn test_for_if() {
    let i = c![n for n in 0..10 if n % 2 == 0];
    let values: Vec<i64> = i.collect();
    assert_eq!(values, vec![0, 2, 4, 6, 8]);
}

#[test]
fn test_for_inclusive() {
    let i = c![n for n in 0..=10];
    let values: Vec<i64> = i.collect();
    assert_eq!(values, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}


#[test]
fn test_for_if_inclusive() {
    let i = c![n for n in 0..=10 if n % 2 == 0];
    let values: Vec<i64> = i.collect();
    assert_eq!(values, vec![0, 2, 4, 6, 8, 10]);
}
