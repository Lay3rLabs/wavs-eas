// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {EASAttestTrigger} from "contracts/Trigger.sol";
import {IEAS} from "@ethereum-attestation-service/eas-contracts/contracts/IEAS.sol";
import {Attestation} from "@ethereum-attestation-service/eas-contracts/contracts/Common.sol";
import {ITypes} from "interfaces/ITypes.sol";
import {Common} from "script/Common.s.sol";
import {console} from "forge-std/console.sol";

/// @title EasTrigger
/// @notice Comprehensive script for EAS attestation operations via WAVS
/// @dev Consolidates all EAS trigger and query functionality
contract EasTrigger is Common {
    // ============================================================
    // TRIGGER FUNCTIONS
    // ============================================================

    /// @notice Trigger EAS attestation with default schema and recipient
    /// @param serviceTriggerAddr Address of the EAS trigger contract
    /// @param data Attestation data string
    function triggerEASAttestation(
        string calldata serviceTriggerAddr,
        string calldata data
    ) public {
        vm.startBroadcast(_privateKey);
        EASAttestTrigger trigger = EASAttestTrigger(
            vm.parseAddress(serviceTriggerAddr)
        );

        // Create JSON payload for EAS attestation component with defaults
        string memory easJsonPayload = string(
            abi.encodePacked(
                '{"schema":"0x0000000000000000000000000000000000000000000000000000000000000000",',
                '"recipient":"0x0000000000000000000000000000000000000000",',
                '"data":"',
                data,
                '",',
                '"expiration_time":0,"revocable":true}'
            )
        );

        trigger.addTrigger(bytes(easJsonPayload));
        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        console.log(
            "EAS Attestation TriggerId:",
            ITypes.TriggerId.unwrap(triggerId)
        );
        console.log("Attestation Data:", data);
        vm.stopBroadcast();
    }

    /// @notice Trigger custom EAS attestation with specific schema and recipient
    /// @param serviceTriggerAddr Address of the EAS trigger contract
    /// @param schema Schema UID (hex string)
    /// @param recipient Recipient address (hex string)
    /// @param data Attestation data string
    function triggerCustomEASAttestation(
        string calldata serviceTriggerAddr,
        string calldata schema,
        string calldata recipient,
        string calldata data
    ) public {
        vm.startBroadcast(_privateKey);
        EASAttestTrigger trigger = EASAttestTrigger(
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

    /// @notice Trigger attestation creation with JSON format using new contract method
    /// @param triggerAddr Address of the EAS attest trigger contract
    /// @param schema Schema UID (hex string)
    /// @param recipient Recipient address (hex string, use 0x0 for no specific recipient)
    /// @param data Attestation data string
    function triggerJsonAttestation(
        string calldata triggerAddr,
        string calldata schema,
        string calldata recipient,
        string calldata data
    ) public {
        vm.startBroadcast(_privateKey);

        EASAttestTrigger trigger = EASAttestTrigger(
            vm.parseAddress(triggerAddr)
        );

        bytes32 schemaBytes = bytes32(vm.parseBytes(schema));
        address recipientAddr = vm.parseAddress(recipient);

        console.log("Creating EAS attestation trigger:");
        console.log("Schema:", schema);
        console.log("Recipient:", recipient);
        console.log("Data:", data);

        trigger.triggerRequestAttestation(schemaBytes, recipientAddr, data);

        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        console.log("Trigger ID:", ITypes.TriggerId.unwrap(triggerId));

        vm.stopBroadcast();
    }

    /// @notice Trigger attestation creation with raw data
    /// @param triggerAddr Address of the EAS attest trigger contract
    /// @param rawData Raw attestation data (will use component defaults)
    function triggerRawAttestation(
        string calldata triggerAddr,
        string calldata rawData
    ) public {
        vm.startBroadcast(_privateKey);

        EASAttestTrigger trigger = EASAttestTrigger(
            vm.parseAddress(triggerAddr)
        );

        console.log("Creating raw EAS attestation trigger:");
        console.log("Data:", rawData);

        trigger.triggerRequestRawAttestation(bytes(rawData));

        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        console.log("Trigger ID:", ITypes.TriggerId.unwrap(triggerId));

        vm.stopBroadcast();
    }

    // ============================================================
    // EXAMPLE TRIGGERS
    // ============================================================

    /// @notice Example: Create a simple testimonial attestation
    /// @param triggerAddr Address of the EAS attest trigger contract
    function triggerTestimonialExample(string calldata triggerAddr) public {
        vm.startBroadcast(_privateKey);

        EASAttestTrigger trigger = EASAttestTrigger(
            vm.parseAddress(triggerAddr)
        );

        // Example schema for testimonials (you'd register this in EAS first)
        bytes32 testimonialSchema = 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;
        address recipient = address(0); // No specific recipient
        string
            memory testimonialData = "This person is trustworthy and professional.";

        console.log("Creating testimonial attestation:");
        console.log("Data:", testimonialData);

        trigger.triggerRequestAttestation(
            testimonialSchema,
            recipient,
            testimonialData
        );

        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        console.log(
            "Testimonial Trigger ID:",
            ITypes.TriggerId.unwrap(triggerId)
        );

        vm.stopBroadcast();
    }

    /// @notice Example: Create a skill verification attestation
    /// @param triggerAddr Address of the EAS attest trigger contract
    /// @param skillHolder Address of the person whose skill is being attested
    function triggerSkillVerificationExample(
        string calldata triggerAddr,
        string calldata skillHolder
    ) public {
        vm.startBroadcast(_privateKey);

        EASAttestTrigger trigger = EASAttestTrigger(
            vm.parseAddress(triggerAddr)
        );

        // Example schema for skill verification
        bytes32 skillSchema = 0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefab00;
        address skillHolderAddr = vm.parseAddress(skillHolder);
        string
            memory skillData = "Verified: Advanced Solidity Development Skills";

        console.log("Creating skill verification attestation:");
        console.log("Skill holder:", skillHolder);
        console.log("Skill:", skillData);

        trigger.triggerRequestAttestation(
            skillSchema,
            skillHolderAddr,
            skillData
        );

        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        console.log(
            "Skill Verification Trigger ID:",
            ITypes.TriggerId.unwrap(triggerId)
        );

        vm.stopBroadcast();
    }

    // ============================================================
    // QUERY FUNCTIONS
    // ============================================================

    /// @notice Show current trigger ID
    /// @param serviceTriggerAddr Address of the trigger contract
    function showTrigger(string calldata serviceTriggerAddr) public view {
        EASAttestTrigger triggerInstance = EASAttestTrigger(
            vm.parseAddress(serviceTriggerAddr)
        );
        ITypes.TriggerId triggerId = triggerInstance.nextTriggerId();

        console.log("Next TriggerID:", ITypes.TriggerId.unwrap(triggerId));
    }

    /// @notice Query EAS attestations for a specific schema and recipient
    /// @param easAddr The EAS contract address
    /// @param schemaId The schema UID to query
    /// @param recipient The recipient address (use address(0) for all)
    function queryAttestations(
        string calldata easAddr,
        string calldata schemaId,
        string calldata recipient
    ) public pure {
        console.log("Querying EAS attestations:");
        console.log("  EAS Address:", easAddr);
        console.log("  Schema ID:", schemaId);
        console.log("  Recipient:", recipient);

        // Note: This is a simplified view - in practice you'd need to query
        // attestations through events or external indexing services
        console.log(
            "Use external tools or event logs to query specific attestations"
        );
    }

    /// @notice Show attestation details by UID
    /// @param easAddr The EAS contract address
    /// @param attestationUid The attestation UID
    function showAttestation(
        string calldata easAddr,
        string calldata attestationUid
    ) public view {
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
            console.log(
                "Attestation not found or invalid UID:",
                attestationUid
            );
        }
    }
}
