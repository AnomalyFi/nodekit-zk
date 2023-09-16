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
    
            // struct G2Point {
            //     bytes data;
            // }
    
            struct RiscBlock {
                bytes key;
                bytes sig;
                bytes wb;
            }
        }
        

    
    

    
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



        let env = ExecutorEnv::builder()
            .add_input(&abi_encoded)
            .build()
            .unwrap();
        let mut exec = Executor::from_elf(env, BLS_ELF).unwrap();
        let session = exec.run().unwrap();
        //assert_eq!(&session.journal, TEST_OUTPUT);
    }
}

