// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {ISchemaRegistry, SchemaRegistry} from "@ethereum-attestation-service/eas-contracts/contracts/SchemaRegistry.sol";
import {IEAS, EAS} from "@ethereum-attestation-service/eas-contracts/contracts/EAS.sol";
import {ISchemaResolver} from "@ethereum-attestation-service/eas-contracts/contracts/resolver/ISchemaResolver.sol";
import {Attester} from "../src/contracts/Attester.sol";
import {SchemaRegistrar} from "../src/contracts/SchemaRegistrar.sol";
import {LogResolver} from "../src/contracts/LogResolver.sol";
import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";
import {Common} from "./Common.s.sol";

/// @title DeployEAS
/// @notice Deployment script for EAS contracts and WAVS EAS integration
contract DeployEAS is Common {
    struct EASDeployment {
        address schemaRegistry;
        address eas;
        address attester;
        address schemaRegistrar;
        address logResolver;
        bytes32 basicSchema;
        bytes32 computeSchema;
    }

    /// @notice Deploy EAS contracts and WAVS integration
    /// @param wavsServiceManagerAddr The WAVS service manager address
    /// @return deployment The deployed contract addresses
    function run(
        string calldata wavsServiceManagerAddr
    ) public returns (EASDeployment memory deployment) {
        vm.startBroadcast(_privateKey);

        address serviceManager = vm.parseAddress(wavsServiceManagerAddr);
        require(
            serviceManager != address(0),
            "Invalid service manager address"
        );

        console.log("Deploying EAS contracts...");

        // 1. Deploy SchemaRegistry
        SchemaRegistry schemaRegistry = new SchemaRegistry();
        deployment.schemaRegistry = address(schemaRegistry);
        console.log("SchemaRegistry deployed at:", deployment.schemaRegistry);

        // 2. Deploy EAS
        EAS eas = new EAS(ISchemaRegistry(deployment.schemaRegistry));
        deployment.eas = address(eas);
        console.log("EAS deployed at:", deployment.eas);

        // 3. Deploy LogResolver
        LogResolver logResolver = new LogResolver(IEAS(deployment.eas));
        deployment.logResolver = address(logResolver);
        console.log("LogResolver deployed at:", deployment.logResolver);

        // 4. Deploy SchemaRegistrar
        SchemaRegistrar schemaRegistrar = new SchemaRegistrar(
            ISchemaRegistry(deployment.schemaRegistry)
        );
        deployment.schemaRegistrar = address(schemaRegistrar);
        console.log("SchemaRegistrar deployed at:", deployment.schemaRegistrar);

        // 5. Deploy Attester (main WAVS integration contract)
        Attester attester = new Attester(
            IEAS(deployment.eas),
            IWavsServiceManager(serviceManager)
        );
        deployment.attester = address(attester);
        console.log("Attester deployed at:", deployment.attester);

        // 6. Register basic schemas
        console.log("Registering schemas...");

        // Basic attestation schema for general data
        deployment.basicSchema = schemaRegistrar.register(
            "bytes32 triggerId,string data,uint256 timestamp",
            ISchemaResolver(deployment.logResolver),
            true // revocable
        );
        console.log(
            "Basic schema registered:",
            vm.toString(deployment.basicSchema)
        );

        // Compute result schema for computation results
        deployment.computeSchema = schemaRegistrar.register(
            "bytes32 triggerId,string computation,bytes result,uint256 timestamp,address operator",
            ISchemaResolver(deployment.logResolver),
            true // revocable
        );
        console.log(
            "Compute schema registered:",
            vm.toString(deployment.computeSchema)
        );

        vm.stopBroadcast();

        // Log deployment summary
        console.log("\n=== EAS Deployment Summary ===");
        console.log("SchemaRegistry:", deployment.schemaRegistry);
        console.log("EAS:", deployment.eas);
        console.log("Attester:", deployment.attester);
        console.log("SchemaRegistrar:", deployment.schemaRegistrar);
        console.log("LogResolver:", deployment.logResolver);
        console.log("Basic Schema ID:", vm.toString(deployment.basicSchema));
        console.log(
            "Compute Schema ID:",
            vm.toString(deployment.computeSchema)
        );
    }
}
