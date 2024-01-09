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
}
