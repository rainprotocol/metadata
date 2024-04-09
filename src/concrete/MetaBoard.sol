// SPDX-License-Identifier: CAL
pragma solidity =0.8.19;

import {IMetaBoardV1} from "../interface/IMetaBoardV1.sol";
import {LibMeta} from "../lib/LibMeta.sol";

contract MetaBoard is IMetaBoardV1 {
    /// @inheritdoc IMetaBoardV1
    function emitMeta(uint256 subject, bytes calldata meta) external {
        LibMeta.checkMetaUnhashedV1(meta);
        emit MetaV1(msg.sender, subject, meta);
    }

    /// Exposes native hashing algorithm (keccak256) to facilitate indexing data
    /// under its hash. This avoids the need to roll a new interface to include
    /// hashes in the event logs.
    function hash(bytes calldata data) external pure returns (bytes32) {
        return keccak256(data);
    }
}
