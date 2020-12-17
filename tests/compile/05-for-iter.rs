use exhaustive::c;

fn main() {
    let i = (0..10).map(|x| x*2);
    let _ = c![n for n in i];
}
