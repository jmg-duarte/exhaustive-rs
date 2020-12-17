use exhaustive::c;

#[test]
fn compile_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile/01-empty-iter.rs");
    t.compile_fail("tests/compile/02-once-iter.rs");
    t.pass("tests/compile/03-for-loop.rs");
    t.pass("tests/compile/04-for-if.rs");
    t.pass("tests/compile/05-for-iter.rs");
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

#[test]
fn test_for_iter() {
    let i = (0..10).map(|x| x * 2).map(|x| x / 2);
    let c = c![n for n in i];
    let values: Vec<i64> = c.collect();
    assert_eq!(values, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_if_expr() {
    let i = (0..10).map(|x| x * 2);
    let is_even = |n:&i32| n % 2 == 0;
    let c = c![n for n in i if is_even(n)];
    let values: Vec<i32> = c.collect();
    assert_eq!(values, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
}
