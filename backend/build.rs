fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/sample.proto")?;
    tonic_build::compile_protos("../proto/sample2.proto")?;
    Ok(())
}
