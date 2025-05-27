fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(&["proto/items.proto"], &["proto/"])?;
    tonic_build::compile_protos("proto/route_guide.proto")?;
    tonic_build::compile_protos("proto/echo.proto")?;
    tonic_build::compile_protos("proto/unaryecho.proto")?;
    Ok(())
}