// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {SimpleTrigger} from "contracts/WavsTrigger.sol";
import {IEAS} from "@ethereum-attestation-service/eas-contracts/contracts/IEAS.sol";
import {Attestation} from "@ethereum-attestation-service/eas-contracts/contracts/Common.sol";
import {ITypes} from "interfaces/ITypes.sol";
import {Common} from "script/Common.s.sol";
import {console} from "forge-std/console.sol";

/// @dev Script to show the result of a trigger and EAS attestations
contract ShowResult is Common {
    function trigger(string calldata serviceTriggerAddr) public view {
        SimpleTrigger triggerInstance = SimpleTrigger(vm.parseAddress(serviceTriggerAddr));
        ITypes.TriggerId triggerId = triggerInstance.nextTriggerId();

        console.log("TriggerID:", ITypes.TriggerId.unwrap(triggerId));
    }

    /// @notice Query EAS attestations for a specific schema and recipient
    /// @param easAddr The EAS contract address
    /// @param schemaId The schema UID to query
    /// @param recipient The recipient address (use address(0) for all)
    function attestations(
        string calldata easAddr,
        string calldata schemaId,
        string calldata recipient
    ) public view {
        IEAS eas = IEAS(vm.parseAddress(easAddr));
        bytes32 schema = vm.parseBytes32(schemaId);
        address recipientAddr = vm.parseAddress(recipient);

        console.log("Querying EAS attestations:");
        console.log("  EAS Address:", easAddr);
        console.log("  Schema ID:", schemaId);
        console.log("  Recipient:", recipient);

        // Note: This is a simplified view - in practice you'd need to query
        // attestations through events or external indexing services
        console.log("Use external tools or event logs to query specific attestations");
    }

    /// @notice Show attestation details by UID
    /// @param easAddr The EAS contract address  
    /// @param attestationUid The attestation UID
    function attestation(string calldata easAddr, string calldata attestationUid) public view {
        IEAS eas = IEAS(vm.parseAddress(easAddr));
        bytes32 uid = vm.parseBytes32(attestationUid);

        try eas.getAttestation(uid) returns (Attestation memory att) {
            console.log("Attestation Details:");
            console.log("  UID:", attestationUid);
            console.log("  Schema:", vm.toString(att.schema));
            console.log("  Recipient:", vm.toString(att.recipient));
            console.log("  Attester:", vm.toString(att.attester));
            console.log("  Time:", att.time);
            console.log("  Expiration:", att.expirationTime);
            console.log("  Revocable:", att.revocable);
            console.log("  Ref UID:", vm.toString(att.refUID));
            console.log("  Data Length:", att.data.length);
            if (att.data.length > 0) {
                console.log("  Data:", string(att.data));
            }
        } catch {
            console.log("Attestation not found or invalid UID:", attestationUid);
        }
    }

    /// @notice Legacy function for backwards compatibility
    /// @dev This function is deprecated as we've moved to EAS-based attestations
    function data(string calldata, uint64 triggerId) public view {
        console.log("DEPRECATED: This function used SimpleSubmit which has been replaced by EAS.");
        console.log("TriggerID:", triggerId);
        console.log("Use 'attestations' or 'attestation' functions to query EAS data.");
    }
}
