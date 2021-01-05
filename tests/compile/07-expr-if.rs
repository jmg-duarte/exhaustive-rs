use exhaustive::c;

fn main() {
    let i = (0..10).map(|x| x * 2);
    let c = c![n for n in i if n % 2 == 0];
    let _: Vec<i32> = c.collect();
}
