// SPDX-License-Identifier: CAL
pragma solidity =0.8.18;

import "forge-std/Test.sol";
import "../src/IMetaV1.sol";
import "../src/LibMeta.sol";
import "../src/MetaBoard.sol";

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
