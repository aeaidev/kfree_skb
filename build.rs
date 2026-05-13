// This build script is optional - cargo handles eBPF compilation automatically
// when using the proper target specification in .cargo/config.toml

fn main() {
    // The eBPF crate is built as part of the workspace
    // No special build steps needed
    println!("cargo:rustc-env=eBPF_BUILD=1");
}
