fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(&["proto/items.proto"], &["proto/"])?;
    tonic_build::compile_protos("proto/route_guide.proto")?;
    Ok(())
}