#!/bin/sh

# build
forge install
cd lib/openzeppelin-contracts && forge build -o ../out && cd ../
cd lib/rain.flow && forge build -o ../out && cd ../
cd lib/rain.interpreter.interface && forge build && cd ../
cd lib/rain.interpreter && forge build -o ../out  && cd ../
forge build 

# copy
cp out/IParserV2.sol/IParserV2.json ../ui/src/lib/abi/
cp out/CloneFactory.sol/CloneFactory.json ../ui/src/lib/abi/
cp out/IFlowV5.sol/IFlowV5.json ../ui/src/lib/abi/
cp out/IERC20.sol/IERC20.json ../ui/src/lib/abi/
cp src/grantpool-flow.rain ../ui/static/rain