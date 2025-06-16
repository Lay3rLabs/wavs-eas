// SPDX-License-Identifier: MIT
pragma solidity 0.8.22;

import {SimpleTrigger} from "contracts/WavsTrigger.sol";
import {ITypes} from "interfaces/ITypes.sol";
import {Common} from "script/Common.s.sol";
import {console} from "forge-std/console.sol";

/// @dev Script to add a new trigger for EAS attestations
contract Trigger is Common {
    function run(
        string calldata serviceTriggerAddr,
        uint256 chainId,
        string calldata attestationId
    ) public {
        vm.startBroadcast(_privateKey);
        SimpleTrigger trigger = SimpleTrigger(
            vm.parseAddress(serviceTriggerAddr)
        );

        // Create JSON payload with chainId and attestationId
        string memory payload = string.concat(
            '{"chainId":',
            vm.toString(chainId),
            ',"attestationId":"',
            attestationId,
            '"}'
        );

        trigger.addTrigger(bytes(payload));
        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        console.log("TriggerId", ITypes.TriggerId.unwrap(triggerId));
        vm.stopBroadcast();
    }
}
