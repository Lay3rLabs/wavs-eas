// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Test} from "forge-std/Test.sol";
import {Attester} from "../../src/contracts/Attester.sol";
import {LogResolver} from "../../src/contracts/LogResolver.sol";
import {EAS} from "@ethereum-attestation-service/eas-contracts/contracts/EAS.sol";
import {SchemaRegistry} from "@ethereum-attestation-service/eas-contracts/contracts/SchemaRegistry.sol";
import {IEAS} from "@ethereum-attestation-service/eas-contracts/contracts/IEAS.sol";
import {ISchemaRegistry} from "@ethereum-attestation-service/eas-contracts/contracts/ISchemaRegistry.sol";

contract AttesterTest is Test {
    Attester public attester;
    LogResolver public resolver;
    EAS public eas;
    SchemaRegistry public schemaRegistry;

    string constant SCHEMA = "uint256 value";
    string constant SCHEMA2 = "uint256 value2";
    bytes32 public schemaId;
    bytes32 public schemaId2;

    address constant ZERO_ADDRESS = address(0);

    function setUp() public {
        // Deploy contracts
        schemaRegistry = new SchemaRegistry();
        eas = new EAS(ISchemaRegistry(address(schemaRegistry)));
        resolver = new LogResolver(IEAS(address(eas)));
        attester = new Attester(IEAS(address(eas)));

        // Register schemas
        schemaId = schemaRegistry.register(SCHEMA, resolver, true);
        schemaId2 = schemaRegistry.register(SCHEMA2, resolver, true);
    }

    function testConstruction_ShouldRevertWithInvalidEAS() public {
        vm.expectRevert(Attester.InvalidEAS.selector);
        new Attester(IEAS(ZERO_ADDRESS));
    }

    function testAttest_ShouldLogAttestedValue() public {
        uint256 value = 123456;

        vm.expectEmit(true, true, true, true);
        emit LogResolver.Attested(schemaId, value);

        bytes32 uid = attester.attest(schemaId, value);

        // Verify the attestation was created
        assertTrue(uid != bytes32(0));
    }

    function testMultiAttest_ShouldLogAttestedValues() public {
        bytes32[] memory schemas = new bytes32[](2);
        schemas[0] = schemaId;
        schemas[1] = schemaId2;

        uint256[][] memory inputs = new uint256[][](2);
        inputs[0] = new uint256[](3);
        inputs[0][0] = 10;
        inputs[0][1] = 100;
        inputs[0][2] = 123456;

        inputs[1] = new uint256[](2);
        inputs[1][0] = 5;
        inputs[1][1] = 23423234;

        // Expect all attestation events
        vm.expectEmit(true, true, true, true);
        emit LogResolver.Attested(schemaId, 10);
        vm.expectEmit(true, true, true, true);
        emit LogResolver.Attested(schemaId, 100);
        vm.expectEmit(true, true, true, true);
        emit LogResolver.Attested(schemaId, 123456);
        vm.expectEmit(true, true, true, true);
        emit LogResolver.Attested(schemaId2, 5);
        vm.expectEmit(true, true, true, true);
        emit LogResolver.Attested(schemaId2, 23423234);

        bytes32[] memory uids = attester.multiAttest(schemas, inputs);

        // Verify attestations were created
        assertEq(uids.length, 5); // 3 + 2 attestations
        for (uint256 i = 0; i < uids.length; i++) {
            assertTrue(uids[i] != bytes32(0));
        }
    }

    function testMultiAttest_ShouldRevertWithEmptySchemas() public {
        bytes32[] memory schemas = new bytes32[](0);
        uint256[][] memory inputs = new uint256[][](0);

        vm.expectRevert(Attester.InvalidInput.selector);
        attester.multiAttest(schemas, inputs);
    }

    function testMultiAttest_ShouldRevertWithMismatchedArrays() public {
        bytes32[] memory schemas = new bytes32[](1);
        schemas[0] = schemaId;

        uint256[][] memory inputs = new uint256[][](2);
        inputs[0] = new uint256[](1);
        inputs[0][0] = 10;
        inputs[1] = new uint256[](1);
        inputs[1][0] = 20;

        vm.expectRevert(Attester.InvalidInput.selector);
        attester.multiAttest(schemas, inputs);
    }

    function testRevoke_ShouldHandleRevoke() public {
        uint256 value = 999;

        // First create an attestation
        bytes32 uid = attester.attest(schemaId, value);

        // Then revoke it
        vm.expectEmit(true, true, true, true);
        emit LogResolver.Revoked(schemaId, value);

        attester.revoke(schemaId, uid);
    }

    function testMultiRevoke_ShouldHandleMultipleRevokes() public {
        // First create multiple attestations
        bytes32[] memory schemas = new bytes32[](2);
        schemas[0] = schemaId;
        schemas[1] = schemaId2;

        uint256[][] memory inputs = new uint256[][](2);
        inputs[0] = new uint256[](3);
        inputs[0][0] = 10;
        inputs[0][1] = 100;
        inputs[0][2] = 123456;

        inputs[1] = new uint256[](2);
        inputs[1][0] = 5;
        inputs[1][1] = 23423234;

        bytes32[] memory uids = attester.multiAttest(schemas, inputs);

        // Prepare revocation data
        bytes32[][] memory schemaUids = new bytes32[][](2);
        schemaUids[0] = new bytes32[](3);
        schemaUids[0][0] = uids[0];
        schemaUids[0][1] = uids[1];
        schemaUids[0][2] = uids[2];

        schemaUids[1] = new bytes32[](2);
        schemaUids[1][0] = uids[3];
        schemaUids[1][1] = uids[4];

        // Expect all revocation events
        vm.expectEmit(true, true, true, true);
        emit LogResolver.Revoked(schemaId, 10);
        vm.expectEmit(true, true, true, true);
        emit LogResolver.Revoked(schemaId, 100);
        vm.expectEmit(true, true, true, true);
        emit LogResolver.Revoked(schemaId, 123456);
        vm.expectEmit(true, true, true, true);
        emit LogResolver.Revoked(schemaId2, 5);
        vm.expectEmit(true, true, true, true);
        emit LogResolver.Revoked(schemaId2, 23423234);

        // Revoke all attestations
        attester.multiRevoke(schemas, schemaUids);
    }

    function testMultiRevoke_ShouldRevertWithEmptySchemas() public {
        bytes32[] memory schemas = new bytes32[](0);
        bytes32[][] memory schemaUids = new bytes32[][](0);

        vm.expectRevert(Attester.InvalidInput.selector);
        attester.multiRevoke(schemas, schemaUids);
    }

    function testMultiRevoke_ShouldRevertWithMismatchedArrays() public {
        bytes32[] memory schemas = new bytes32[](1);
        schemas[0] = schemaId;

        bytes32[][] memory schemaUids = new bytes32[][](2);
        schemaUids[0] = new bytes32[](1);
        schemaUids[0][0] = bytes32(uint256(1));
        schemaUids[1] = new bytes32[](1);
        schemaUids[1][0] = bytes32(uint256(2));

        vm.expectRevert(Attester.InvalidInput.selector);
        attester.multiRevoke(schemas, schemaUids);
    }
}
