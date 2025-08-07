# The Plan

## FSV (get a basic thing working)
Fix eas-attest component and Attester.sol contract:
- [x] Response should be an AttestationRequest
- [x] Trigger should be AttestationRequested
- [x] Script to create a new schema
- [ ] Clean up trigger script
- [ ] Realistic example schema

## Multi-component
Fix eas-compute component:
- [ ] Triggered by new attestations
- [ ] Update it to make it work with a particular use case in the spec

Deployment improvements (tackle after fixing eas-attest component):
- [] Fix up deployment so two services are deployed (eas-attest and eas-compute)
- [] Clean up trigger script and function naming

## Service Improvements
- [] Implement a real use case
- [] Add eas-indexer sidecar to docker compose
- [] Add WAVS workflows
