use noodles_vcf as vcf;
use std::collections::HashMap;
use std::rc::Rc;
use std::{cell::RefCell, io::Error};

pub enum Genotype {
    //HomRef, -- no gVCF support yet
    //Unknown, -- no BED support yet
    Het,
    HomAlt,
}

pub struct Person {
    sample_id: String,
    father_id: Option<String>, //RefCell<Rc<Individual>>?
    mother_id: Option<String>,
    affected: bool,
}

pub struct Family {
    members: HashMap<String,Person>, // TODO: check all sample_ids (father/mother) are present
}


pub fn build_family_structure_from_csv(file_path: &str) -> Result<Family, Error> {
    let mut family_csv = csv::Reader::from_path(file_path)?;
    family_csv.headers()?;
    let mut members: HashMap<String,Person> = HashMap::new();
    for result in family_csv.records() {
        let record = result?;
        let sample_id = record.get(0).unwrap().to_string();
        let father_id = match record.get(1).unwrap().trim() {
            "" => None,
            s => Some(s.to_string()),
        };
        let mother_id = match record.get(2).unwrap().trim() {
            "" => None,
            s => Some(s.to_string()),
        };
        let affected: bool = match record.get(3).unwrap().trim().to_ascii_lowercase().as_str() {
            "1"|"y"|"yes"|"t"|"true"|"affected" => true,
            "0"|"n"|"no"|"f"|"false"|"unaffected" => false, 
            _ => return Err(Error::new(std::io::ErrorKind::InvalidInput, 
                format!("Affected status must be '1|y|yes|t|true|affected' or '0|n|no|ff|false|unaffected', found '{}'", record.get(3).unwrap()))),
        };
        let person = Person {
            sample_id: sample_id.clone(),
            father_id,
            mother_id,
            affected,
        };
        members.insert(sample_id, person);
    }
    Ok(Family { members })
}


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
    use varisat::{Lit, CnfFormula, ExtendFormula};


    #[test]
    fn build_family_structure() {
        let family = build_family_structure_from_csv("tests/resources/sample_family.csv").unwrap();
        assert_eq!(family.members.len(), 3);
        let s1 = family.members.get("S1").unwrap();
        assert_eq!(s1.father_id.as_ref().unwrap(), "S2");
        assert_eq!(s1.mother_id.as_ref().unwrap(), "S3");
        assert!(s1.affected);
        let s2 = family.members.get("S2").unwrap();
        assert!(s2.father_id.is_none());
        assert!(s2.mother_id.is_none());
        assert!(!s2.affected);
        let s3 = family.members.get("S3").unwrap();
        assert!(s3.father_id.is_none());
        assert!(s3.mother_id.is_none());
        assert!(!s3.affected);
    }

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

    #[test]
    fn solve_sat_problem() {
        let x: Lit = Lit::from_dimacs(1);
        let y: Lit = Lit::from_dimacs(2);

        let mut formula = CnfFormula::new();
        formula.add_clause(&[x, y]);
    }
}