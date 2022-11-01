//file to build the protobuf
fn main() -> Result<(), Box<dyn std::error::Error>>{
    tonic_build::compile_protos("protobuf/CasperMessage.proto")?;
    Ok(())
}