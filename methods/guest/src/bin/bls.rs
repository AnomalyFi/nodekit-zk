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

//use ethabi::{ethereum_types::U256, ParamType, Token};

use milagro_bls::*;

use risc0_zkvm::guest::env;
//use hex_literal::hex;
use alloy_sol_types::SolType;
// use alloy_primitives::{Address, U256, Bytes};
// use alloy_sol_types::{sol, SolCall, SolType, sol_data::*};
// use hex_literal::hex;

risc0_zkvm::guest::entry!(main);




fn main() {
    // Read data sent from the application contract.
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    // Type array passed to `ethabi::decode_whole` should match the types encoded in
    // the application contract.


    alloy_sol_types::sol! {
        struct WarpBlock {
            uint256 height;
            uint256 block_root;
            uint256 parent_root;
        }

        // struct G2Point {
        //     bytes data;
        // }

        struct RiscBlock {
            bytes key;
            bytes sig;
            bytes wb;
        }
    }

    //TODO the issue is related to the G2Point[] and I can look into the alloy telegram to get help
    let decoded = RiscBlock::decode(&input_bytes, true).unwrap();




    let keys: Vec<u8> = decoded.key.clone();

    let sig: Vec<u8> = decoded.sig.clone();

    let msg: Vec<u8> = decoded.wb.clone();

    //let result = aggregate_verification(&keys, &sig, &msg);

    //let mut pubkeys = vec![];

    let mut agg_sig = AggregateSignature::from_uncompressed_bytes(&sig).unwrap();

    let mut agg_pub = AggregatePublicKey::from_bytes(&keys).unwrap();


    // for i in 0..keys.len() {
    //     let pubkey = PublicKey::from_uncompressed_bytes(&keys[i].data).unwrap();
    //     pubkeys.push(pubkey);
    // }

    // let pubkeys_as_ref: Vec<&PublicKey> = pubkeys.iter().collect();
    // let agg_pub = AggregatePublicKey::aggregate(pubkeys_as_ref.as_slice()).unwrap();
    let verified = agg_sig.fast_aggregate_verify_pre_aggregated(&msg[..], &agg_pub);
    //verified




    // let input = ethabi::decode_whole(&[ParamType::Uint(256)], &input_bytes).unwrap();
    // let n: U256 = input[0].clone().into_uint().unwrap();

    //Commit the journal that will be received by the application contract.
    //Encoded types should match the args expected by the application callback.
    //env::commit_slice(&ethabi::encode(&[Token::Uint(n), Token::Uint(result)]));





    // alloy_sol_types::sol! {
    //     struct BlockData {
    //         bytes data;
    //     }
    // }
    // let msg = hex::decode("00000d6d28a3ce9ce2ee52031bf86d78e6b379bf8e034913e49383c47fd58bde6f7f0d6d28a3ce9ce2ee52031bf86d78e6b379bf8e034913e49383c47fd58bde6f7f000000700000000000000002000000000000000359277f9e870aa4e694d1cf312ce5a40a1a88d2c55fde2b42279f0be6ac83d54f7e15715b701983ada9335e41df9f5ad440ca67506cd8090cb764d7cd1aad569bf7086d92e3b346efdd53eb077fbfd2c9bf127f5eb8c92ea1ed82703d20c7501f").unwrap();

    // let block_data = BlockData { data: msg };

    // let abi_encoded = BlockData::encode(&block_data);
    // print!("{:?}", abi_encoded)

    
}
