fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-env-changed=REGENERATE_PROTO");
    println!("cargo:rerun-if-changed=protos");

    if std::env::var("REGENERATE_PROTO").is_err() {
        return Ok(());
    }

    tonic_prost_build::configure()
        .bytes(".packet.Packet.data")
        .bytes(".bundle.JitoBundle.bundle_id")
        .bytes(".bundle.JitoBundle.identity_address")
        .bytes(".searcher.PendingTxNotification.identity_address")
        .out_dir("src/generated")
        .compile_protos(
            &[
                "protos/auth.proto",
                "protos/packet.proto",
                "protos/shared.proto",
                "protos/bundle.proto",
                "protos/searcher.proto",
            ],
            &["protos"],
        )?;

    Ok(())
}
