// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {ISimpleTrigger} from "interfaces/IWavsTrigger.sol";

contract SimpleTrigger is ISimpleTrigger {
    /// @notice EAS Attested event that WAVS component expects
    event AttestedEvent(
        address indexed recipient,
        address indexed attester,
        bytes32 uid,
        bytes32 indexed schema_uid
    );
    /// @inheritdoc ISimpleTrigger
    TriggerId public nextTriggerId;

    /// @inheritdoc ISimpleTrigger
    mapping(TriggerId _triggerId => Trigger _trigger) public triggersById;
    /// @notice See ISimpleTrigger.triggerIdsByCreator
    mapping(address _creator => TriggerId[] _triggerIds) internal _triggerIdsByCreator;

    /// @inheritdoc ISimpleTrigger
    function addTrigger(bytes memory _data) external {
        // Get the next trigger id
        nextTriggerId = TriggerId.wrap(TriggerId.unwrap(nextTriggerId) + 1);
        TriggerId _triggerId = nextTriggerId;

        // Create the trigger
        Trigger memory _trigger = Trigger({creator: msg.sender, data: _data});

        // Update storages
        triggersById[_triggerId] = _trigger;
        _triggerIdsByCreator[msg.sender].push(_triggerId);

        // Emit AttestedEvent that the EAS component expects
        // Generate a mock UID from trigger ID and data hash
        bytes32 uid = keccak256(abi.encodePacked(_triggerId, block.timestamp));
        bytes32 schema_uid = bytes32(0); // Default schema, can be configured
        
        emit AttestedEvent(
            address(0), // recipient (can be extracted from _data if needed)
            msg.sender, // attester is the caller
            uid,
            schema_uid
        );
    }

    /// @inheritdoc ISimpleTrigger
    function getTrigger(TriggerId triggerId) external view override returns (TriggerInfo memory _triggerInfo) {
        Trigger storage _trigger = triggersById[triggerId];
        _triggerInfo = TriggerInfo({triggerId: triggerId, creator: _trigger.creator, data: _trigger.data});
    }

    /// @inheritdoc ISimpleTrigger
    function triggerIdsByCreator(address _creator) external view returns (TriggerId[] memory _triggerIds) {
        _triggerIds = _triggerIdsByCreator[_creator];
    }
}
