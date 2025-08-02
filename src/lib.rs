use noodles_vcf as vcf;
use std::io::Error;

#[allow(dead_code)] // Work in progress
fn read_vcf_file(file_path: &str) -> Result<vcf::Header, Error> {
    let mut reader = vcf::io::reader::Builder::default().build_from_path(file_path)?;
    let header: vcf::Header = reader.read_header()?;

    for result in reader.records() {
        let _record: vcf::Record = result?;
        // TODO Move to stream
    }
    Ok(header)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_vcf_file() {
        let mut reader = vcf::io::reader::Builder::default().build_from_path("tests/resources/HG001_GRCh38_1_22_v4.2.1_benchmark.snippet.vcf").unwrap();
        let header = reader.read_header().unwrap();
        assert!(header.contigs().len() > 0);
        for result in reader.records() {
            let record: vcf::Record = result.unwrap();
            assert!(!record.samples().is_empty());
        }
    }
}