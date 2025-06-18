// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {EASAttestTrigger} from "contracts/EASAttestTrigger.sol";
import {ITypes} from "interfaces/ITypes.sol";
import {Common} from "script/Common.s.sol";
import {console} from "forge-std/console.sol";

/// @title EASAttestTriggerScript
/// @notice Script to trigger EAS attestation creation via WAVS
contract EASAttestTriggerScript is Common {
    /// @notice Trigger attestation creation with JSON format
    /// @param triggerAddr Address of the EAS attest trigger contract
    /// @param schema Schema UID (hex string)
    /// @param recipient Recipient address (hex string, use 0x0 for no specific recipient)
    /// @param data Attestation data string
    function runJsonAttestation(
        string calldata triggerAddr,
        string calldata schema,
        string calldata recipient,
        string calldata data
    ) public {
        vm.startBroadcast(_privateKey);
        
        EASAttestTrigger trigger = EASAttestTrigger(vm.parseAddress(triggerAddr));
        
        bytes32 schemaBytes = bytes32(vm.parseBytes(schema));
        address recipientAddr = vm.parseAddress(recipient);
        
        console.log("Creating EAS attestation trigger:");
        console.log("Schema:", schema);
        console.log("Recipient:", recipient);
        console.log("Data:", data);
        
        trigger.createAttestationTrigger(schemaBytes, recipientAddr, data);
        
        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        console.log("Trigger ID:", ITypes.TriggerId.unwrap(triggerId));
        
        vm.stopBroadcast();
    }

    /// @notice Trigger attestation creation with raw data
    /// @param triggerAddr Address of the EAS attest trigger contract  
    /// @param rawData Raw attestation data (will use component defaults)
    function runRawAttestation(
        string calldata triggerAddr,
        string calldata rawData
    ) public {
        vm.startBroadcast(_privateKey);
        
        EASAttestTrigger trigger = EASAttestTrigger(vm.parseAddress(triggerAddr));
        
        console.log("Creating raw EAS attestation trigger:");
        console.log("Data:", rawData);
        
        trigger.createRawAttestationTrigger(bytes(rawData));
        
        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        console.log("Trigger ID:", ITypes.TriggerId.unwrap(triggerId));
        
        vm.stopBroadcast();
    }

    /// @notice Example: Create a simple testimonial attestation
    /// @param triggerAddr Address of the EAS attest trigger contract
    function runTestimonialExample(string calldata triggerAddr) public {
        vm.startBroadcast(_privateKey);
        
        EASAttestTrigger trigger = EASAttestTrigger(vm.parseAddress(triggerAddr));
        
        // Example schema for testimonials (you'd register this in EAS first)
        bytes32 testimonialSchema = 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;
        address recipient = address(0); // No specific recipient
        string memory testimonialData = "This person is trustworthy and professional.";
        
        console.log("Creating testimonial attestation:");
        console.log("Data:", testimonialData);
        
        trigger.createAttestationTrigger(testimonialSchema, recipient, testimonialData);
        
        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        console.log("Testimonial Trigger ID:", ITypes.TriggerId.unwrap(triggerId));
        
        vm.stopBroadcast();
    }

    /// @notice Example: Create a skill verification attestation
    /// @param triggerAddr Address of the EAS attest trigger contract
    /// @param skillHolder Address of the person whose skill is being attested
    function runSkillVerificationExample(
        string calldata triggerAddr,
        string calldata skillHolder
    ) public {
        vm.startBroadcast(_privateKey);
        
        EASAttestTrigger trigger = EASAttestTrigger(vm.parseAddress(triggerAddr));
        
        // Example schema for skill verification
        bytes32 skillSchema = 0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefab00;
        address skillHolderAddr = vm.parseAddress(skillHolder);
        string memory skillData = "Verified: Advanced Solidity Development Skills";
        
        console.log("Creating skill verification attestation:");
        console.log("Skill holder:", skillHolder);
        console.log("Skill:", skillData);
        
        trigger.createAttestationTrigger(skillSchema, skillHolderAddr, skillData);
        
        ITypes.TriggerId triggerId = trigger.nextTriggerId();
        console.log("Skill Verification Trigger ID:", ITypes.TriggerId.unwrap(triggerId));
        
        vm.stopBroadcast();
    }
}