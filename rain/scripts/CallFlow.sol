// SPDX-License-Identifier: CAL
pragma solidity ^0.8.19;

import {Flow} from "rain.flow/concrete/basic/Flow.sol";
import "forge-std/Script.sol";
import {CloneFactory} from "rain.factory/concrete/CloneFactory.sol";
import {EvaluableConfigV3} from "rain.interpreter.interface/interface/IInterpreterCallerV2.sol";
import {RainterpreterExpressionDeployerNPE2, RainterpreterExpressionDeployerNPE2ConstructionConfig, CONSTRUCTION_META_HASH} from "rain.interpreter/concrete/RainterpreterExpressionDeployerNPE2.sol";
import {RainterpreterParserNPE2} from "rain.interpreter/concrete/RainterpreterParserNPE2.sol";
import {RainterpreterStoreNPE2} from "rain.interpreter/concrete/RainterpreterStoreNPE2.sol";
import {RainterpreterNPE2} from "rain.interpreter/concrete/RainterpreterNPE2.sol";
import {EvaluableV2} from "rain.interpreter.interface/lib/caller/LibEvaluable.sol";
import {SignedContextV1, EvaluableConfigV3} from "rain.interpreter.interface/interface/IInterpreterCallerV2.sol";
import {IInterpreterStoreV2} from "rain.interpreter.interface/interface/IInterpreterStoreV2.sol";
import {IInterpreterV2} from "rain.interpreter.interface/interface/IInterpreterV2.sol";

contract CloneFlow is Script {
    function run() public {
      string memory POLYGON_RPC_URL = vm.envString("POLYGON_RPC_URL");
      uint256 forkId = vm.createFork(POLYGON_RPC_URL);
      vm.startPrank(0x8d0F52e2A9d9fF7F4a6952C32552dEAd4f732DD5);
      vm.selectFork(forkId);

      address RAIN_INTERPRETER_ADDRESS = 0x8bb0e1Ade233f386668f6e3c11762f18bF8293b3;
      address RAIN_STORE_ADDRESS = 0xCCe6D0653B6DAC3B5fAd3F2A8E47cCE537126aD0;
      address depositExpression = 0xd36E23f324480AaE6A42Ee4490fEf3FD03bDBfeE;

      EvaluableV2 memory evaluableDeposit = EvaluableV2(IInterpreterV2(RAIN_INTERPRETER_ADDRESS), IInterpreterStoreV2(RAIN_STORE_ADDRESS), depositExpression);
      Flow flow = Flow(0x0b4af3781FA212E2E4BaB6bA1086fab1FA48d225);
      uint256[] memory callerContext = new uint256[](1);
      callerContext[0] = 1;
      SignedContextV1[] memory signedContext = new SignedContextV1[](0);
      
      
      flow.flow(
        evaluableDeposit, 
        callerContext,
        signedContext
      );
    }
}
