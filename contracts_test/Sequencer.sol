// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.17;

contract Sequencer {

    mapping(uint256 blockHeight => uint256 commitment) public commitments;
    uint256 public blockHeight;
    uint256 public constant MAX_BLOCKS = 500;

    event NewBlock(uint256 blockNumber);
    event NewBlocks(uint256 firstBlockNumber, uint256 numBlocks);


    error TooManyBlocks(uint256 numBlocks);
    error IncorrectBlockNumber(uint256 blockNumber, uint256 expectedBlockNumber);
    error NoKeySelected();
    error NotEnoughStake();

    struct WarpBlock {
        uint256 height;
        uint256 blockRoot;
        uint256 parentRoot;
    }

    /// @notice Function to add block. Future version will need verification of BLS 
    function addBlock(WarpBlock calldata warp) external {
        uint256 firstBlockNumber = blockHeight;
        if (warp.height != blockHeight) {
            revert IncorrectBlockNumber(warp.height, blockHeight);
        }
        commitments[blockHeight] = warp.blockRoot;
        blockHeight += 1;
        emit NewBlock(firstBlockNumber);
    }

    function newBlocks(WarpBlock[] calldata blocks) external {
        if (blocks.length > MAX_BLOCKS) {
            revert TooManyBlocks(blocks.length);
        }

        uint256 firstBlockNumber = blockHeight;
        for (uint256 i = 0; i < blocks.length; ++i) {
            if (blocks[i].height != blockHeight) {
                // Fail quickly if this blocks is for the wrong block. In particular, this saves the
                // caller some gas in the race condition where two clients try to sequence the same
                // block at the same time, and the first one wins.
                revert IncorrectBlockNumber(blocks[i].height, blockHeight);
            }

            commitments[blockHeight] = blocks[i].blockRoot;
            blockHeight += 1;
        }

        emit NewBlocks(firstBlockNumber, blocks.length);
    }

}