fn main() {
    let mut e = 1.0;
    let mut n = 0.0;
    let mut f = 1.0;
    while e < e + f {
        n += 1.0;
        f /= n;
        println!("{} {}", n, e);
        e += f;
    }
}
