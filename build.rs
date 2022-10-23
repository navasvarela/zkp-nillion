// This builds the client and server using the protobuf definition
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/zkp_auth.proto")?;
    Ok(())
}