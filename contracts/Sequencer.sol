// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.17;

import {IBonsaiRelay} from "bonsai/IBonsaiRelay.sol";
import {BonsaiCallbackReceiver} from "bonsai/BonsaiCallbackReceiver.sol";

contract Sequencer is BonsaiCallbackReceiver {
    event NewStakingKey(G2Point stakingKey, uint256 amount, uint256 index);

    mapping(uint256 blockHeight => uint256 commitment) public commitments;
    uint256 public blockHeight;

    // Stake table related data structures
    mapping(uint256 index => uint256 amount) private _stakeAmounts;
    G2Point[] private _stakingKeys;

    event NewBlocks(uint256 firstBlockNumber, uint256 numBlocks);

    error TooManyBlocks(uint256 numBlocks);
    error InvalidQC(uint256 blockNumber);
    error IncorrectBlockNumber(uint256 blockNumber, uint256 expectedBlockNumber);
    error NoKeySelected();
    error NotEnoughStake();

     /// @notice Image ID of the only zkVM binary to accept callbacks from.
    bytes32 public immutable blsImageId;

    /// @notice Gas limit set on the callback from Bonsai.
    /// @dev Should be set to the maximum amount of gas your callback might reasonably consume.
    uint64 private constant BONSAI_CALLBACK_GAS_LIMIT = 100000;

    /// @notice Initialize the contract, binding it to a specified Bonsai relay and RISC Zero guest image.
    constructor(IBonsaiRelay bonsaiRelay, bytes32 _blsImageId) BonsaiCallbackReceiver(bonsaiRelay) {
        blsImageId = _blsImageId;
    }

    struct WarpBlock {
        uint256 height;
        uint256 blockRoot;
        uint256 parentRoot;
    }


    struct G1Point {
        uint256 x;
        uint256 y;
    }

    // G2 group element where x \in Fp2 = x0 * z + x1
    struct G2Point {
        uint256 x0;
        uint256 x1;
        uint256 y0;
        uint256 y1;
    }


    /// @notice Callback function logic for processing verified journals from Bonsai.
    function storeResult(uint256 height, WarpBlock calldata warp) external onlyBonsaiCallback(fibImageId) {
        uint256 firstBlockNumber = blockHeight;
        if (warp.height != blockHeight) {
            // Fail quickly if this QC is for the wrong block. In particular, this saves the
            // caller some gas in the race condition where two clients try to sequence the same
            // block at the same time, and the first one wins.
            revert IncorrectBlockNumber(qcs[i].height, blockHeight);
        }

        // Check that QC is signed and well-formed.
        //TODO may need to add back
        // if (!_verifyWarpBlock(warp)) {
        //     revert InvalidQC(blockHeight);
        // }

        commitments[blockHeight] = qcs[i].blockCommitment;
        blockHeight += 1;
        

        emit NewBlocks(firstBlockNumber);
    }

    /// @notice Sends a request to Bonsai to have have the nth Fibonacci number calculated.
    /// @dev This function sends the request to Bonsai through the on-chain relay.
    ///      The request will trigger Bonsai to run the specified RISC Zero guest program with
    ///      the given input and asynchronously return the verified results via the callback below.
    function calculateFibonacci(uint256 n) external {
        bonsaiRelay.requestCallback(
            fibImageId, abi.encode(n), address(this), this.storeResult.selector, BONSAI_CALLBACK_GAS_LIMIT
        );
    }

    function newBlock(
        bytes memory message,
        G1Point memory sig,
        bool[] memory bitmap,
        uint256 minStakeThreshold
    ) external {
        require(bitmap.length <= _stakingKeys.length, "bitmap is too long");

        // Build aggregated public key

        // Loop until we find a one in the bitmap
        uint256 index = 0;
        while (!bitmap[index] && index < bitmap.length) {
            index++;
        }

        if (index >= bitmap.length) {
            revert NoKeySelected();
        }

        // Compute the stake corresponding to the signers and check if it is enough
        uint256 stake = 0;
        for (uint256 i = index; i < bitmap.length; i++) {
            if (bitmap[i]) {
                stake += _stakeAmounts[i]; // TODO check to avoid wrapping around?
            }
        }

        if (stake < minStakeThreshold) {
            revert NotEnoughStake();
        }

        G2Point[] keys = new G2Point[](bitmap.length);


        for (uint256 i = index + 1; i < bitmap.length; i++) {
            if (bitmap[i]) {
                keys[i] = _stakingKeys[i];
            }
        }


        bonsaiRelay.requestCallback(
            blsImageId, abi.encode(keys, sig, message), address(this), this.storeResult.selector, BONSAI_CALLBACK_GAS_LIMIT
        );
    }

    
    function _verifyWarpBlock(WarpBlock calldata /* qc */ ) private pure returns (bool) {
        // TODO Check the QC
        // TODO Check the block number
        return true;
    }


    /// @dev Stake table related functions
    /// @notice This function is for testing purposes only. The real sequencer
    ///         contract will perform several checks before adding a new key (e.g.
    ///         validate deposits).
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

    /// @dev Verify an aggregated signature against a bitmap (use to reconstruct
    ///      the aggregated public key) and some stake threshold. If the stake
    ///      involved by the signers is bigger than the threshold and the signature is
    ///      valid then the validation passes, otherwise the transaction
    ///      reverts.
    /// @param message message to check the signature against
    /// @param sig aggregated signature
    /// @param bitmap bit vector that corresponds to the public keys of the stake
    ///        table to take into account to build the aggregated public key
    /// @param minStakeThreshold total stake that must me matched by the
    ///        signers in order for the signature to be valid
    function verifyAggSig(
        bytes memory message,
        G1Point memory sig,
        bool[] memory bitmap,
        uint256 minStakeThreshold
    ) public view {
        require(bitmap.length <= _stakingKeys.length, "bitmap is too long");

        // Build aggregated public key

        // Loop until we find a one in the bitmap
        uint256 index = 0;
        while (!bitmap[index] && index < bitmap.length) {
            index++;
        }

        if (index >= bitmap.length) {
            revert NoKeySelected();
        }

        // Compute the stake corresponding to the signers and check if it is enough
        uint256 stake = 0;
        for (uint256 i = index; i < bitmap.length; i++) {
            if (bitmap[i]) {
                stake += _stakeAmounts[i]; // TODO check to avoid wrapping around?
            }
        }

        if (stake < minStakeThreshold) {
            revert NotEnoughStake();
        }

        BN254.G2Point memory aggPk = _stakingKeys[index];
        for (uint256 i = index + 1; i < bitmap.length; i++) {
            if (bitmap[i]) {
                BN254.G2Point memory pk = _stakingKeys[i];

                // Note: (x,y) coordinates for each field component must be inverted.
                uint256 p1xy = aggPk.x0;
                uint256 p1xx = aggPk.x1;
                uint256 p1yy = aggPk.y0;
                uint256 p1yx = aggPk.y1;
                uint256 p2xy = pk.x0;
                uint256 p2xx = pk.x1;
                uint256 p2yy = pk.y0;
                uint256 p2yx = pk.y1;

                (uint256 p3xx, uint256 p3xy, uint256 p3yx, uint256 p3yy) =
                    BN256G2.ECTwistAdd(p1xx, p1xy, p1yx, p1yy, p2xx, p2xy, p2yx, p2yy);
                aggPk = BN254.G2Point(p3xy, p3xx, p3yy, p3yx);
            }
        }

        BLSSig.verifyBlsSig(message, sig, aggPk);
    }
}