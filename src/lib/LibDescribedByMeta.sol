// SPDX-License-Identifier: CAL
pragma solidity ^0.8.19;

import {IDescribedByMetaV1} from "../interface/unstable/IDescribedByMetaV1.sol";
import {IMetaBoardV1} from "../interface/IMetaBoardV1.sol";

error MetadataMismatch(IDescribedByMetaV1 described, bytes32 expected, bytes32 actual);

/// @title LibDescribedByMeta
library LibDescribedByMeta {
    function emitForDescribedAddress(IMetaBoardV1 metaboard, IDescribedByMetaV1 described, bytes memory meta)
        internal
    {
        bytes32 expected = described.describedByMetaV1();
        bytes32 actual = keccak256(meta);
        if (actual != expected) {
            revert MetadataMismatch(described, expected, actual);
        }
        metaboard.emitMeta(uint256(uint160(address(described))), meta);
    }
}
