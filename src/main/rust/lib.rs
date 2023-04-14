// Include the modules, which are generated from the .proto files
pub mod phenopacket_schema {
    // Generated by [`prost-build`]
    // include!(concat!(env!("OUT_DIR"), "/mod.rs"));
    // I don't know how to generate this automagically, so it's manually imported for now.
    pub mod org {
        pub mod ga4gh {
            pub mod pedigree {
                pub mod v1 {
                    include!(concat!(env!("OUT_DIR"), "/org.ga4gh.pedigree.v1.rs"));
                    include!(concat!(env!("OUT_DIR"), "/org.ga4gh.pedigree.v1.serde.rs"));
                }
            }
            pub mod vrs {
                pub mod v1 {
                    include!(concat!(env!("OUT_DIR"), "/org.ga4gh.vrs.v1.rs"));
                    include!(concat!(env!("OUT_DIR"), "/org.ga4gh.vrs.v1.serde.rs"));
                }
            }
            pub mod vrsatile {
                pub mod v1 {
                    include!(concat!(env!("OUT_DIR"), "/org.ga4gh.vrsatile.v1.rs"));
                    include!(concat!(env!("OUT_DIR"), "/org.ga4gh.vrsatile.v1.serde.rs"));
                }
            }
        }
        pub mod phenopackets {
            pub mod schema {
                pub mod v1 {
                    pub mod core {
                        include!(concat!(env!("OUT_DIR"), "/org.phenopackets.schema.v1.core.rs"));
                        include!(concat!(env!("OUT_DIR"), "/org.phenopackets.schema.v1.core.serde.rs"));
                    }
                    include!(concat!(env!("OUT_DIR"), "/org.phenopackets.schema.v1.rs"));
                    include!(concat!(env!("OUT_DIR"), "/org.phenopackets.schema.v1.serde.rs"));
                }
                pub mod v2 {
                    pub mod core {
                        include!(concat!(env!("OUT_DIR"), "/org.phenopackets.schema.v2.core.rs"));
                        include!(concat!(env!("OUT_DIR"), "/org.phenopackets.schema.v2.core.serde.rs"));
                    }
                    include!(concat!(env!("OUT_DIR"), "/org.phenopackets.schema.v2.rs"));
                    include!(concat!(env!("OUT_DIR"), "/org.phenopackets.schema.v2.serde.rs"));
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;
    use pbjson_types::Timestamp;
    use crate::phenopacket_schema::org::phenopackets::schema::v2::core::{Individual, PhenotypicFeature, Sex, TimeElement};
    use crate::phenopacket_schema::org::phenopackets::schema::v2::core::OntologyClass;
    use crate::phenopacket_schema::org::phenopackets::schema::v2::core::time_element::Element;
    use crate::phenopacket_schema::org::phenopackets::schema::v2::Phenopacket;

    #[test]
    fn phenopacket() {
        let ontology_class = OntologyClass { id: "HP:0000001".to_string(), label: "Phenotypic abnormality".to_string() };
        let mut phenotypic_feature = PhenotypicFeature::default();
        phenotypic_feature.r#type = Some(ontology_class);

        let mut phenotypic_features = vec!(phenotypic_feature);

        let mut individual = Individual::default();
        individual.id = "subject1".to_string();
        individual.sex = Sex::Male.into();
        individual.time_at_last_encounter = Some(TimeElement { element: Some(Element::Timestamp(Timestamp { seconds: 1681483297, nanos: 0 })) });

        let mut phenopacket = Phenopacket::default();
        phenopacket.id = "example1".to_string();
        phenopacket.subject = Some(individual);
        phenopacket.phenotypic_features.append(&mut phenotypic_features);

        let j = serde_json::to_string_pretty(&phenopacket).unwrap();
        println!("{}", j);

        let deserde = serde_json::from_str(&j).unwrap();
        assert_eq!(phenopacket, deserde);

        let yaml = serde_yaml::to_string(&phenopacket).unwrap();
        println!("{}", yaml);
    }

    #[test]
    fn parse_phenopacket_json() {
        let phenopacket = read_phenopacket();
        println!("{:?}", phenopacket);

        let json = serde_json::to_string_pretty(&phenopacket).unwrap();
        println!("{}", json);
    }

    #[test]
    fn parse_phenopacket_json_write_yaml() {
        let phenopacket = read_phenopacket();
        println!("{:?}", phenopacket);

        let yaml = serde_yaml::to_string(&phenopacket).unwrap();
        println!("{}", yaml);
    }

    fn read_phenopacket() -> Phenopacket {
        let file_path = Path::new("src/test/python/covid19.json");
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let phenopacket: Phenopacket = serde_json::from_reader(reader).unwrap();
        phenopacket
    }
}