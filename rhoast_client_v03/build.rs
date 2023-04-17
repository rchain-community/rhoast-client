//file to build the protobuf
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //v0.13
    tonic_build::configure()
        .compile(
            &[
                "protobufv03/CasperMessage.proto",
                "protobufv03/DeployServiceCommon.proto",
                "protobufv03/ProposeServiceCommon.proto",
            ],
            &["protobufv03"],
        )
        .unwrap();
    tonic_build::configure()
        .compile(&["protobufv03/DeployServiceV1.proto"], &["protobufv03"])
        .unwrap();
    tonic_build::compile_protos("protobufv03/scalapb/scalapb.proto")?;
    tonic_build::compile_protos("protobufv03/RhoTypes.proto")?;
    tonic_build::compile_protos("protobufv03/routing.proto")?;
    tonic_build::compile_protos("protobufv03/ServiceError.proto")?;
    Ok(())
}
