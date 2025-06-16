# WAVS-EAS

This is a an example that integrates Ethereum Attestation Service (EAS) with verifiable offchain applications powered by WAVS.

This allows for AVSs to make attestations and perform computations over attestation graphs.

TODO:
1. Modify attester.sol to implement the IWavsServiceHandler interface, on handleSignedEnvelope it should make an attestation
2. Deploy scripts to deploy EAS contracts (WAVS tooling already handles deploying other contracts)
3. WAVS component that makes an attestation
4. A WAVS AVS that performs computations over attestations
5. Simple UI to make attestations

WAVS Component Examples:
- An WAVS AVS that can make attestations
- A WAVS AVS that performs computations over attestations
- Composing botth together (i.e. a WAVS AVS that performs computations over attestations and makes attestations)
