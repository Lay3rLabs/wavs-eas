# EAS Attestation Component - Usage Examples

## Simplified Architecture

With the removal of the redundant `NewTrigger` event, the component now has a much cleaner architecture:

### Direct EAS Event Processing
```
EAS Contract → Attested Event → WAVS → Component → New Attestation
```

### Raw Data Processing  
```
Raw Data → Component → Formatted Attestation
```

## No More Trigger Wrapper Complexity

**Before (with NewTrigger)**:
```
Contract → NewTrigger(TriggerInfo) → WAVS → Component
```

**Now (simplified)**:
```
EAS → Attested Event → WAVS → Component
OR
Raw Data → Component
```

## Example Integration

### 1. Direct EAS Monitoring
WAVS can directly monitor EAS `Attested` events and trigger the component:

```solidity
// EAS contract emits this automatically
event Attested(
    address indexed recipient,
    address indexed attester, 
    bytes32 uid,
    bytes32 indexed schemaUID
);
```

### 2. Custom Data Input
For custom workflows, just pass attestation data directly:

```json
{
  "schema": "0x...",
  "recipient": "0x...", 
  "data": "Custom attestation content"
}
```

### 3. Raw Text Input
```bash
echo "Simple text attestation" | make wasi-exec
```

## Benefits of Simplified Approach

1. **Less Complexity**: No intermediate trigger events needed
2. **Direct Integration**: Works directly with EAS events  
3. **Cleaner Code**: Fewer types and simpler logic
4. **Better Performance**: Less event parsing overhead
5. **Easier Testing**: Direct data input without wrapping