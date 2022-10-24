# Implementation of Chaum-Pedersen Protocol

The present repository contains an implementation of the Chaum-Pedersen Protocol. The implementation consists in a server and a client using gRPC protocol for communication. 

## Implementation

The implementation in Rust is in a single project with two binaries, one for the server called `zkp-server` and another one for the client called `zkp-client`. This was done for simplicity but in the real world it might be better to separate the client and server in their own projects.

### External Dependencies

The following dependencies were used:
- tonic, a mature gRPC framework
- [tokio](https://tokio.rs/), an asynchronous runtime framework

## Limitations

The code presented here is valid for testing purposes only. There are some aspects to consider if we wanted this code to support a real life use case.
### Storage

The code uses basic in-memory storage for Authentications as well as User registrations. The storage is backed by a Hashmap using `Arc` and `Mutex` for simple thread safety. 

A very basic `Store` trait has been provided with the intention to allow pluggable implementations of other storage backend.

### Choice of `g`,`h`,`q`, Storage

For the protocol to work securely, it is important to generate large numbers for `g`,`h` and `q`. Both `g` and `h` are public keys that generate groups of prime order `q`. 

The [original paper from Chaum and Pedersen](https://link.springer.com/content/pdf/10.1007/3-540-48071-4_7.pdf) suggests a likely mechanism for that. 

Both client and server need to make use of the same (`g`,`h`,`q`) for the protocol to work correctly. Ideally this should be read from a configuration file or environmental variables (as it is the usual practice in Dockerland).

For convenience, I have extended the gRPC protocol with a call to initialise the protocol with an appropriate set of `g`,`h` and `q`. The client uses the call to initialise and generate the authentication challenges. 

### Client

The client has hardcoded steps. This was done for simplicity. 

Ideally the client should offer some sort of interactivity. Perhaps via a command line interface or a REST API. 

### Session storage







