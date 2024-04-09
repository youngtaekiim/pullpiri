fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/apiserver.proto")?;
    tonic_build::compile_protos("proto/apiserver/request.proto")?;
    tonic_build::compile_protos("proto/apiserver/updateworkload.proto")?;
    tonic_build::compile_protos("proto/statemanager.proto")?;
    Ok(())
}
