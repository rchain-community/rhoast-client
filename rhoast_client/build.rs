//file to build the protobuf
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //v0.12
    tonic_build::configure()
        .compile(
            &[
                "protobuf/CasperMessage.proto",
                "protobuf/DeployServiceCommon.proto",
                "protobuf/ProposeServiceCommon.proto",
            ],
            &["protobuf"],
        )
        .unwrap();
    tonic_build::configure()
        .compile(&["protobuf/DeployServiceV1.proto"], &["protobuf"])
        .unwrap();
    tonic_build::compile_protos("protobuf/scalapb/scalapb.proto")?;
    tonic_build::compile_protos("protobuf/RhoTypes.proto")?;
    tonic_build::compile_protos("protobuf/routing.proto")?;
    tonic_build::compile_protos("protobuf/ServiceError.proto")?;
    Ok(())
}
