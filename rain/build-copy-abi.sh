#!/bin/sh

# build
forge install
forge build 
forge build --root lib/openzeppelin-contracts -o out
forge build --root lib/rain.flow -o out
forge build --root lib/rain.interpreter.interface -o out
forge build --root lib/rain.interpreter -o out

# copy
cp out/IParserV2.sol/IParserV2.json ../ui/src/lib/abi/
cp out/CloneFactory.sol/CloneFactory.json ../ui/src/lib/abi/
cp out/IFlowV5.sol/IFlowV5.json ../ui/src/lib/abi/
cp out/IERC20.sol/IERC20.json ../ui/src/lib/abi/
cp src/grantpool-flow.rain ../ui/static/rain