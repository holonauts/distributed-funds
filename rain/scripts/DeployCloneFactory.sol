import {CloneFactory} from "rain.factory/src/concrete/CloneFactory.sol";
import "forge-std/Script.sol";

contract Implementation is Script {
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYMENT_KEY");
        vm.startBroadcast(deployerPrivateKey);

        CloneFactory clonefactory = new CloneFactory();
        console2.log("CloneFactory Address: ", address(clonefactory));
    }
}
