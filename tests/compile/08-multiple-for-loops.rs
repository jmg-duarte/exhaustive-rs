use exhaustive::c;

fn main() {
    let _ = c![(n, m) for n in 0..10 for m in 0..=5];
}
