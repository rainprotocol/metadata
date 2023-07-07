// SPDX-License-Identifier: CAL
pragma solidity =0.8.18;

import "forge-std/Script.sol";
import "forge-std/Test.sol";
import "forge-std/StdJson.sol";

contract MetaBoardEntityTest is Script, Test {
    using stdJson for string;

    function run() public {
        uint256 deployer = vm.envUint("DEPLOYER_KEY");
        address deployerAddress = vm.rememberKey(deployer);
        string memory config = vm.readFile(
            "broadcast/1_EmitMeta.sg.sol/31337/run-latest.json"
        );
        address metaBoard = stdJson.readAddress(
            config,
            ".receipts[0].contractAddress"
        );

        bytes32 trx = stdJson.readBytes32(config, ".transactions[1].hash");
        bytes memory meta_ = stdJson.readBytes(
            config,
            ".transactions[1].arguments[1]"
        );

        string memory resposne = string(getMetaV1(trx));

        bytes memory payloadBytes = vm.parseBytes(
            stdJson.readString(resposne, ".payload")
        );

        assertEq(stdJson.readString(resposne, ".id"), vm.toString(trx));
        assertEq(stdJson.readAddress(resposne, ".sender"), deployerAddress);
        assertEq(stdJson.readAddress(resposne, ".meta_board"), metaBoard);
        assertEq(
            stdJson.readUint(resposne, ".magic_number"),
            0xffe5ffb4a3ff2cde
        );
        assertEq(stdJson.readUint(resposne, ".subject"), 1);
        assertEq(
            stdJson.readString(resposne, ".content_type"),
            "application/json"
        );
        for (uint256 i = 0; i < 208; i++) {
            assertEq(payloadBytes[i], meta_[i + 12]);
        }

        require(!failed(), "MetaBoardEntityTest failed");
        console.log("MetaV1 entity tests passed");
    }

    function getMetaV1(bytes32 transaction) internal returns (bytes memory) {
        string[] memory command = new string[](9);
        command[0] = "cargo";
        command[1] = "run";
        command[2] = "--quiet";
        command[3] = "--manifest-path";
        command[4] = "metaboard-cli/Cargo.toml";
        command[5] = "query";
        command[6] = "meta-v1";
        command[7] = "-t";
        command[8] = vm.toString(transaction);

        return vm.ffi(command);
    }
}
