use exhaustive::c;

fn main() {
    let i = (0..10).map(|x| x * 2);
    let is_even = |n:&i32| n % 2 == 0;
    let c = c![n for n in i if is_even(n)];
    let _: Vec<i32> = c.collect();
}
