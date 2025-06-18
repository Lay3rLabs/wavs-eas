// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {ISimpleTrigger} from "interfaces/IWavsTrigger.sol";

/// @title EASAttestTrigger
/// @notice Simplified trigger contract for EAS attestation creation
/// @dev This contract allows direct attestation data submission to WAVS for processing
contract EASAttestTrigger is ISimpleTrigger {
    /// @inheritdoc ISimpleTrigger
    TriggerId public nextTriggerId = TriggerId.wrap(1);

    /// @inheritdoc ISimpleTrigger
    mapping(TriggerId _triggerId => Trigger _trigger) public triggersById;
    
    /// @notice Internal mapping for triggers by creator
    mapping(address _creator => TriggerId[] _triggerIds) internal _triggerIdsByCreator;

    /// @notice EAS Attested event that WAVS component expects
    event AttestedEvent(
        address indexed recipient,
        address indexed attester,
        bytes32 uid,
        bytes32 indexed schema_uid
    );

    /// @notice Event for direct attestation requests (no wrapper events)
    /// @param creator Address requesting the attestation
    /// @param schema Schema UID for the attestation
    /// @param recipient Recipient of the attestation
    /// @param data Attestation data
    event AttestationRequested(
        address indexed creator,
        bytes32 indexed schema,
        address indexed recipient,
        bytes data
    );

    /// @notice Creates an attestation trigger with JSON data
    /// @param schema The schema UID for the attestation
    /// @param recipient The recipient address (use zero address for no specific recipient)
    /// @param data The attestation data as string
    function createAttestationTrigger(
        bytes32 schema,
        address recipient,
        string calldata data
    ) external {
        // Create JSON payload for the EAS component
        string memory jsonPayload = string(abi.encodePacked(
            '{"schema":"', _bytes32ToHex(schema), 
            '","recipient":"', _addressToHex(recipient),
            '","data":"', data, 
            '","expiration_time":0,"revocable":true}'
        ));

        bytes memory triggerData = bytes(jsonPayload);
        
        // Store trigger using inherited interface
        _addTrigger(triggerData);

        // Generate a unique UID for this attestation request
        bytes32 uid = keccak256(abi.encodePacked(nextTriggerId, block.timestamp, msg.sender));

        // Emit AttestedEvent that the EAS component expects
        emit AttestedEvent(recipient, msg.sender, uid, schema);

        // Emit simplified event for monitoring
        emit AttestationRequested(msg.sender, schema, recipient, triggerData);
    }

    /// @notice Creates an attestation trigger with raw bytes data
    /// @param data Raw attestation data (will use component defaults)
    function createRawAttestationTrigger(bytes calldata data) external {
        _addTrigger(data);

        // Generate a unique UID for this attestation request
        bytes32 uid = keccak256(abi.encodePacked(nextTriggerId, block.timestamp, msg.sender));
        
        // Emit AttestedEvent with default values for raw data
        emit AttestedEvent(address(0), msg.sender, uid, bytes32(0));
        
        emit AttestationRequested(msg.sender, bytes32(0), address(0), data);
    }

    /// @inheritdoc ISimpleTrigger
    function addTrigger(bytes memory _data) external {
        _addTrigger(_data);

        // Generate a unique UID for this attestation request
        bytes32 uid = keccak256(abi.encodePacked(nextTriggerId, block.timestamp, msg.sender));
        
        // Emit AttestedEvent with default values for generic trigger
        emit AttestedEvent(address(0), msg.sender, uid, bytes32(0));
    }

    /// @inheritdoc ISimpleTrigger
    function getTrigger(TriggerId triggerId) external view override returns (TriggerInfo memory _triggerInfo) {
        Trigger storage _trigger = triggersById[triggerId];
        _triggerInfo = TriggerInfo({
            triggerId: triggerId, 
            creator: _trigger.creator, 
            data: _trigger.data
        });
    }

    /// @inheritdoc ISimpleTrigger
    function triggerIdsByCreator(address _creator) external view returns (TriggerId[] memory _triggerIds) {
        _triggerIds = _triggerIdsByCreator[_creator];
    }

    /// @notice Internal function to add trigger and emit raw data event
    /// @param _data The trigger data
    function _addTrigger(bytes memory _data) internal {
        // Increment trigger ID
        nextTriggerId = TriggerId.wrap(TriggerId.unwrap(nextTriggerId) + 1);
        TriggerId _triggerId = nextTriggerId;

        // Create the trigger
        Trigger memory _trigger = Trigger({creator: msg.sender, data: _data});

        // Update storage
        triggersById[_triggerId] = _trigger;
        _triggerIdsByCreator[msg.sender].push(_triggerId);

        // WAVS monitors raw data events - no NewTrigger wrapper needed
        // Component processes _data directly as raw TriggerData
    }

    /// @notice Converts bytes32 to hex string
    /// @param value The bytes32 value to convert
    /// @return Hex string representation
    function _bytes32ToHex(bytes32 value) internal pure returns (string memory) {
        bytes memory alphabet = "0123456789abcdef";
        bytes memory str = new bytes(66);
        str[0] = '0';
        str[1] = 'x';
        for (uint256 i = 0; i < 32; i++) {
            str[2 + i * 2] = alphabet[uint8(value[i] >> 4)];
            str[3 + i * 2] = alphabet[uint8(value[i] & 0x0f)];
        }
        return string(str);
    }

    /// @notice Converts address to hex string
    /// @param addr The address to convert
    /// @return Hex string representation
    function _addressToHex(address addr) internal pure returns (string memory) {
        bytes memory alphabet = "0123456789abcdef";
        bytes memory str = new bytes(42);
        str[0] = '0';
        str[1] = 'x';
        for (uint256 i = 0; i < 20; i++) {
            str[2 + i * 2] = alphabet[uint8(uint160(addr) >> (8 * (19 - i) + 4))];
            str[3 + i * 2] = alphabet[uint8(uint160(addr) >> (8 * (19 - i)))];
        }
        return string(str);
    }
}