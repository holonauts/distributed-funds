// SPDX-License-Identifier: CAL
pragma solidity ^0.8.18;

import "rain.flow/concrete/basic/Flow.sol";
import "forge-std/Script.sol";

contract Implementation is Script {
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYMENT_KEY");
        vm.startBroadcast(deployerPrivateKey);

        Flow flow = new Flow();
        console2.log("Flow Address: ", address(flow));
    }
}
