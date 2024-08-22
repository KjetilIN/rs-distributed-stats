/// Main method for the tonic library.
/// This is here so that the library will generate the code needed.
///
/// See: https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md#generating-server-and-client-code
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/statservice.proto")?;
    Ok(())
}
