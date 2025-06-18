// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test} from "forge-std/Test.sol";
import {LogResolver} from "../../src/contracts/LogResolver.sol";
import {EAS} from "@ethereum-attestation-service/eas-contracts/contracts/EAS.sol";
import {SchemaRegistry} from "@ethereum-attestation-service/eas-contracts/contracts/SchemaRegistry.sol";
import {
    IEAS,
    AttestationRequest,
    AttestationRequestData,
    RevocationRequest,
    RevocationRequestData
} from "@ethereum-attestation-service/eas-contracts/contracts/IEAS.sol";
import {ISchemaRegistry} from "@ethereum-attestation-service/eas-contracts/contracts/ISchemaRegistry.sol";
import {NO_EXPIRATION_TIME, EMPTY_UID} from "@ethereum-attestation-service/eas-contracts/contracts/Common.sol";

contract LogResolverTest is Test {
    LogResolver public resolver;
    EAS public eas;
    SchemaRegistry public schemaRegistry;

    string constant SCHEMA = "uint256 value";
    bytes32 public schemaId;

    address constant ZERO_ADDRESS = address(0);

    function setUp() public {
        // Deploy contracts
        schemaRegistry = new SchemaRegistry();
        eas = new EAS(ISchemaRegistry(address(schemaRegistry)));
        resolver = new LogResolver(IEAS(address(eas)));

        // Register schema with the resolver
        schemaId = schemaRegistry.register(SCHEMA, resolver, true);
    }

    function testConstruction_ShouldInitializeCorrectly() public {
        // Verify the resolver was created successfully
        assertTrue(address(resolver) != address(0));
    }

    function testOnAttest_ShouldEmitAttestedEvent() public {
        uint256 testValue = 12345;

        // Expect the Attested event to be emitted
        vm.expectEmit(true, true, true, true);
        emit LogResolver.Attested(schemaId, testValue);

        // Create an attestation which will trigger the resolver
        eas.attest(
            AttestationRequest({
                schema: schemaId,
                data: AttestationRequestData({
                    recipient: address(0),
                    expirationTime: NO_EXPIRATION_TIME,
                    revocable: true,
                    refUID: EMPTY_UID,
                    data: abi.encode(testValue),
                    value: 0
                })
            })
        );
    }

    function testOnAttest_ShouldHandleMultipleValues() public {
        uint256[] memory values = new uint256[](3);
        values[0] = 100;
        values[1] = 200;
        values[2] = 300;

        for (uint256 i = 0; i < values.length; i++) {
            vm.expectEmit(true, true, true, true);
            emit LogResolver.Attested(schemaId, values[i]);

            eas.attest(
                AttestationRequest({
                    schema: schemaId,
                    data: AttestationRequestData({
                        recipient: address(0),
                        expirationTime: NO_EXPIRATION_TIME,
                        revocable: true,
                        refUID: EMPTY_UID,
                        data: abi.encode(values[i]),
                        value: 0
                    })
                })
            );
        }
    }

    function testOnRevoke_ShouldEmitRevokedEvent() public {
        uint256 testValue = 54321;

        // First create an attestation
        bytes32 uid = eas.attest(
            AttestationRequest({
                schema: schemaId,
                data: AttestationRequestData({
                    recipient: address(0),
                    expirationTime: NO_EXPIRATION_TIME,
                    revocable: true,
                    refUID: EMPTY_UID,
                    data: abi.encode(testValue),
                    value: 0
                })
            })
        );

        // Now revoke it and expect the Revoked event
        vm.expectEmit(true, true, true, true);
        emit LogResolver.Revoked(schemaId, testValue);

        eas.revoke(RevocationRequest({schema: schemaId, data: RevocationRequestData({uid: uid, value: 0})}));
    }

    function testOnRevoke_ShouldHandleMultipleRevocations() public {
        uint256[] memory values = new uint256[](2);
        values[0] = 777;
        values[1] = 888;

        bytes32[] memory uids = new bytes32[](2);

        // Create attestations
        for (uint256 i = 0; i < values.length; i++) {
            uids[i] = eas.attest(
                AttestationRequest({
                    schema: schemaId,
                    data: AttestationRequestData({
                        recipient: address(0),
                        expirationTime: NO_EXPIRATION_TIME,
                        revocable: true,
                        refUID: EMPTY_UID,
                        data: abi.encode(values[i]),
                        value: 0
                    })
                })
            );
        }

        // Revoke them and check events
        for (uint256 i = 0; i < values.length; i++) {
            vm.expectEmit(true, true, true, true);
            emit LogResolver.Revoked(schemaId, values[i]);

            eas.revoke(RevocationRequest({schema: schemaId, data: RevocationRequestData({uid: uids[i], value: 0})}));
        }
    }

    function testOnAttest_ShouldDecodeCorrectly() public {
        // Test with edge case values
        uint256[] memory testValues = new uint256[](4);
        testValues[0] = 0;
        testValues[1] = 1;
        testValues[2] = type(uint256).max;
        testValues[3] = 2 ** 128 - 1;

        for (uint256 i = 0; i < testValues.length; i++) {
            vm.expectEmit(true, true, true, true);
            emit LogResolver.Attested(schemaId, testValues[i]);

            eas.attest(
                AttestationRequest({
                    schema: schemaId,
                    data: AttestationRequestData({
                        recipient: address(0),
                        expirationTime: NO_EXPIRATION_TIME,
                        revocable: true,
                        refUID: EMPTY_UID,
                        data: abi.encode(testValues[i]),
                        value: 0
                    })
                })
            );
        }
    }

    function testFullAttestationRevocationCycle() public {
        uint256 testValue = 999;

        // Expect attestation event
        vm.expectEmit(true, true, true, true);
        emit LogResolver.Attested(schemaId, testValue);

        // Create attestation
        bytes32 uid = eas.attest(
            AttestationRequest({
                schema: schemaId,
                data: AttestationRequestData({
                    recipient: address(0),
                    expirationTime: NO_EXPIRATION_TIME,
                    revocable: true,
                    refUID: EMPTY_UID,
                    data: abi.encode(testValue),
                    value: 0
                })
            })
        );

        // Verify attestation exists
        assertTrue(eas.isAttestationValid(uid));

        // Expect revocation event
        vm.expectEmit(true, true, true, true);
        emit LogResolver.Revoked(schemaId, testValue);

        // Revoke attestation
        eas.revoke(RevocationRequest({schema: schemaId, data: RevocationRequestData({uid: uid, value: 0})}));
    }
}
