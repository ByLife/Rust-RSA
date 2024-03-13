extern crate rand;
extern crate num;

use rand::Rng;
use num::integer::gcd;


fn string_to_vec(s: String) -> Vec<u64> {
    let mut v: Vec<u64> = Vec::new();
    for c in s.chars() {
        v.push(c as u64);
    }
    v
}

fn vec_to_string(v: Vec<u64>) -> String {
    let mut s = String::new();
    for c in v {
        s.push(c as u8 as char);
    }
    s
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn random_prime() -> u64 {
    let mut rng = rand::thread_rng();
    let mut n = rng.gen_range(1000..10000);
    while !is_prime(n) {
        n = rng.gen_range(1000..10000);
    }
    n
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
           result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}
// return phi and n

fn n() -> (u64, u64){
    let p = random_prime();
    let q = random_prime();
    let n = p * q;
    let phi = (p - 1) * (q - 1);
    (n, phi)
}



fn main() {
    let message = "Hello, world! é lé méc";
    let message_int = string_to_vec(message.to_string());
    println!("Message déchiffré: {:?}", message);

    let n_res = n();

    let n = n_res.0;
    let phi = n_res.1;

    let mut e = random_prime();
    while e < phi {
        if gcd(e as u64, phi as u64) == 1 {
            break;
        }
        e += 1;
    }

    let mut d = random_prime();

    while d < phi {
        if (d * e) % phi == 1 {
            break;
        }
        d += 1;
    }

    let mut encrypted: Vec<u64> = Vec::new();

    for m in message_int {
        let c = mod_pow(m, e as u64, n);
        encrypted.push(c);
    }

    println!("{:?}", encrypted);

    let mut decrypted: Vec<u64> = Vec::new();

    for c in encrypted {
        let m = mod_pow(c, d as u64, n);
        decrypted.push(m);
    }

    println!("{:?}", decrypted);

    let decrypted_message = vec_to_string(decrypted);
    println!("{}", decrypted_message);

}
