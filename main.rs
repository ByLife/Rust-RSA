extern crate rand;
use rand::Rng;


fn StringToVec(s: String) -> Vec<u16> {
    let mut v: Vec<u16> = Vec::new();
    for c in s.chars() {
        v.push(c as u16);
    }
    v
}

fn is_prime(n: u64) -> bool {
    if n % 2 == 0 || n % 3 == 0 {
        false
    }
    true
}
fn random_prime() -> u64 {
    let mut rng = rand::thread_rng();
    let mut n = rng.gen_range(1000, 10000);
    while !is_prime(n) {
        n = rng.gen_range(1000, 10000);
    }
    n
}




fn main() {
    let message = "Hello, world! é lé méc";
    let message_int = StringToVec(message.to_string());
    println!("{:?}", message_int);

    let p = random_prime();
    let q = random_prime();

    let n = p * q;

    let phi = (p - 1) * (q - 1);

    println!("p: {}, q: {}, n: {}, phi: {}", p, q, n, phi);
}
