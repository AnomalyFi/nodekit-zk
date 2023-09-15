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
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.17;

import {BonsaiTest} from "bonsai/BonsaiTest.sol";
import {IBonsaiRelay} from "bonsai/IBonsaiRelay.sol";
import {BonsaiStarter} from "contracts/BonsaiStarter.sol";

contract BonsaiStarterTest is BonsaiTest {
    // bool useZkvmGuest;
    // bytes32 imageId;

    // function setUp() public withRelay {
    //     useZkvmGuest = vm.envOr("TEST_USE_ZKVM", true);
    //     if (useZkvmGuest) {
    //         imageId = queryImageId("FINALIZE_VOTES");
    //     }

        
    //     gov = governor(token);
    //     scene = scenario(gov, token);

    //     // Enable recording of logs so we can build the ballot list.
    //     vm.recordLogs();
    // }

    function testMockCall() public {
        // Deploy a new starter instance
        BonsaiStarter starter = new BonsaiStarter(
            IBonsaiRelay(bonsaiRelay),
            queryImageId('FIBONACCI'));

        // Anticipate a callback request to the relay
        vm.expectCall(address(bonsaiRelay), abi.encodeWithSelector(IBonsaiRelay.requestCallback.selector));
        // Request the callback
        starter.calculateFibonacci(128);

        // Anticipate a callback invocation on the starter contract
        vm.expectCall(address(starter), abi.encodeWithSelector(BonsaiStarter.storeResult.selector));
        // Relay the solution as a callback
        runPendingCallbackRequest();

        // Validate the Fibonacci solution value
        uint256 result = starter.fibonacci(128);
        assertEq(result, uint256(407305795904080553832073954));
    }
}
