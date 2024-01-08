// SPDX-License-Identifier: CAL
pragma solidity =0.8.19;

import {Test} from "forge-std/Test.sol";
import {IMetaV1, NotRainMetaV1, META_MAGIC_NUMBER_V1} from "src/interface/IMetaV1.sol";
import {LibMeta} from "src/lib/LibMeta.sol";
import {MetaBoard} from "src/concrete/MetaBoard.sol";

contract MetaBoardTest is Test, IMetaV1 {
    function testEmitMeta(uint256 subject_, bytes memory data_) public {
        vm.assume(!LibMeta.isRainMetaV1(data_));

        MetaBoard metaBoard_ = new MetaBoard();

        bytes memory meta_ = abi.encodePacked(META_MAGIC_NUMBER_V1, data_);
        vm.expectEmit(false, false, false, true);
        //slither-disable-next-line reentrancy-events
        emit MetaV1(address(this), subject_, meta_);
        metaBoard_.emitMeta(subject_, meta_);

        vm.expectRevert(abi.encodeWithSelector(NotRainMetaV1.selector, data_));
        metaBoard_.emitMeta(subject_, data_);
    }
}
