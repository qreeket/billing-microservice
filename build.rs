use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original_out_dir = PathBuf::from(env::var("OUT_DIR")?);

    // generate proto files for server only using tonic-build
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(original_out_dir.join("billing_descriptor.bin"))
        .compile(&[
            "protos/billing.proto",
            "protos/billing_service.proto",
        ], &["protos"])?;

    Ok(())
}