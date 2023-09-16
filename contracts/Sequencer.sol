// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.17;

import {IBonsaiRelay} from "./IBonsaiRelay.sol";
import {BonsaiCallbackReceiver} from "./BonsaiCallbackReceiver.sol";

contract Sequencer is BonsaiCallbackReceiver {
    event NewStakingKey(G2Point stakingKey, uint256 amount, uint256 index);

    mapping(uint256 blockHeight => uint256 commitment) public commitments;
    uint256 public blockHeight;

    // Mapping for Avalanche P-Chain Validators
    mapping(uint256 index => uint256 amount) private _stakeAmounts;
    G2Point[] private _stakingKeys;

    event NewBlock(uint256 blockNumber);

    error TooManyBlocks(uint256 numBlocks);
    error IncorrectBlockNumber(uint256 blockNumber, uint256 expectedBlockNumber);
    error NoKeySelected();
    error NotEnoughStake();

     /// @notice Image ID of the only zkVM binary to accept callbacks from.
    bytes32 public immutable blsImageId;

    /// @notice Initialize the contract, binding it to a specified Bonsai relay and RISC Zero guest image.
    constructor(IBonsaiRelay bonsaiRelay, bytes32 _blsImageId) BonsaiCallbackReceiver(bonsaiRelay) {
        blsImageId = _blsImageId;
    }

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

    /// @notice Callback function logic for processing verified journals from Bonsai.
    function storeResult(WarpBlock calldata warp) external onlyBonsaiCallback(blsImageId) {
        uint256 firstBlockNumber = blockHeight;
        if (warp.height != blockHeight) {
            revert IncorrectBlockNumber(warp.height, blockHeight);
        }

        commitments[blockHeight] = warp.block_root;
        blockHeight += 1;
        

        emit NewBlock(firstBlockNumber);
    }

    // function addBlock(
    //     bytes memory message,
    //     bytes memory sig,
    //     bool[] memory bitmap,
    //     uint256 minStakeThreshold
    // ) external {
    //     require(bitmap.length <= _stakingKeys.length, "bitmap is too long");

    //     // Build aggregated public key
    //     uint256 index = 0;
    //     while (!bitmap[index] && index < bitmap.length) {
    //         index++;
    //     }

    //     if (index >= bitmap.length) {
    //         revert NoKeySelected();
    //     }

    //     // Compute the stake corresponding to the signers and check if it is enough
    //     uint256 stake = 0;
    //     for (uint256 i = index; i < bitmap.length; i++) {
    //         if (bitmap[i]) {
    //             stake += _stakeAmounts[i]; 
    //         }
    //     }

    //     if (stake < minStakeThreshold) {
    //         revert NotEnoughStake();
    //     }

    //     //TODO fix the keys because bytes[] is bytes
    //     G2Point[] memory keys = new G2Point[](bitmap.length);


    //     for (uint256 i = index + 1; i < bitmap.length; i++) {
    //         if (bitmap[i]) {
    //             keys[i] = _stakingKeys[i];
    //         }
    //     }

    //     RiscBlock memory rb = RiscBlock(keys, sig, message);

    //     bonsaiRelay.requestCallback(
    //         blsImageId, abi.encode(rb), address(this), this.storeResult.selector, 100000
    //     );
    // }


    function addBlockDemo(
        RiscBlock calldata risc
    ) external {
        bonsaiRelay.requestCallback(
            blsImageId, abi.encode(risc), address(this), this.storeResult.selector, 10000000
        );
    }

    /// @dev Avalanche Validator Staking
    /// @notice Only for testing. Needs a way to verify on P-Chain for real version
    /// @param stakingKey public key for the BLS scheme
    /// @param amount stake corresponding to the staking key
    function addNewStakingKey(G2Point memory stakingKey, uint256 amount) public {
        //TODO maybe change the key to be a bytes array
        uint256 index = _stakingKeys.length;
        _stakeAmounts[index] = amount;
        _stakingKeys.push(stakingKey);
        emit NewStakingKey(stakingKey, amount, index);
    }

    function getStakingKey(uint256 index) public view returns (G2Point memory, uint256) {
        //TODO maybe change the key to be a bytes array
        return (_stakingKeys[index], _stakeAmounts[index]);
    }


}