// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use std::io::Read;

use ethabi::{ethereum_types::U256, ParamType, Token};

use milagro_bls::*;

use risc0_zkvm::guest::env;
use alloy_primitives::{Address, U256};
use alloy_sol_types::{sol, SolCall, SolType, sol_data::*};
use hex_literal::hex;

risc0_zkvm::guest::entry!(main);



sol! {
    struct WarpBlock {
        uint256 height;
        uint256 blockRoot;
        uint256 parentRoot;
    }
}

sol! {
    struct G2Point {
        bytes data;
    }
}

sol! {
    struct RiscBlock {
        //TODO 
        G2Point[] keys;
        bytes sig;
        bytes wb;
    }
}


fn fibonacci(n: U256) -> U256 {
    let (mut prev, mut curr) = (U256::one(), U256::one());
    for _ in 2..=n.as_u32() {
        (prev, curr) = (curr, prev + curr);
    }
    curr
}


fn aggregate_verfication() {
    //let n = 128;

    let mut pubkeys = vec![];


    let uncompressed_g1 = hex::decode("19dcd680433d724d31d1cb835cb2e09e2d5f41aa2a8b07f082aedfa8f6b518e4554b61df21b7c6afa424a2b70d91d8bd0ec1467e6814ebce60472771b5ae23d7ff7428f0d9442aeae1d766e32dc2e7b6a11852b3801f7f7335ba20fb58903c3915ae486b94b054afb48a50efa541e672e7830d09385ec7593fd61816ed19025f34fe4850a0641b81994f4d2c742d92d5132120dd43a3a81cf8d26a5e14757a735bcbdb62436a0f745c57ccf1810c6f475369e01acad2b23bc4718e55fa708619").unwrap();
    let mut agg_sig = AggregateSignature::from_uncompressed_bytes(&uncompressed_g1).unwrap();
    
    
    
   // let mut agg_sig = AggregateSignature::new();
   // let msg: &[u8; 14] = b"signed message";

    let msg = hex::decode("00000d6d28a3ce9ce2ee52031bf86d78e6b379bf8e034913e49383c47fd58bde6f7f0d6d28a3ce9ce2ee52031bf86d78e6b379bf8e034913e49383c47fd58bde6f7f000000700000000000000002000000000000000359277f9e870aa4e694d1cf312ce5a40a1a88d2c55fde2b42279f0be6ac83d54f7e15715b701983ada9335e41df9f5ad440ca67506cd8090cb764d7cd1aad569bf7086d92e3b346efdd53eb077fbfd2c9bf127f5eb8c92ea1ed82703d20c7501f").unwrap();


    let pubkey_one_byte = hex::decode("053aa60e1df4b714b9ddb7eb5b99e0167aab20a3d48a7bf54410d812fb0327120aee7934da6ee11f6d4b6212a494f0890a2ec5be6645ee662cd7c012a22a1f5fe4bc64124977404bcc7e12479358b8537e9024346936297a9a7fd2b1bb9ff8d5").unwrap();
    let mut pubkey_one = PublicKey::from_uncompressed_bytes(&pubkey_one_byte).unwrap();
    pubkeys.push(pubkey_one);

    
    let pubkey_two_byte = hex::decode("0b2692b1a1a1f6e3157438b91c9d413fd89af9f2acfd09524249e69d3cb69a5f07137e6c546d89295a97c81292400a3006a331e93483b318b506cea0164fd05c3b57ff7865c5be3d855daf5db1bd981754e74873a645d76bd97cab7d26ce7272").unwrap();
    let mut pubkey_two = PublicKey::from_uncompressed_bytes(&pubkey_two_byte).unwrap();
    pubkeys.push(pubkey_two);

    let pubkey_three_byte = hex::decode("144d1b6e807a834ed4a98f6bb073149a9408112439e4f24ff56627f25d32ae7af0e03f11cd8e58045c011a5286bb1ca102e69be4cc34e35db98448dcef8006d52448260c94e9183b7d91443246e9c0463ad5bee66f3f54cb9bd916cbaeb231a1").unwrap();
    let mut pubkey_three = PublicKey::from_uncompressed_bytes(&pubkey_three_byte).unwrap();
    pubkeys.push(pubkey_three);


    let pubkey_four_byte = hex::decode("028d10587d090e199131f42abb43693fdae73a686f9fb855c25b9f312808adb853633949febf79e42298371a2990dcc80d137bc9eaac2561e4d30d20d64c6e0cd6c5b9d7a6593927c786d48c0f4012118f24f3d24330c39a368befce4993f3fb").unwrap();
    let mut pubkey_four = PublicKey::from_uncompressed_bytes(&pubkey_four_byte).unwrap();
    pubkeys.push(pubkey_four);

    
    let pubkey_five_byte = hex::decode("171a27b194934c328aa8712f9c07e50de58af91d6f07a81a5041bac2f87c20b5fc0d839d4fd9f0c921273ced28eb147c0f6326a64db38da22f911c9374fbc1a643985206cfc240f4af68d292a60508929325e383260d50b67260cc068bb563c8").unwrap();
    let mut pubkey_five = PublicKey::from_uncompressed_bytes(&pubkey_five_byte).unwrap();
    pubkeys.push(pubkey_five);
    

    let pubkeys_as_ref: Vec<&PublicKey> = pubkeys.iter().collect();
    let agg_pub = AggregatePublicKey::aggregate(pubkeys_as_ref.as_slice()).unwrap();
    let verified = agg_sig.fast_aggregate_verify_pre_aggregated(&msg[..], &agg_pub);
    assert!(verified);  
}

fn main() {
    // Read data sent from the application contract.
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    // Type array passed to `ethabi::decode_whole` should match the types encoded in
    // the application contract.


    //TODO this first object needs to be able unwrap the data into a bytes[] of keys, 

    let data = hex!(
        "a9059cbb"
        "0000000000000000000000008bc47be1e3abbaba182069c89d08a61fa6c2b292"
        "0000000000000000000000000000000000000000000000000000000253c51700"
    );
    let decoded = WarpBlock::decode(&data, true).unwrap();

    let input = ethabi::decode_whole(&[ParamType::Uint(256)], &input_bytes).unwrap();
    let n: U256 = input[0].clone().into_uint().unwrap();

    // Run the computation.
    let result = fibonacci(n);

    aggregate_verfication();

    // Commit the journal that will be received by the application contract.
    // Encoded types should match the args expected by the application callback.
    env::commit_slice(&ethabi::encode(&[Token::Uint(n), Token::Uint(result)]));
}
