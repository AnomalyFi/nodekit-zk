// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.17;

import {IBonsaiRelay} from "bonsai/IBonsaiRelay.sol";
import {BonsaiCallbackReceiver} from "bonsai/BonsaiCallbackReceiver.sol";

//import {BonsaiCallbackReceiver} from "./BonsaiCallbackReceiver.sol";

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
        return (_stakingKeys[index], _stakeAmounts[index]);
    }


}