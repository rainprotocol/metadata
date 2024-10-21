// SPDX-License-Identifier: LicenseRef-DCL-1.0
// SPDX-FileCopyrightText: Copyright (c) 2020 thedavidmeister
pragma solidity =0.8.19;

import {Script} from "forge-std/Script.sol";
import {MetaBoard} from "src/concrete/MetaBoard.sol";

/// @title Deploy
/// @notice A script that deploys all contracts. This is intended to be run on
/// every commit by CI to a testnet such as mumbai.
contract Deploy is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYMENT_KEY");

        vm.startBroadcast(deployerPrivateKey);

        new MetaBoard();

        vm.stopBroadcast();
    }
}
