// SPDX-License-Identifier: CAL
pragma solidity =0.8.18;

import "forge-std/Script.sol";
import "forge-std/Test.sol";
import "forge-std/StdJson.sol";
import "./Subgraph.sol";

contract MetaV1EntityTest is Script, Test, Subgraph {
    using stdJson for string;

    function run() public {
        string memory config = vm.readFile(
            "broadcast/1_EmitMeta.sg.sol/31337/run-latest.json"
        );
        address metaBoard = stdJson.readAddress(
            config,
            ".receipts[0].contractAddress"
        );
        bytes32 meta_id = stdJson.readBytes32(
            config,
            ".receipts[1].transactionHash"
        );
        waitForSubgraphToSync(block.number);
        string memory response = string(getMetaBoard(address(metaBoard)));

        assertEq(stdJson.readAddress(response, ".id"), address(metaBoard));
        assertEq(stdJson.readAddress(response, ".address"), address(metaBoard));
        assertEq(stdJson.readUint(response, ".meta_count"), 1);
        assertEq(
            stdJson.readString(response, ".metas[0]"),
            vm.toString(meta_id)
        );
        require(!failed(), "MetaV1EntityTest failed");
        console.log("MetaBoard entity test passed");
    }

    function getMetaBoard(address metaBoard) internal returns (bytes memory) {
        string[] memory command = new string[](9);
        command[0] = "cargo";
        command[1] = "run";
        command[2] = "--quiet";
        command[3] = "--manifest-path";
        command[4] = "metaboard-cli/Cargo.toml";
        command[5] = "query";
        command[6] = "meta-board";
        command[7] = "-m";
        command[8] = vm.toString(metaBoard);

        return vm.ffi(command);
    }
}
