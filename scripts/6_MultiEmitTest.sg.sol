// SPDX-License-Identifier: CAL
pragma solidity =0.8.18;

import "forge-std/Script.sol";
import "forge-std/Test.sol";
import "forge-std/StdJson.sol";
import "./Subgraph.sol";

contract MultiEmitTest is Script, Test, Subgraph {
    using stdJson for string;

    function run() public {
        string memory config = vm.readFile(
            "broadcast/1_EmitMeta.sg.sol/31337/run-latest.json"
        );
        address metaBoard = stdJson.readAddress(
            config,
            ".receipts[0].contractAddress"
        );

        string memory multiReceipts = vm.readFile(
            "broadcast/5_MultiEmitMeta.sg.sol/31337/run-latest.json"
        );
        waitForSubgraphToSync(block.number);
        string memory response = string(getMetaBoard(address(metaBoard)));

        for (uint256 i = 0; i < 2; i++)
            assertEq(
                stdJson.readString(
                    response,
                    string(abi.encodePacked(".metas[", vm.toString(i + 1), "]"))
                ),
                stdJson.readString(
                    multiReceipts,
                    string(
                        abi.encodePacked(
                            ".transactions[",
                            vm.toString(i),
                            "].hash"
                        )
                    )
                )
            );
        require(!failed(), "MultiEmitTest failed");
        console.log("Multi event test passed");
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
