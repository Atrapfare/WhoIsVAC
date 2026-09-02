use embed_manifest::{embed_manifest, manifest::ExecutionLevel, new_manifest};

fn main() {
    let windows = std::env::var_os("CARGO_CFG_WINDOWS").is_some();
    // Release only: in a debug build this would pop a UAC dialog on every run.
    let release = std::env::var("PROFILE").as_deref() == Ok("release");

    if windows && release {
        embed_manifest(
            new_manifest("WhoIsVAC")
                .requested_execution_level(ExecutionLevel::RequireAdministrator),
        )
        .expect("unable to embed the application manifest");
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=offsets/cs2-dumper.exe");
}
