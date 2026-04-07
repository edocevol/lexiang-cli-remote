fn main() -> anyhow::Result<()> {
    // Embed schemas at compile time
    println!("cargo:rerun-if-changed=schemas/");
    Ok(())
}
