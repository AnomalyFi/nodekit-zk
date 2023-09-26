// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.17;

contract Sequencer {

    mapping(uint256 blockHeight => uint256 commitment) public commitments;
    uint256 public blockHeight;

    event NewBlock(uint256 blockNumber);

    error TooManyBlocks(uint256 numBlocks);
    error IncorrectBlockNumber(uint256 blockNumber, uint256 expectedBlockNumber);
    error NoKeySelected();
    error NotEnoughStake();

    struct WarpBlock {
        uint256 height;
        uint256 block_root;
        uint256 parent_root;
    }

    /// @notice Function to add block. Future version will need verification of BLS signatures and verification of parent root.
    function addBlock(WarpBlock calldata warp) external {
        uint256 firstBlockNumber = blockHeight;
        if (warp.height != blockHeight) {
            revert IncorrectBlockNumber(warp.height, blockHeight);
        }
        commitments[blockHeight] = warp.block_root;
        blockHeight += 1;
        emit NewBlock(firstBlockNumber);
    }

}