pub mod parse_file {

    pub use yaml_rust2::{YamlEmitter, YamlLoader};

    pub struct Query {
        qyear: u32,
        qinv: String,
        qschool: String,
        qevent: String,
        school_number: u32,
        file: String,
        rank: u32,
    }

    impl Query {
        pub fn build_query(qyear: u32, qinv: String, qschool: String, qevent: String) -> Query {
            Query {
                qyear,
                qinv,
                qschool,
                qevent,
                school_number: 0,
                file: String::new(),
                rank: 0,
            }
        }

        fn _get_file(&mut self, qyear: u32, qinv: String) {
            self.file = "2024-nCA_states.yaml".to_string();
        }
        fn _find_school(&mut self, qschool: String) {
            let docs = YamlLoader::load_from_str(&self.file).unwrap();
            let doc = &docs[0];
            println!("{}", doc["Teams"][0]["school"].as_str().unwrap());
            self.school_number;
        }
        pub fn find_rank(&mut self, school_number: u32, qevent: String) {
            self.rank;
        }
        pub fn print_fields(&self) {
            println!(
                "year: {}, invitational: {}, school: {}, event: {}",
                &self.qyear, &self.qinv, &self.qschool, &self.qevent
            );
        }
    }
}
