# EAS Deployment Guide

This document explains the Phase 2 implementation of EAS (Ethereum Attestation Service) contracts in the WAVS project, replacing the deprecated `WavsSubmit.sol` with a comprehensive EAS-based attestation system.

## Overview

The WAVS project has migrated from a simple data submission model to a full EAS integration that provides:

- **On-chain attestations** with verifiable schemas
- **Schema management** for different types of data
- **Resolver contracts** for custom attestation logic
- **Offchain verification** capabilities
- **Multi-attestation** support for batch operations

## New Contract Architecture

### Core EAS Contracts

1. **SchemaRegistry** - Manages attestation schemas
2. **EAS** - Main attestation contract
3. **Attester** - WAVS service handler (replaces WavsSubmit)
4. **SchemaRegistrar** - Helper for registering schemas
5. **LogResolver** - Resolver that emits events for attestations

### Deprecated Contracts

- ❌ `WavsSubmit.sol` - Replaced by `Attester.sol`
- ✅ `WavsTrigger.sol` - Still used for triggering events

## Deployment Scripts

### New Deployment Script

**Primary deployment script:** `script/deploy-eas-contracts.sh`

```bash
# Deploy all EAS contracts
bash ./script/deploy-eas-contracts.sh
```

This script will:
1. Deploy SchemaRegistry and EAS contracts
2. Deploy LogResolver for event handling
3. Deploy SchemaRegistrar for schema management  
4. Deploy Attester contract (main WAVS handler)
5. Register default schemas for basic and compute data
6. Deploy SimpleTrigger (unchanged)
7. Save deployment addresses to `.docker/deployment_summary.json`

### Legacy Script Compatibility

**Legacy script:** `script/deploy-contracts.sh` (now deprecated)

The old deployment script now forwards to the new EAS deployment script with a deprecation warning.

### Foundry Deployment Script

**Foundry script:** `script/DeployEAS.s.sol`

Low-level Foundry script used by the shell script for precise contract deployment and configuration.

## Environment Configuration

### New Environment Variables

Add these variables to your `.env` file after deployment:

```bash
# EAS Contract Addresses
EAS_REGISTRY_ADDR=0x...        # SchemaRegistry address
EAS_ADDR=0x...                 # EAS main contract address  
EAS_ATTESTER_ADDR=0x...        # Attester contract (WAVS handler)
EAS_SCHEMA_REGISTRAR_ADDR=0x... # Schema management contract
EAS_LOG_RESOLVER_ADDR=0x...     # Log resolver contract

# Service Contract Addresses
SERVICE_TRIGGER_ADDR=0x...      # SimpleTrigger address
SERVICE_SUBMISSION_ADDR=0x...   # Points to EAS_ATTESTER_ADDR

# Schema IDs
EAS_BASIC_SCHEMA=0x...          # Basic data schema UID
EAS_COMPUTE_SCHEMA=0x...        # Computation result schema UID
```

## Default Schemas

The deployment automatically registers two schemas:

### Basic Schema
```
bytes32 triggerId,string data,uint256 timestamp
```
For general WAVS data submissions.

### Compute Schema  
```
bytes32 triggerId,string computation,bytes result,uint256 timestamp,address operator
```
For computation results with operator attribution.

## Usage Examples

### Querying Attestations

Use the updated `ShowResult.s.sol` script:

```bash
# Query specific attestation
forge script script/ShowResult.s.sol:ShowResult \\
    --sig 'attestation(string,string)' \\
    "${EAS_ADDR}" "${ATTESTATION_UID}"

# Query attestations by schema
forge script script/ShowResult.s.sol:ShowResult \\
    --sig 'attestations(string,string,string)' \\
    "${EAS_ADDR}" "${SCHEMA_UID}" "${RECIPIENT_ADDR}"
```

### Creating Attestations

The `Attester.sol` contract provides multiple ways to create attestations:

1. **Direct attestation:**
   ```solidity
   attester.attest(schemaUID, recipient, data)
   ```

2. **Multi-attestation:**
   ```solidity
   attester.multiAttest(schemas, recipients, dataArray)
   ```

3. **WAVS handler (automatic):**
   ```solidity
   attester.handleSignedEnvelope(envelope, signature)
   ```

## Migration from WavsSubmit

### Key Differences

| WavsSubmit | EAS Attester |
|------------|--------------|
| Simple data storage | Schema-based attestations |
| Trigger ID mapping | EAS UID system |
| Basic validation | Full EAS verification |
| No standards | EIP-712 compatible |

### Code Changes Required

1. **Replace contract references:**
   ```solidity
   // Old
   SimpleSubmit submit = SimpleSubmit(address);
   
   // New  
   Attester attester = Attester(address);
   IEAS eas = IEAS(easAddress);
   ```

2. **Update data queries:**
   ```solidity
   // Old
   bytes memory data = submit.getData(triggerId);
   
   // New
   Attestation memory att = eas.getAttestation(uid);
   bytes memory data = att.data;
   ```

3. **Use environment variables:**
   ```bash
   # Old
   SERVICE_SUBMISSION_ADDR
   
   # New (backwards compatible)
   EAS_ATTESTER_ADDR  # or SERVICE_SUBMISSION_ADDR
   ```

## Deployment Files

After deployment, check these files:

- `.docker/deployment_summary.json` - Consolidated deployment info
- `.docker/eas_deploy.json` - Full Foundry deployment logs  
- `.docker/trigger.json` - SimpleTrigger deployment details

## Testing

All EAS contracts include comprehensive test suites:

```bash
# Run all tests
forge test

# Run specific EAS tests
forge test --match-contract Attester
forge test --match-contract SchemaRegistrar
forge test --match-contract LogResolver
```

## Next Steps

1. **Deploy contracts:** Run `bash ./script/deploy-eas-contracts.sh`
2. **Update environment:** Add contract addresses to `.env`
3. **Update WASI components:** Configure components to use new contract addresses
4. **Test integration:** Verify WAVS operators can submit to Attester contract
5. **Monitor attestations:** Use EAS explorer tools to view on-chain attestations

## Support

For issues with EAS deployment:
1. Check deployment logs in `.docker/eas_deploy.json`
2. Verify all environment variables are set
3. Ensure WAVS service manager is deployed and accessible
4. Run tests to verify contract functionality