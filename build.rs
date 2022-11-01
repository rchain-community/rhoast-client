//file to build the protobuf
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("protobuf/CasperMessage.proto")?;
    tonic_build::compile_protos("protobuf/scalapb/scalapb.proto")?;
    tonic_build::compile_protos("protobuf/DeployServiceCommon.proto")?;
    tonic_build::compile_protos("protobuf/DeployServiceV1.proto")?;
    tonic_build::compile_protos("protobuf/ProposeServiceV1.proto")?;
    tonic_build::compile_protos("protobuf/RhoTypes.proto")?;
    tonic_build::compile_protos("protobuf/routing.proto")?;
    Ok(())
}
