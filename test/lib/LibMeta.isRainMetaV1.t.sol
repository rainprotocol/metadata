// SPDX-License-Identifier: CAL
pragma solidity =0.8.19;

import {Test} from "forge-std/Test.sol";
import {LibMeta} from "src/lib/LibMeta.sol";
import {UnexpectedMetaHash, NotRainMetaV1, META_MAGIC_NUMBER_V1} from "src/interface/IMetaV1.sol";

contract LibMetaIsRainMetaV1Test is Test {
    /// All data with the magic number prefix will be considered to be rain meta
    /// and all without will not.
    function testIsRainMetaV1Fuzz(bytes memory data) public {
        bytes memory meta = abi.encodePacked(META_MAGIC_NUMBER_V1, data);
        // True with prefix.
        assertTrue(LibMeta.isRainMetaV1(meta));
        // False without prefix.
        assertTrue(!LibMeta.isRainMetaV1(data));
    }
}
