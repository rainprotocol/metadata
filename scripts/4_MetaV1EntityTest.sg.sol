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

    function waitForSubgraphToSync() internal {
        string[] memory command = new string[](6);
        command[0] = "cargo";
        command[1] = "run";
        command[2] = "--quiet";
        command[3] = "--manifest-path";
        command[4] = "metaboard-cli/Cargo.toml";
        command[5] = "wait";

        vm.ffi(command);
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

//8d91c10e82300c86df65672e9ae8c177c083315e8c31454a32031dd93ae34278778b828006f1b6a6fffeff6b7bac94a6d2b3539be793d112e4fb50a2daa824303a152982a2293d15c820350fdbf5e92dd81ade81a658548745af436b8d557554292043a1305ed232c81d46e3f014ef980e5a231848538b6e80e3503ed83ea6133441335e5eeae56a3df0f2c9152fdc9b758a3fcc3eb634b3a3afe5dc90b88d99bac314ee798a778eeffc03100bcdf16b06e3b94592b663608c3d43a273cd419472cc12022439f67699a70b6b43e2f800
//0xff0a89c674ee7874a50058d0
