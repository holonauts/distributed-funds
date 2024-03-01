import "rain.flow/concrete/basic/Flow.sol";

contract Implementation is Script {
    function run() public {
        vm.startBroadcast(deployerPrivateKey);

        Flow flow = new Flow();
    }
}
