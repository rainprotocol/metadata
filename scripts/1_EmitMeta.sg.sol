// SPDX-License-Identifier: CAL
pragma solidity =0.8.18;

import "forge-std/Script.sol";
import "../src/IMetaV1.sol";
import "../src/LibMeta.sol";
import "../src/MetaBoard.sol";

contract EmitMetaScript is Script {
    bytes meta;

    function setUp() public {
        meta = getMetaData();
    }

    function run() public {
        uint256 deployer = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80;
        vm.startBroadcast(deployer);
        MetaBoard metaBoard = new MetaBoard();
        metaBoard.emitMeta(1, meta);
        vm.stopBroadcast();
    }

    function getMetaData() internal returns (bytes memory) {
        string[] memory command = new string[](18);
        command[0] = "cargo";
        command[1] = "run";
        command[2] = "--quiet";
        command[3] = "--manifest-path";
        command[4] = "cli/Cargo.toml";
        command[5] = "build";
        command[6] = "-t";
        command[7] = "json";
        command[8] = "-E";
        command[9] = "hex";
        command[10] = "-i";
        command[11] = "scripts/abi.json";
        command[12] = "-m";
        command[13] = "solidity-abi-v2";
        command[14] = "-e";
        command[15] = "deflate";
        command[16] = "-l";
        command[17] = "en";

        return vm.ffi(command);
    }
}
