
pub fn gcd(a: i128, b: i128) -> i128 {
    let mut d;
    let mut q = b;
    let mut r = a%b;
    while r != 0 {
        d = q;
        q = r;
        r = d%q;
    }
    q
}