# WAVS-EAS

This is a an example that integrates Ethereum Attestation Service (EAS) with verifiable offchain applications powered by WAVS.

This allows for AVSs to make attestations and perform computations over attestation graphs.

TODO:
1. WAVS component that makes an attestation
2. A WAVS AVS that performs computations over attestations
3. Simple UI to make attestations

WAVS Component Examples:
- An WAVS AVS that can make attestations
- A WAVS AVS that performs computations over attestations
- Composing botth together (i.e. a WAVS AVS that performs computations over attestations and makes attestations)

Flow:
1. User makes an attestation using the WAVS component
2. The attestation is stored on the EAS contract
3. The WAVS AVS performs computations over the attestations
4. The results are stored on the EAS contract
5. The WAVS component can make attestations based on the results of the computations
6. A new attestation of the graph is made using using an attestation (to update membership)
7. Graphs and computations over the graph can be proven
