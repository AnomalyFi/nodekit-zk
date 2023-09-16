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

use anyhow::Context;
use bonsai_ethereum_relay::sdk::client::{CallbackRequest, Client};
use clap::Parser;
use ethers::{abi::ethabi, types::Address};
use alloy_sol_types::SolType;
use methods::BLS_ID;
use risc0_zkvm::sha::Digest;

/// Exmaple code for sending a REST API request to the Bonsai relay service to
/// requests, execution, proving, and on-chain callback for a zkVM guest
/// application.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// Adress for the Sequencer application contract.
    address: Address,

    key: Vec<u8>,
    
    sig: Vec<u8>,
    
    wb: Vec<u8>,


    /// Bonsai Relay API URL.
    #[arg(long, env, default_value = "http://localhost:8080")]
    bonsai_relay_api_url: String,

    /// Bonsai API key. Used by the relay to send requests to the Bonsai proving
    /// service. Defaults to empty, providing no authentication.
    #[arg(long, env, default_value = "")]
    bonsai_api_key: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    // initialize a relay client
    let relay_client = Client::from_parts(
        args.bonsai_relay_api_url.clone(), // Set BONSAI_API_URL or replace this line.
        args.bonsai_api_key.clone(),       // Set BONSAI_API_KEY or replace this line.
    )
    .context("Failed to initialize the relay client")?;

    // Initialize the input for the FIBONACCI guest.
    //let input = ethabi::encode(&[ethers::abi::Token::Uint(args.number.into())]);

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

    let risc_block: RiscBlock = RiscBlock {
        key: args.key.into(),
        sig: args.sig.into(),
        wb: args.wb.into()
    };

    let input: Vec<u8> = RiscBlock::encode(&risc_block);

    let selector: Vec<u8> = hex::decode("32c8da6f").unwrap();


    let mut array = [0; 4]; // Initialize a [u8; 4] array with all zeros
    array.copy_from_slice(&selector[..4]); // Copy the first four bytes from the vector
    

    // Create a CallbackRequest for your contract
    // example: (contracts/BonsaiStarter.sol).
    let request = CallbackRequest {
        callback_contract: args.address,
        // you can use the command `solc --hashes contracts/BonsaiStarter.sol`
        // to get the value for your actual contract (9f2275c0: storeResult(uint256,uint256))
        function_selector: array,
        gas_limit: 3000000,
        image_id: Digest::from(BLS_ID).into(),
        input,
    };

    // Send the callback request to the Bonsai Relay.
    relay_client
        .callback_request(request)
        .await
        .context("Callback request failed")?;

    Ok(())
}
