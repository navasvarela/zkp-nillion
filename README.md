# Implementation of Chaum-Pedersen Protocol

The present repository contains an implementation of the Chaum-Pedersen Protocol. The implementation consists in a server and a client using gRPC protocol for communication. 

## Implementation

The implementation in Rust is in a single project with two binaries, one for the server called `zkp-server` and another one for the client called `zkp-client`. This was done for simplicity but in the real world it might be better to separate the client and server in their own projects.

### External Dependencies

The following dependencies were used:
- tonic, a mature gRPC framework
- [tokio](https://tokio.rs/), an asynchronous runtime framework
- prime, for prime number generation.

## Limitations

The code presented here is valid for testing purposes only. If we wanted this code to support a real life use case we should improve it. 

### Storage

The code uses basic in-memory storage for Authentications as well as User registrations. The storage is backed by a Hashmap using `Arc` and `Mutex` for simple thread safety. 

A very basic `Store` trait has been provided with the intention to allow pluggable implementations of other storage backend.

### Choice of `g`,`h`,`q`, Storage

For the protocol to work securely, it is important to generate large numbers for `g`,`h` and `q`. Both `g` and `h` are public keys that generate groups of prime order `q`. The modulus `p` of the group is also important. 

The [original paper from Chaum and Pedersen](https://link.springer.com/content/pdf/10.1007/3-540-48071-4_7.pdf) suggests a likely mechanism for that. 

Both client and server need to make use of the same (`g`,`h`,`q`) for the protocol to work correctly. Ideally this should be read from a configuration file or environmental variables (as it is the usual practice in Dockerland).

For convenience, I have extended the gRPC protocol with a call to initialise the protocol with an appropriate set of `g`,`h` and `q`. The client uses the call to initialise and generate the authentication challenges. 

I have also added a naive implementation of a utility that generates the modulus and the order of prime order Group, as well as the two generators. The implementation is in the file [zkp_cripto.rs](src/zkp_crypto.rs). 

### Client

The client has hardcoded steps. This was done for simplicity. 

Ideally the client should offer some sort of interactivity. Perhaps via a command line interface or a REST API. 

### Numbers

The solution is implemented using primitive types i64. This limits the size of numbers that can be used in the protocol. Bigger numbers lead to better security in general, so the protocol should be extended to use arbitrary precision arithmetic.

Ideally, the solution would work with BigInt types. Changes should be made in several places. A suggestion would be to replace `int64` by `String` types in the gRPC protobuf specification. I believe `String` is the only type with variable length in gRPC.

### Cryptography

The present implementation only contains the original based on exponentiations. Some steps have been taken to provide support
The files [prover.rs](src/prover.rs) and []

## Testing

Some unit tests have been added. They can be run using `cargo test`.

For an integration test the server can be run first and then the client. For convenience a couple of make targets have been created: `run-server` and `run-client`.





