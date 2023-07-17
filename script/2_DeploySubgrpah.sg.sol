// SPDX-License-Identifier: CAL
pragma solidity =0.8.18;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";

contract DeploySubgraphScript is Script {
    using stdJson for string;

    function run() public {
        string memory config = vm.readFile(
            "broadcast/1_EmitMeta.sg.sol/31337/run-latest.json"
        );
        address metaBoard = stdJson.readAddress(
            config,
            ".receipts[0].contractAddress"
        );
        uint256 blockNumber = stdJson.readUint(
            config,
            ".receipts[0].blockNumber"
        );

        deploySubgraph(metaBoard, blockNumber);
        console.log("Subgraph deployed");
    }

    function deploySubgraph(address metaBoard, uint256 block_number) public {
        string[] memory command = new string[](16);
        command[0] = "cargo";
        command[1] = "run";
        command[2] = "--quiet";
        command[3] = "--manifest-path";
        command[4] = "metaboard-cli/Cargo.toml";
        command[5] = "deploy";
        command[6] = "-c";
        command[7] = vm.toString(metaBoard);
        command[8] = "-b";
        command[9] = vm.toString(block_number);
        command[10] = "-t";
        command[11] = "subgraph/subgraph.template.yaml";
        command[12] = "-o";
        command[13] = "subgraph/subgraph.yaml";
        command[14] = "-r";
        command[15] = "subgraph/";
        
        vm.ffi(command);
    }
}
