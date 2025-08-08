# The Plan

## FSV (get a basic thing working)
Fix eas-attest component and Attester.sol contract:
- [x] Response should be an AttestationRequest
- [x] Trigger should be AttestationRequested
- [x] Script to create a new schema
- [x] Clean up trigger script
- [x] Figure out how to query attestations (with indexer)
- [x] Remove log resolver
- [x] Make sure attestations are being indexed
- [ ] Add Governor contract example with custom votes extension?
- [ ] Realistic vouching schema
- [ ] Example wavs component that loads attestations for a recipient and does compute over them to calculate voting power


## Multi-component
Fix eas-compute component:
- [ ] Triggered by new attestations? Something else?
- [ ] Update it to make it work with a particular use case in the spec

Deployment improvements (tackle after fixing eas-attest component):
- [] Fix up deployment so two or more services are deployed (eas-attest and eas-compute)

## Service Improvements
- [] Implement a real use case (governance, incentives payout)
- [] Add eas-indexer sidecar to docker compose
- [] EAS indexer component (indexes certain attestations without a resolver? Might even do this in a cross-chain way)
- [] Add WAVS workflows
- [] Expand the registrar to be a WAVS service?
- [] EAS Verify component?

# Notes

- The EAS Indexer contact could be interesting? Maybe something like that could be the basis of the Membrane contract.
- Is an Inderer resolver contract something that makes sense? Or does it make more sense to make an Indexer contract that WAVS can interact with?
- Might be cool to build an offchain verifier service, that provides an example of verifying offchain attestations.


Indexer contract could be used to query attestations for computation without needing to store them offchain?

Membrane could be implemented in a few different ways. I think it's similar to the Indexer...but maybe not. It's really just one number that's result of a WAVS workflow...


Let's also make it so a WAVS service can index attestations, probably should be a separate contract (or chain even?)?
