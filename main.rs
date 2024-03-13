extern crate rand;
extern crate num;

use std::env;
use rand::Rng;
use num::integer::gcd;

struct RSA {
    e: u64,
    d: u64,
    n: u64,
    phi: u64,
}

impl RSA {
    fn new() -> RSA {
        let (n, phi) = (73109369, 73092096);
        println!("RSA.N: {:?}", n);
        println!("RSA.Phi: {:?}", phi);
        let e = 10273;
        let mut d = 632713;
        while (d * e) % phi != 1 {
            d += 1;
        }

        RSA { e, d, n, phi }
    }

    fn encrypt(&self, message: String) -> Vec<u64> {
        let message_int = string_to_vec(message);
        let mut encrypted: Vec<u64> = Vec::new();
        for m in message_int {
            let c = mod_pow(m, self.e, self.n);
            encrypted.push(c);
        }
        encrypted
    }

    fn decrypt(&self, encrypted: Vec<u64>) -> String {
        let mut decrypted: Vec<u64> = Vec::new();
        for c in encrypted {
            let m = mod_pow(c, self.d, self.n);
            decrypted.push(m);
        }
        vec_to_string(decrypted)
    }
}

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


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} --encrypt(-e) or --decrypt(-d) <message>", args[0]);
        return;
    }

    let rsa = RSA::new();
    match args[1].as_str() {
        "--encrypt" | "-e" => {
            let message = &args[2];
            let encrypted = rsa.encrypt(message.to_string());
                        
            print!("Encrypted: ");
            for e in &encrypted {
                print!("{},", e);
            }
            println!();

        },
        "--decrypt" | "-d" => {
            let encrypted: Vec<u64> = args[2].split(',').map(|x| x.parse().unwrap()).collect();
            println!("Encrypted: {:?}", encrypted);
            println!("RSA.N: {:?}", rsa.n);
            let decrypted = rsa.decrypt(encrypted);
            println!("Decrypted: {}", decrypted);
        },
        _ => println!("Usage: {} --encrypt(-e) or --decrypt(-d) <message>", args[0]),
    }
}
