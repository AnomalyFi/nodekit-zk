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

//! Generated crate containing the image ID and ELF binary of the build guest.
include!(concat!(env!("OUT_DIR"), "/methods.rs"));

#[cfg(test)]
mod test {
    use risc0_zkvm::{Executor, ExecutorEnv};
    use alloy_sol_types::SolType;
   // use milagro_bls::{AggregatePublicKey};



    use crate::BLS_ELF;




    #[test]
    fn process_basic_finalization_input() {
        alloy_sol_types::sol! {
            struct WarpBlock {
                uint256 height;
                uint256 block_root;
                uint256 parent_root;
            }
    
            struct G2Point {
                bytes data;
            }
    
            struct RiscBlock {
                bytes key;
                bytes sig;
                bytes wb;
            }
        }
        
    
        //TODO I want bytes because I want this to be an object that is using the pubkeys or signatures
        //let b = Bytes::from_static(b"hello");
    
    
        // let mut pubkey_one_byte = hex::decode("053aa60e1df4b714b9ddb7eb5b99e0167aab20a3d48a7bf54410d812fb0327120aee7934da6ee11f6d4b6212a494f0890a2ec5be6645ee662cd7c012a22a1f5fe4bc64124977404bcc7e12479358b8537e9024346936297a9a7fd2b1bb9ff8d5").unwrap();
    
    
        // let pubkey_one: G2Point = G2Point {
        //     data: pubkey_one_byte,
        // };
        
        // let pubkey_two_byte: Vec<u8> = hex::decode("0b2692b1a1a1f6e3157438b91c9d413fd89af9f2acfd09524249e69d3cb69a5f07137e6c546d89295a97c81292400a3006a331e93483b318b506cea0164fd05c3b57ff7865c5be3d855daf5db1bd981754e74873a645d76bd97cab7d26ce7272").unwrap();
    
        // let pubkey_two: G2Point = G2Point {
        //     data: pubkey_two_byte,
        // };
    
        // let pubkey_three_byte: Vec<u8> = hex::decode("144d1b6e807a834ed4a98f6bb073149a9408112439e4f24ff56627f25d32ae7af0e03f11cd8e58045c011a5286bb1ca102e69be4cc34e35db98448dcef8006d52448260c94e9183b7d91443246e9c0463ad5bee66f3f54cb9bd916cbaeb231a1").unwrap();
        
        // let pubkey_three: G2Point = G2Point {
        //     data: pubkey_three_byte,
        // };
    
        // let pubkey_four_byte: Vec<u8> = hex::decode("028d10587d090e199131f42abb43693fdae73a686f9fb855c25b9f312808adb853633949febf79e42298371a2990dcc80d137bc9eaac2561e4d30d20d64c6e0cd6c5b9d7a6593927c786d48c0f4012118f24f3d24330c39a368befce4993f3fb").unwrap();
      
        // let pubkey_four: G2Point = G2Point {
        //     data: pubkey_four_byte,
        // };
    
        // let pubkey_five_byte: Vec<u8> = hex::decode("171a27b194934c328aa8712f9c07e50de58af91d6f07a81a5041bac2f87c20b5fc0d839d4fd9f0c921273ced28eb147c0f6326a64db38da22f911c9374fbc1a643985206cfc240f4af68d292a60508929325e383260d50b67260cc068bb563c8").unwrap();
        
        // let pubkey_five: G2Point = G2Point {
        //     data: pubkey_five_byte,
        // };
    
        // //TODO need to fix this
        
        // let points: Vec<G2Point> = vec![pubkey_one, pubkey_two, pubkey_three, pubkey_four, pubkey_five];

        // let mut pubkeys = vec![];

        
        // for i in 0..points.len() {
        //     let pubkey = PublicKey::from_uncompressed_bytes(&points[i].data.clone()).unwrap();
        //     pubkeys.push(pubkey);
        // }

        // let pubkeys_as_ref: Vec<&PublicKey> = pubkeys.iter().collect();
        // let agg_pub = AggregatePublicKey::aggregate(pubkeys_as_ref.as_slice()).unwrap();

        // let agg_pub_bytes = agg_pub.clone().as_bytes();
    
        let uncompressed_g1: Vec<u8> = hex::decode("19dcd680433d724d31d1cb835cb2e09e2d5f41aa2a8b07f082aedfa8f6b518e4554b61df21b7c6afa424a2b70d91d8bd0ec1467e6814ebce60472771b5ae23d7ff7428f0d9442aeae1d766e32dc2e7b6a11852b3801f7f7335ba20fb58903c3915ae486b94b054afb48a50efa541e672e7830d09385ec7593fd61816ed19025f34fe4850a0641b81994f4d2c742d92d5132120dd43a3a81cf8d26a5e14757a735bcbdb62436a0f745c57ccf1810c6f475369e01acad2b23bc4718e55fa708619").unwrap();
        
        
        let agg_pub_bytes: Vec<u8> = hex::decode("b91988515049df6da5f7eebbb37971bf3c87ebc338bb06e9c9cec2676a4cfa364746560d6912b88515cc9e96669075d7").unwrap();
        
       // let mut agg_sig = AggregateSignature::new();
       // let msg: &[u8; 14] = b"signed message";
    
       let msg: Vec<u8> = hex::decode("00000d6d28a3ce9ce2ee52031bf86d78e6b379bf8e034913e49383c47fd58bde6f7f0d6d28a3ce9ce2ee52031bf86d78e6b379bf8e034913e49383c47fd58bde6f7f000000700000000000000002000000000000000359277f9e870aa4e694d1cf312ce5a40a1a88d2c55fde2b42279f0be6ac83d54f7e15715b701983ada9335e41df9f5ad440ca67506cd8090cb764d7cd1aad569bf7086d92e3b346efdd53eb077fbfd2c9bf127f5eb8c92ea1ed82703d20c7501f").unwrap();
    
        //TODO the keys are causing issues with decoding.
       let risc_block: RiscBlock = RiscBlock {
            key: agg_pub_bytes,
            sig: uncompressed_g1,
            wb: msg
        };
    
        let abi_encoded: Vec<u8> = RiscBlock::encode(&risc_block);


        //let encoded = RiscBlock::decode(&abi_encoded.clone(), true).unwrap();

        // let hex_abi = hex::encode(abi_encoded.clone());
        // println!("Hello data {}", hex_abi);
    
    
    
    
        // //TODO I need to redo the inputs and outputs for this
        // let TEST_INPUT: &'static [u8] = &hex!(
        // "123ef2afce66c417062d3d2c69ca0a612c95de6ae9331e5e9640a361b787c1c8"
        // "000001004f81992fce2e1846dd528ec0102e6ee1f61ed3e20000000000000000000000000000000000000000000000000000"
        // "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        // "0001001bf6aa43f8d7be0bb024a5c78f3306de98255be17d70a6c6a55af54bb15a21301d41b6777aec47f9cf191533f0c351"
        // "eca97fde3756db8dd50882a26dcfa5ea0465615a2bd0468e6a715cf9378e9b28ba4314d567dd731e083c4b3d6f44e8f03bfb"
        // );
    
        // let TEST_OUTPUT: &'static [u8] = &hex!(
        // "123ef2afce66c417062d3d2c69ca0a612c95de6ae9331e5e9640a361b787c1c8"
        // "2b45288717bd1179cdda9be4ae9cb416e4e42028537046902c3a173596b4d623"
        // "000000014f81992fce2e1846dd528ec0102e6ee1f61ed3e2"
        // "00000000fbf974a059aa2e376f258d6c3238ad6c2bdb58ca"
        // );


        let env = ExecutorEnv::builder()
            .add_input(&abi_encoded)
            .build()
            .unwrap();
        let mut exec = Executor::from_elf(env, BLS_ELF).unwrap();
        let session = exec.run().unwrap();
        //assert_eq!(&session.journal, TEST_OUTPUT);
    }
}

