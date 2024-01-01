use secp256k1;
use secp256k1::PublicKey;
use rand::Rng;

mod signatures;
mod payment;

mod wallets;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
}
