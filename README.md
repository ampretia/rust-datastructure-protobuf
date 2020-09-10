# Rust Datastructure Experiment

## Background

Within Hyperledger Fabric there are Endorsement Policies that define who needs to sign off on resource access. For example

```
AND(Org1.MEMBER,OR(Org2.MEMBER,Org3.MEMBER))
```
means that we need approval from a Member of Org1, AND at least one Member of Org2 or Org3.

This repo is an example of how to define this as a datastructure within Rust, and also how to copy this into a Protobuf for serialization over network links. 

## Code

Single `main.rs` with the definition of the datastructures, the protobuf definitions come from an external crate. 


