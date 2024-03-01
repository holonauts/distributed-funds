// SPDX-License-Identifier: CAL
pragma solidity ^0.8.18;

import {Flow} from "rain.flow/concrete/basic/Flow.sol";
import "forge-std/Script.sol";
import {CloneFactory} from "rain.factory/src/concrete/CloneFactory.sol";
import {EvaluableConfigV3} from "rain.interpreter.interface/interface/IInterpreterCallerV2.sol";
import {IExpressionDeployerV3} from "rain.interpreter.interface/interface/IExpressionDeployerV3.sol";

contract Implementation is Script {
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYMENT_KEY");

        address rainDeployerAddress = 0xB16bbF12ECE3414af72F660aB63F4dDa1D7250FA;
        bytes memory bytecode = hex"1359f956188877b970f30cfa1bf11346a3d204195d895855d660e9bebef9d989";
        uint256[] memory constants = new uint256[](0);
        EvaluableConfigV3[] memory evaluableConfigs = new EvaluableConfigV3[](1);
        evaluableConfigs[0] = EvaluableConfigV3(IExpressionDeployerV3(rainDeployerAddress), bytecode, constants);

        bytes memory configEncoded = abi.encode(evaluableConfigs);
        console2.log("evaluable config encoded: ");
        console2.logBytes(configEncoded);

        Flow flow = new Flow();
        flow.initialize(configEncoded);
        
    }
}
