# EAS Attestation Trigger Scripts

This directory contains scripts for triggering EAS attestation creation via WAVS components.

## Available Scripts

### 1. EASAttestTrigger.s.sol
**Purpose**: Dedicated EAS attestation trigger contract with clean API

**Functions**:
- `runJsonAttestation()` - Creates structured JSON attestation
- `runRawAttestation()` - Creates raw data attestation  
- `runTestimonialExample()` - Example testimonial attestation
- `runSkillVerificationExample()` - Example skill verification

**Usage Examples**:
```bash
# Create a testimonial attestation
forge script script/EASAttestTrigger.s.sol:EASAttestTriggerScript --sig "runTestimonialExample(string)" \
  "0x1234567890123456789012345678901234567890" --rpc-url $RPC_URL --broadcast

# Create a skill verification
forge script script/EASAttestTrigger.s.sol:EASAttestTriggerScript --sig "runSkillVerificationExample(string,string)" \
  "0x1234567890123456789012345678901234567890" \
  "0x5678901234567890123456789012345678901234" --rpc-url $RPC_URL --broadcast

# Create custom JSON attestation
forge script script/EASAttestTrigger.s.sol:EASAttestTriggerScript --sig "runJsonAttestation(string,string,string,string)" \
  "0x1234567890123456789012345678901234567890" \
  "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefab00" \
  "0x9876543210987654321098765432109876543210" \
  "This is my attestation data" --rpc-url $RPC_URL --broadcast

# Create raw data attestation
forge script script/EASAttestTrigger.s.sol:EASAttestTriggerScript --sig "runRawAttestation(string,string)" \
  "0x1234567890123456789012345678901234567890" \
  "Simple text attestation" --rpc-url $RPC_URL --broadcast
```

### 2. Trigger.s.sol (Updated)
**Purpose**: Updated existing trigger script with EAS support

**Functions**:
- `run()` - Backward compatible, now creates EAS attestations with default schema
- `runEAS()` - Custom EAS attestation with schema and recipient

**Usage Examples**:
```bash
# Backward compatible (creates EAS attestation with defaults)
forge script script/Trigger.s.sol:Trigger --sig "run(string,string)" \
  "0x1234567890123456789012345678901234567890" \
  "My simple attestation" --rpc-url $RPC_URL --broadcast

# Custom EAS attestation
forge script script/Trigger.s.sol:Trigger --sig "runEAS(string,string,string,string)" \
  "0x1234567890123456789012345678901234567890" \
  "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefab00" \
  "0x9876543210987654321098765432109876543210" \
  "Custom attestation data" --rpc-url $RPC_URL --broadcast
```

## JSON Format

The scripts create JSON payloads that the EAS attestation component can parse:

```json
{
  "schema": "0xabcdef...",
  "recipient": "0x123456...", 
  "data": "Attestation content",
  "expiration_time": 0,
  "revocable": true
}
```

## Integration with WAVS

1. **Deploy Trigger Contract**: Deploy `EASAttestTrigger.sol` or use existing `WavsTrigger.sol`
2. **Configure WAVS**: Point WAVS to monitor the trigger contract
3. **Run Scripts**: Execute trigger scripts to create attestation requests
4. **WAVS Processing**: WAVS picks up trigger data and routes to EAS attestation component
5. **Component Output**: Component formats attestation data for EAS submission

## No NewTrigger Events

The system has been simplified to remove redundant `NewTrigger` event wrappers:

**Before**:
```
Script → NewTrigger(TriggerInfo) → WAVS → Component
```

**Now**:
```  
Script → Raw Data → WAVS → Component
```

This makes the system more efficient and easier to understand.