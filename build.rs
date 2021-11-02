use std::io::Result;
fn main() -> Result<()> {
    println!("{:?}", prost_build::protoc());
    prost_build::compile_protos(&["proto/client.proto"], &["proto/"])?;
    Ok(())
}
