use secp256k1;
use secp256k1::PublicKey;
use rand::Rng;
use anyhow::Result;
//mod signatures;
//mod payment;

mod wallets;

const URL: &str = "https://eth-ropsten.alchemyapi.io/v2/owrGyNISGOXRtlnncV2XGYZ4a6DvdYi_";

fn main() -> Result<()> {
    let keypair =wallets::create_keypair();
    println!("keypair: {:?}", keypair);
    Ok(())
}
