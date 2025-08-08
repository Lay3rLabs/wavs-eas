# WAVS-EAS Deployment Guide

This guide explains how to deploy the complete WAVS-EAS infrastructure with automatic attestation indexing capabilities.

## Overview

The WAVS-EAS deployment includes the following components:

### Core EAS Infrastructure
- **SchemaRegistry**: Manages attestation schemas
- **EAS**: Main Ethereum Attestation Service contract
- **Indexer**: Provides efficient querying of attestations
- **LogResolver**: Simple resolver for logging attestation events
- **IndexerResolver**: Automatically indexes attestations on creation

### WAVS Integration
- **Attester**: Main WAVS integration contract for creating attestations
- **SchemaRegistrar**: Utility for registering new schemas
- **EASAttestTrigger**: Trigger contract for WAVS workflows

### Pre-configured Schemas
- **Basic Schema**: For general attestation data with automatic indexing
- **Compute Schema**: For computation results with automatic indexing

## Prerequisites

1. **Foundry** installed and configured
2. **Private key** with sufficient funds for deployment
3. **WAVS Service Manager** address (required for Attester deployment)
4. **RPC endpoint** configured in your environment

## Deployment

### Environment Setup

```bash
# Set your private key (use .env file in production)
export PRIVATE_KEY=0x...

# Set RPC URL
export RPC_URL=https://your-rpc-endpoint

# Get your WAVS Service Manager address
export WAVS_SERVICE_MANAGER=0x...
```

### Deploy All Contracts

```bash
# Deploy the complete EAS infrastructure
forge script script/DeployEAS.s.sol:DeployEAS \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --verify \
    $WAVS_SERVICE_MANAGER
```

### Expected Output

The deployment will output addresses for all deployed contracts:

```
=== EAS Deployment Summary ===
SchemaRegistry: 0x...
EAS: 0x...
Attester: 0x...
SchemaRegistrar: 0x...
LogResolver: 0x...
Indexer: 0x...
IndexerResolver: 0x...
EASAttestTrigger: 0x...
Basic Schema ID: 0x...
Compute Schema ID: 0x...
```

## Usage Examples

### Creating Auto-Indexed Attestations

Once deployed, attestations created using the pre-configured schemas will be automatically indexed:

```solidity
// Using the Basic Schema
bytes32 uid = eas.attest(
    AttestationRequest({
        schema: basicSchemaId,
        data: AttestationRequestData({
            recipient: recipientAddress,
            expirationTime: 0,
            revocable: true,
            refUID: bytes32(0),
            data: abi.encode(
                triggerId,     // bytes32
                "sample data", // string
                block.timestamp // uint256
            ),
            value: 0
        })
    })
);

// The attestation is now automatically indexed!
// Query it using the Indexer
bool isIndexed = indexer.isAttestationIndexed(uid); // returns true
```

### Querying Indexed Attestations

```solidity
// Get all attestations for a schema
bytes32[] memory attestations = indexer.getSchemaAttestationUIDs(
    basicSchemaId,
    0,    // start offset
    10,   // count
    false // reverse order
);

// Get attestations received by a specific address
bytes32[] memory received = indexer.getReceivedAttestationUIDs(
    recipientAddress,
    basicSchemaId,
    0,
    10,
    false
);

// Get attestations sent by a specific attester
bytes32[] memory sent = indexer.getSentAttestationUIDs(
    attesterAddress,
    basicSchemaId,
    0,
    10,
    false
);
```

### Using the WAVS Attester Contract

```solidity
// The Attester contract provides WAVS-specific functionality
Attester attester = Attester(attesterAddress);

// Create attestations through the WAVS workflow
// (specific methods depend on your WAVS integration needs)
```

## Verification

### Verify Deployment Success

1. **Check contract addresses**: All addresses should be non-zero
2. **Verify indexer configuration**:
   ```bash
   cast call $INDEXER_ADDRESS "getEAS()" --rpc-url $RPC_URL
   # Should return the EAS contract address
   ```

3. **Verify schema registration**:
   ```bash
   cast call $SCHEMA_REGISTRY "getSchema(bytes32)" $BASIC_SCHEMA_ID --rpc-url $RPC_URL
   # Should return schema details with IndexerResolver as resolver
   ```

4. **Test automatic indexing**:
   ```bash
   # Create a test attestation and verify it gets indexed
   # (See test files for examples)
   ```

### Run Integration Tests

```bash
# Test the deployment
forge test --match-contract DeployEASTest -v

# Test IndexerResolver functionality
forge test --match-contract IndexerResolverTest -v

# Test existing functionality remains intact
forge test --match-contract LogResolverTest -v
```

## Advanced Configuration

### Custom Schemas with Indexing

To create new schemas that automatically index attestations:

```solidity
// Register a new schema with IndexerResolver
bytes32 customSchemaId = schemaRegistrar.register(
    "uint256 value,address user,string description",
    ISchemaResolver(indexerResolverAddress),
    true // revocable
);
```

### Using LogResolver for Simple Logging

If you don't need indexing but want event logging:

```solidity
// Register schema with LogResolver instead
bytes32 loggingSchemaId = schemaRegistrar.register(
    "uint256 value",
    ISchemaResolver(logResolverAddress),
    true
);
```

## Gas Considerations

### Indexing Overhead

- **Automatic indexing** adds ~200k gas per attestation
- **Query benefits** significantly outweigh the creation overhead
- **Historical data** remains accessible even after revocation

### Optimization Tips

1. **Batch operations** when possible
2. **Consider indexing needs** when designing schemas
3. **Use LogResolver** for simple use cases that don't require querying

## Troubleshooting

### Common Issues

1. **"Invalid service manager address"**
   - Ensure WAVS Service Manager address is correct and non-zero

2. **Gas estimation failures**
   - Increase gas limit for deployment transactions
   - Check account has sufficient funds

3. **Schema registration failures**
   - Verify resolver addresses are correct
   - Check schema syntax is valid

### Getting Help

1. **Run tests** to verify local functionality works
2. **Check logs** from deployment for specific error messages
3. **Verify** all prerequisites are met

## Security Considerations

1. **Private key management**: Never commit private keys to source control
2. **Contract verification**: Always verify contracts on block explorers
3. **Access controls**: Review who can create attestations in your schemas
4. **Upgrade paths**: Consider how contracts might need to be upgraded

## Next Steps

After deployment:

1. **Configure WAVS workflows** to use the deployed trigger contracts
2. **Set up monitoring** for attestation events
3. **Build applications** that leverage the indexed attestation data
4. **Consider governance** for schema management and upgrades

For more detailed information about individual contracts, see the contract-specific documentation in `src/contracts/`.