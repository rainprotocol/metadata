// SPDX-License-Identifier: CAL
pragma solidity =0.8.18;

import "forge-std/Script.sol";
import "../src/IMetaV1.sol";
import "../src/LibMeta.sol";
import "../src/IMetaBoardV1.sol";

contract EmitMetaScript is Script {
    using stdJson for string;

    function run() public {
        uint256 deployer = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80;
        address deployer_address = address(uint160(deployer));
        vm.startBroadcast(deployer);

        string memory config = vm.readFile(
            "broadcast/1_EmitMeta.sg.sol/31337/run-latest.json"
        );
        address metaBoardAddress = stdJson.readAddress(
            config,
            ".receipts[0].contractAddress"
        );

        bytes memory meta = stdJson.readBytes(
            config,
            ".transactions[1].arguments[1]"
        );

        IMetaBoardV1 metaBoard = IMetaBoardV1(metaBoardAddress);

        metaBoard.emitMeta(1, meta);
        metaBoard.emitMeta(1, meta);

        vm.stopBroadcast();
        console.log("Multiple events emmited");
    }
}
