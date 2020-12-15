use exhaustive::c;

fn main() {
    let _ = c![n for n in 0..10 if n % 2 == 0];
}
