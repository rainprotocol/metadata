// SPDX-License-Identifier: CAL
pragma solidity =0.8.16;

import "./IMetaBoardV1.sol";
import "./LibMeta.sol";

contract MetaBoard is IMetaBoardV1 {
    /// @inheritdoc IMetaBoardV1
    function emitMeta(bytes calldata meta_) public {
        LibMeta.checkMetaUnhashed(meta_);
        emit MetaV1(msg.sender, meta_);
    }
}
