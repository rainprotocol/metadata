// SPDX-License-Identifier: CAL
pragma solidity =0.8.18;

import "forge-std/Script.sol";
import "../src/IMetaV1.sol";
import "../src/LibMeta.sol";
import "../src/IMetaBoardV1.sol";

contract EmitMetaScript is Script {
    bytes meta;
    using stdJson for string;

    function run() public {
        uint256 deployer = vm.envUint("DEPLOYER_KEY");
        address deployer_address = address(uint160(deployer));
        vm.startBroadcast(deployer);

        string memory config = vm.readFile("scripts/config.json");
        address metaBoard_address = stdJson.readAddress(config, ".contract");
        meta = stdJson.readBytes(config, ".meta");

        IMetaBoardV1 metaBoard = IMetaBoardV1(metaBoard_address);

        metaBoard.emitMeta(1, meta);
        metaBoard.emitMeta(1, meta);
        metaBoard.emitMeta(1, meta);
        metaBoard.emitMeta(1, meta);

        vm.stopBroadcast();
    }
}
