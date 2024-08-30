use anyhow::Context;
use trustfall_rustdoc_adapter_v28::Crate;

pub fn serde_read_rustdoc(file_data: &str) -> anyhow::Result<Crate> {
    serde_json::from_str(file_data).with_context(|| "Unexpected parse error")
}

pub fn simd_read_rustdoc(file_data: &mut [u8]) -> anyhow::Result<Crate> {
    let tape = simd_json::to_tape(file_data)?;
    tape.deserialize().with_context(|| "Unexpected parse error")
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read as _;
    use std::path::Path;

    #[test]
    fn test_serde_read_rustdocs() {
        let path = Path::new("/home/david/oss/cargo-semver-checks/localdata/test_data/trait_newly_sealed/old/rustdoc.json");
        let mut file_data = String::new();
        File::open(path)
            .with_context(|| format!("Failed to open file: {:?}", path))
            .unwrap()
            .read_to_string(&mut file_data)
            .with_context(|| format!("Failed to read file: {:?}", path))
            .unwrap();
        assert!(serde_read_rustdoc(&file_data).is_ok());
    }

    #[test]
    fn test_simd_read_rustdocs() {
        let path = Path::new("/home/david/oss/cargo-semver-checks/localdata/test_data/trait_newly_sealed/old/rustdoc.json");
        let mut file_data = Vec::new();
        File::open(path)
            .with_context(|| format!("Failed to open file: {:?}", path))
            .unwrap()
            .read_to_end(&mut file_data)
            .with_context(|| format!("Failed to read file: {:?}", path))
            .unwrap();
        assert!(simd_read_rustdoc(&mut file_data).is_ok());
    }
}
