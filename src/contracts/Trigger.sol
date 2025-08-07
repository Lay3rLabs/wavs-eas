// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

/// @title EASAttestTrigger
/// @notice Simplified trigger contract for WAVS EAS testing
/// @dev This contract allows direct attestation data submission to WAVS for testing
contract EASAttestTrigger {
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
    function triggerRequestAttestation(
        bytes32 schema,
        address recipient,
        string calldata data
    ) external {
        // Create JSON payload for the EAS component
        string memory jsonPayload = string(
            abi.encodePacked(
                '{"schema":"',
                _bytes32ToHex(schema),
                '","recipient":"',
                _addressToHex(recipient),
                '","data":"',
                data,
                '","expiration_time":0,"revocable":true}'
            )
        );

        bytes memory triggerData = bytes(jsonPayload);

        // Emit AttestationRequested event that the EAS component expects
        emit AttestationRequested(msg.sender, schema, recipient, triggerData);
    }

    /// @notice Creates an attestation trigger with raw bytes data
    /// @param data Raw attestation data (will use component defaults)
    function triggerRequestRawAttestation(
        bytes32 schema,
        address recipient,
        bytes calldata data
    ) external {
        // Emit AttestedEvent with default values for raw data
        emit AttestationRequested(msg.sender, schema, recipient, data);
    }

    /// @notice Converts bytes32 to hex string
    /// @param value The bytes32 value to convert
    /// @return Hex string representation
    function _bytes32ToHex(
        bytes32 value
    ) internal pure returns (string memory) {
        bytes memory alphabet = "0123456789abcdef";
        bytes memory str = new bytes(66);
        str[0] = "0";
        str[1] = "x";
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
        str[0] = "0";
        str[1] = "x";
        for (uint256 i = 0; i < 20; i++) {
            str[2 + i * 2] = alphabet[
                uint8(uint160(addr) >> (8 * (19 - i) + 4))
            ];
            str[3 + i * 2] = alphabet[uint8(uint160(addr) >> (8 * (19 - i)))];
        }
        return string(str);
    }
}
