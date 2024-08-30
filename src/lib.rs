use anyhow::{bail, Context};
use serde::Deserialize;
use simd_json::prelude::ValueObjectAccessAsScalar;
use simd_json::Tape;
use trustfall_rustdoc::VersionedCrate;

#[derive(Deserialize)]
struct RustdocFormatVersion {
    format_version: u32,
}

fn parse_or_report_error<T>(file_data: &str) -> anyhow::Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    serde_json::from_str(file_data).with_context(|| "Unexpected parse error for file")
}

fn detect_rustdoc_format_version(file_data: &str) -> anyhow::Result<u32> {
    let versioned_crate = serde_json::from_str::<RustdocFormatVersion>(file_data)
        .with_context(|| "unrecognized rustdoc format for file")?;
    Ok(versioned_crate.format_version)
}

fn simd_detect_rustdoc_format_version(tape: &Tape) -> anyhow::Result<u32> {
    tape.as_value()
        .get_u32("format_version")
        .with_context(|| "unrecognized rustdoc format for file")
}

fn simd_parse_or_report_error<T>(tape: Tape) -> anyhow::Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    tape.deserialize()
        .with_context(|| "unexpected parse error for file")
}

pub fn simd_read_rustdoc(file_data: &mut [u8]) -> anyhow::Result<VersionedCrate> {
    let tape = simd_json::to_tape(file_data)?;
    let format_version = simd_detect_rustdoc_format_version(&tape)?;
    match format_version {
        28 => Ok(VersionedCrate::V28(simd_parse_or_report_error(tape)?)),
        29 => Ok(VersionedCrate::V29(simd_parse_or_report_error(tape)?)),
        30 => Ok(VersionedCrate::V30(simd_parse_or_report_error(tape)?)),
        32 => Ok(VersionedCrate::V32(simd_parse_or_report_error(tape)?)),
        33 => Ok(VersionedCrate::V33(simd_parse_or_report_error(tape)?)),
        _ => bail!("rustdoc format v{format_version} is not supported"),
    }
}

pub fn serde_read_rustdoc(file_data: &str) -> anyhow::Result<VersionedCrate> {
    let format_version = detect_rustdoc_format_version(file_data)?;
    match format_version {
        28 => Ok(VersionedCrate::V28(parse_or_report_error(file_data)?)),
        29 => Ok(VersionedCrate::V29(parse_or_report_error(file_data)?)),
        30 => Ok(VersionedCrate::V30(parse_or_report_error(file_data)?)),
        32 => Ok(VersionedCrate::V32(parse_or_report_error(file_data)?)),
        33 => Ok(VersionedCrate::V33(parse_or_report_error(file_data)?)),
        _ => bail!("rustdoc format v{format_version} is not supported"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_serde_read_rustdocs() {
        let path = Path::new("/home/david/oss/cargo-semver-checks/localdata/test_data/trait_newly_sealed/old/rustdoc.json");
        let file_data = std::fs::read_to_string(path).unwrap();
        assert!(serde_read_rustdoc(&file_data).is_ok());
    }

    #[test]
    fn test_simd_read_rustdocs() {
        let path = Path::new("/home/david/oss/cargo-semver-checks/localdata/test_data/trait_newly_sealed/old/rustdoc.json");
        let mut file_data = std::fs::read(path).unwrap();
        assert!(simd_read_rustdoc(&mut file_data).is_ok());
    }
}
