fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::compile_protos("proto/models.proto")?;
  tonic_build::compile_protos("proto/http_client.proto")?;
  Ok(())
}
