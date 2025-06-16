// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

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
        console.log(
            "EAS Attestation TriggerId:",
            ITypes.TriggerId.unwrap(triggerId)
        );
        vm.stopBroadcast();
    }

    /// @notice New function for custom EAS attestation with schema and recipient
    function runEAS(
        string calldata serviceTriggerAddr,
        string calldata schema,
        string calldata recipient,
        string calldata data
    ) public {
        vm.startBroadcast(_privateKey);
        SimpleTrigger trigger = SimpleTrigger(
            vm.parseAddress(serviceTriggerAddr)
        );

        // Create JSON payload for EAS attestation component
        string memory easJsonPayload = string(
            abi.encodePacked(
                '{"schema":"',
                schema,
                '",',
                '"recipient":"',
                recipient,
                '",',
                '"data":"',
                data,
                '",',
                '"expiration_time":0,"revocable":true}'
            )
        );

        trigger.addTrigger(bytes(easJsonPayload));
        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        console.log(
            "Custom EAS Attestation TriggerId:",
            ITypes.TriggerId.unwrap(triggerId)
        );
        console.log("Schema:", schema);
        console.log("Recipient:", recipient);
        console.log("Data:", data);
        vm.stopBroadcast();
    }
}
