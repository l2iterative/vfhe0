<img align="right" src="title.png" width="325" alt="Paris city view"/>

# Verify Zama TFHE bootstrapping in RISC Zero

This repo is a fork of Louis Tremblay Thibault's VFHE implementation (https://github.com/tremblaythibaultl/vfhe). 

VFHE is exploring how to verify VFHE bootstrapping off the shelf in ZK by porting an existing Rust 
library of FHE---called toy FHE (https://github.com/tremblaythibaultl/ttfhe)---out of the box. There is a demand for 
very efficient verification of such FHE operation, which is needed for FHE ZK-Rollup as well as FHE mining.

We have been experimenting with the VFHE library, mainly on how to make it more efficient so that the proof generation 
time can be reduced.

## Roadmap

For the purpose of checkpointing the library, a few tags are created, reflecting different versions of this library. 
People can compare between these tags and the latest version.

- 

## Articles

This repo is associated with a few articles that describe the entire journey.

- Tech Deep Dive: Verifying FHE in RISC Zero, Part I ([here](https://l2ivresearch.substack.com/p/tech-deep-dive-verifying-fhe-in-risc?)), which talks about the following.
  - How to load constant parameters into the guest in a zero-copy manner
  - How to use [Ghidra](https://github.com/NationalSecurityAgency/ghidra) to analyze the executable file from RISC Zero


## License

Louis uses the Apache 2 license. We intend to do the same. 
