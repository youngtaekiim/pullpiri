fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/constants.proto")?;
    tonic_build::compile_protos("proto/apiserver.proto")?;
    tonic_build::compile_protos("proto/apiserver/request.proto")?;
    tonic_build::compile_protos("proto/apiserver/updateworkload.proto")?;
    tonic_build::compile_protos("proto/apiserver/scenario.proto")?;
    tonic_build::compile_protos("proto/statemanager.proto")?;
    tonic_build::compile_protos("proto/gateway.proto")?;
    Ok(())
}
