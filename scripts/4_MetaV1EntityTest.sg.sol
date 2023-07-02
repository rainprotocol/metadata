// SPDX-License-Identifier: CAL
pragma solidity =0.8.18;

import "forge-std/Script.sol";
import "forge-std/Test.sol";
import "forge-std/StdJson.sol";

contract MetaBoardEntityTest is Script, Test {
    using stdJson for string;

    function run() public {
        string memory config = vm.readFile("scripts/config.json");
        address metaBoard = stdJson.readAddress(config, ".contract");
        uint256 blockNumber = stdJson.readUint(config, ".block");

        string memory receipt = vm.readFile(
            "broadcast/EmitMeta.s.sol/31337/run-latest.json"
        );
        bytes32 trx = stdJson.readBytes32(receipt, ".transactions[1].hash");

        // bytes memory resposne = getMetaV1(trx);
        // (
        //     address id,
        //     address _address,
        //     uint256 metaCount,
        //     string[] memory metas
        // ) = abi.decode(resposne, (address, address, uint256, string[]));

        // console2.logAddress(id);
        // assertEq(id, address(metaBoard));
        // assertEq(_address, address(metaBoard));
        // assertEq(1, metaCount);
        // require(!failed(), "test case failed");
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
