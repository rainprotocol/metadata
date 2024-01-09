// SPDX-License-Identifier: CAL
pragma solidity =0.8.19;

import {Test} from "forge-std/Test.sol";
import {LibMeta} from "src/lib/LibMeta.sol";
import {UnexpectedMetaHash, NotRainMetaV1, META_MAGIC_NUMBER_V1} from "src/interface/IMetaV1.sol";

contract LibMetaCheckMetaHashedV1Test is Test {
    /// When the data has a magic number, and the hash of the data matches the
    /// expected hash passed to the check, it should not revert.
    function testCheckMetaHashedV1Happy(bytes memory data) external pure {
        bytes memory meta = abi.encodePacked(META_MAGIC_NUMBER_V1, data);
        bytes32 metaHash = keccak256(meta);
        LibMeta.checkMetaHashedV1(metaHash, meta);
    }

    /// When the data has a magic number but the hash of the data does not
    /// match the expected hash passed to the check, it should revert.
    function testCheckMetaHashedV1GoodMagicBadHash(bytes memory data, bytes32 expectedHash) public {
        bytes memory meta = abi.encodePacked(META_MAGIC_NUMBER_V1, data);
        bytes32 metaHash = keccak256(meta);
        vm.assume(metaHash != expectedHash);
        vm.expectRevert(abi.encodeWithSelector(UnexpectedMetaHash.selector, expectedHash, metaHash));
        LibMeta.checkMetaHashedV1(expectedHash, meta);
    }

    /// When the data does not have a magic number, it should revert even if
    /// the hash of the data matches the expected hash passed to the check.
    function testCheckMetaHashedV1BadMagicGoodHash(bytes memory meta) public {
        bytes32 metaHash = keccak256(meta);
        vm.expectRevert(abi.encodeWithSelector(NotRainMetaV1.selector, meta));
        LibMeta.checkMetaHashedV1(metaHash, meta);
    }

    /// When the data does not have a magic number, and the hash of the data
    /// does not match the expected hash passed to the check, it should revert.
    function testCheckMetaHashedV1BadMagicBadHash(bytes memory meta, bytes32 expectedHash) public {
        bytes32 metaHash = keccak256(meta);
        vm.assume(metaHash != expectedHash);

        vm.expectRevert(abi.encodeWithSelector(UnexpectedMetaHash.selector, expectedHash, metaHash));
        LibMeta.checkMetaHashedV1(expectedHash, meta);
    }
}
