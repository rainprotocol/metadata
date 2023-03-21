// SPDX-License-Identifier: CAL
pragma solidity ^0.8.18;

import "forge-std/Test.sol";
import "../src/LibMeta.sol";

contract LibMetaTest is Test {
    function testIsRainMetaV1Fuzz(bytes memory data_) public {
        bytes memory meta_ = abi.encodePacked(META_MAGIC_NUMBER_V1, data_);
        // True with prefix.
        assertTrue(LibMeta.isRainMetaV1(meta_));
        // False without prefix.
        assertTrue(!LibMeta.isRainMetaV1(data_));
    }

    function testCheckMetaUnhashedFuzz(bytes memory data_) public {
        bytes memory meta_ = abi.encodePacked(META_MAGIC_NUMBER_V1, data_);
        LibMeta.checkMetaUnhashed(meta_);

        vm.expectRevert(abi.encodeWithSelector(NotRainMetaV1.selector, data_));
        LibMeta.checkMetaUnhashed(data_);
    }

    function testCheckMetaHashed(bytes memory data_) public {
        bytes memory meta_ = abi.encodePacked(META_MAGIC_NUMBER_V1, data_);
        bytes32 metaHash_ = keccak256(meta_);
        LibMeta.checkMetaHashed(metaHash_, meta_);

        vm.expectRevert(abi.encodeWithSelector(NotRainMetaV1.selector, data_));
        bytes32 dataHash_ = keccak256(data_);
        LibMeta.checkMetaHashed(dataHash_, data_);

        vm.expectRevert(abi.encodeWithSelector(UnexpectedMetaHash.selector, metaHash_, dataHash_));
        LibMeta.checkMetaHashed(metaHash_, data_);
    }
}
