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
import {IERC20} from "openzeppelin/token/ERC20/IERC20.sol";

contract CloneFlow is Script {
    function run() public {
      address RAIN_INTERPRETER_ADDRESS = 0x8bb0e1Ade233f386668f6e3c11762f18bF8293b3;
      address RAIN_STORE_ADDRESS = 0xCCe6D0653B6DAC3B5fAd3F2A8E47cCE537126aD0;

      address FLOW_CLONE_ADDRESS = 0xE12d142a226E0C7096039cebbaCB4dff625Dd8a3;
      address DEPOSIT_EXPRESSION_ADDRESS = 0xf7F99860Ee1D60ca5981d53A342e5B1BF68032E2;
      
      address MY_WALLET_ADDRESS = 0x8d0F52e2A9d9fF7F4a6952C32552dEAd4f732DD5;
      uint256 AMOUNT = 100;      

      string memory POLYGON_RPC_URL = vm.envString("POLYGON_RPC_URL");
      uint256 forkId = vm.createFork(POLYGON_RPC_URL);
      vm.startPrank(MY_WALLET_ADDRESS);
      vm.selectFork(forkId);

      // Confirm my wallet has granted allowance to flow clone contract
      IERC20 paymentToken = IERC20(0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359);
      uint256 allowance = paymentToken.allowance(MY_WALLET_ADDRESS, FLOW_CLONE_ADDRESS);
      console.log("allowance", allowance);
      uint256 balance = paymentToken.balanceOf(MY_WALLET_ADDRESS);
      console.log("balance", balance);

      // Call deposit on flow clone
      EvaluableV2 memory evaluableDeposit = EvaluableV2(IInterpreterV2(RAIN_INTERPRETER_ADDRESS), IInterpreterStoreV2(RAIN_STORE_ADDRESS), DEPOSIT_EXPRESSION_ADDRESS);
      Flow flow = Flow(FLOW_CLONE_ADDRESS);
      uint256[] memory callerContext = new uint256[](1);
      callerContext[0] = AMOUNT;
      SignedContextV1[] memory signedContext = new SignedContextV1[](0);
      
      
      flow.flow(
        evaluableDeposit, 
        callerContext,
        signedContext
      );
    }
}
