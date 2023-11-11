// SPDX-License-Identifier: CAL
pragma solidity =0.8.19;

import {IMetaBoardV1} from "../interface/IMetaBoardV1.sol";
import {LibMeta} from "../lib/LibMeta.sol";

contract MetaBoard is IMetaBoardV1 {
    /// @inheritdoc IMetaBoardV1
    function emitMeta(uint256 subject_, bytes calldata meta_) public {
        LibMeta.checkMetaUnhashed(meta_);
        emit MetaV1(msg.sender, subject_, meta_);
    }
}
