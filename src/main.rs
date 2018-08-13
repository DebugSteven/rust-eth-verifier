extern crate byteorder;
extern crate hex;
extern crate tiny_keccak;

use byteorder::{BigEndian, WriteBytesExt};
use hex::*;
use tiny_keccak::Keccak;

enum Scalar {
    U32(u32),
    U64(u64),
}

struct ETHBlock {
    parent_hash: String,
    ommers_hash: String,
    beneficiary: String,
    state_root: String,
    trans_root: String,
    receipts_root: String,
    logs_bloom: String,
    difficulty: Scalar,
    block_number: Scalar,
    gas_limit: Scalar,
    gas_used: Scalar,
    time_stamp: Scalar,
    extra_data: String,
    mix_hash: String,
    nonce: String
}

/// We are using block 6136388
/// https://www.etherchain.org/block/6136388
fn main() -> Result<(), FromHexError> {
    let mut sha3 = Keccak::new_sha3_256();

    let parent_hash: &str = "0db63e6cdea9336497f1eb3285ab41cbf03810a2fdd1f59dd738d195edf2b9fb";
    sha3.update(&decode_hex(parent_hash)?);

    let ommers_hash: &str = "1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"; 
    sha3.update(&decode_hex(ommers_hash)?);

    let beneficiary: &str = "b2930B35844a230f00E51431aCAe96Fe543a0347";
    sha3.update(&decode_hex(beneficiary)?);

    let state_root: &str = "8195a59f1929801f6480678f758b8e4dd7a48aaf2f401b9a81bf8563844d37dd";
    sha3.update(&decode_hex(state_root)?);

    let trans_root: &str = "beefe8120e58cf6479219ad6025989ce6070ee63d867fe5923caa7fd35817fbe";
    sha3.update(&decode_hex(trans_root)?);

    let receipts_root: &str = "6260f057d7a0868ce773ed747409a27012ed934f25bfb2dd38c8ac68acde9ee4";
    sha3.update(&decode_hex(receipts_root)?);

    let logs_bloom: &str = "011800010a0401000000901285002001480040800010410004030009000004ac8400848148850001101096004000040111000100209202143920002180240080802060c360040055d038000c10c1020000920220702d013800704000480511014130220402400002008020105010090000409620000401004000021d40008200001900131200068461c0040410009008400400008100400404102306011604004a005c2446504008008400801104804810046500409080620154c00800012008080c1082010c04f26048a00a00840044048ae005c20842490114402006004b008174020000984100018044218800a804802054000104902862c0450000094300";
    sha3.update(&decode_hex(logs_bloom)?);

    let difficulty: Scalar = Scalar::U64(3_472_904_411_554_402);
    sha3.update(&unsigned_to_vec(difficulty));

    let block_number: Scalar = Scalar::U64(6136388);
    sha3.update(&unsigned_to_vec(block_number));

    let gas_limit: Scalar = Scalar::U64(8_000_029);
    sha3.update(&unsigned_to_vec(gas_limit));

    let gas_used: Scalar = Scalar::U64(7_990_201);
    sha3.update(&unsigned_to_vec(gas_used));

    let time_stamp: Scalar = Scalar::U32(1_534_110_627);
    sha3.update(&unsigned_to_vec(time_stamp));

    let extra_data: &str = "73656f33";
    sha3.update(&decode_hex(extra_data)?);

    let mix_hash: &str = "1dcedb754ff2f8ffb952f84b8ee103fd651e9de273a2bea8fe9f95ccf35cfb6f";
    sha3.update(&decode_hex(mix_hash)?);

    let nonce: &str = "68d1eb905041834e";
    sha3.update(&decode_hex(nonce)?);

    let mut result: [u8; 32] = [0; 32];
    sha3.finalize(&mut result);

    println!("our hash {:?}", result);

    println!("{:#?}", hex::encode(&result));

    Ok(())
}

fn decode_hex(hex_str: &str) -> Result<Vec<u8>, FromHexError> {
   hex::decode(hex_str.as_bytes())
}

fn unsigned_to_vec(scalar: Scalar) -> Vec<u8> {
    let mut vec: Vec<u8> = vec![];
    match scalar {
        Scalar::U32(n) => vec.write_u32::<BigEndian>(n).unwrap(),
        Scalar::U64(n) => vec.write_u64::<BigEndian>(n).unwrap(), 
    };
    vec
}
