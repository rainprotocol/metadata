// SPDX-License-Identifier: CAL
pragma solidity =0.8.18;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";


contract Subgraph is Script {
    function waitForSubgraphToSync(uint256 blockNumber) public {
        string[] memory command = new string[](8);
        command[0] = "cargo";
        command[1] = "run";
        command[2] = "--quiet";
        command[3] = "--manifest-path";
        command[4] = "metaboard-cli/Cargo.toml";
        command[5] = "wait";
        command[6] = "-b";
        command[7] = vm.toString(blockNumber);

        string memory resposne = string(vm.ffi(command));
        require(stdJson.readUint(resposne, ".status") == 1, "Subgraph sync failed");
    }
}