fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from("src/proto");
    // 生成protobuf
    // prost_build::Config::new()
    // .out_dir(&out_dir).compile_protos(&["proto/product.proto"], &["proto"])?;
    // prost_build::Cocompile_protos(&["proto/product.proto"], &["proto"], &out_dir)?;
    //生成protobuf and grpc api
    tonic_build::configure().out_dir(&out_dir).compile(&["proto/product.proto"], &["proto"])?;
    Ok(())
}
