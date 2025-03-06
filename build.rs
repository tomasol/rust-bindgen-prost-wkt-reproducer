use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let proto_path = PathBuf::from("input.proto");
    tonic_build::configure()
        .compile_well_known_types(true)
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .extern_path(".google.protobuf.Duration", "::prost_wkt_types::Duration")
        .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
        .build_server(false)
        .build_client(true)
        .compile_protos(&[proto_path], &[] as &[PathBuf; 0])?;
    Ok(())
}
