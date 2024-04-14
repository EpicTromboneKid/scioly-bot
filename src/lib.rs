pub mod parse_file {

    use std::{fs::File, io::Read};
    use yaml_rust2::{YamlEmitter, YamlLoader};
    use String;

    pub struct Query {
        qyear: u32,
        qinv: String,
        qschool: String,
        qevent: String,
        _school_number: u32,
        _rank: u32,
    }

    impl Query {
        pub fn build_query(qyear: u32, qinv: String, qschool: String, qevent: String) -> Query {
            Query {
                qyear,
                qinv,
                qschool,
                qevent,
                _school_number: 0,
                _rank: 0,
            }
        }

        fn _get_file(&self, _qyear: u32, _qinv: String) -> File {
            let return_value: File =
                File::open("/home/chaas/neovim/scioly-bot/src/2024-nCA_states.yaml").expect("hi");
            println!("{:?}", return_value);
            return_value
        }

        fn find_school(&self, _qschool: &String) {
            let mut test_file = String::new();
            let _ = self
                ._get_file(22, "lol".to_string())
                .read_to_string(&mut test_file);
            println!("{:?}", &test_file);
            let docs =
                YamlLoader::load_from_str(test_file.as_str()).expect("Loading file didn't work");
            let doc = &docs[0];
            println!("{doc:?}");
            println!("the docs: {:?}", docs);
            println!("{}", doc["Teams"][0]["school"].as_str().unwrap());
        }

        fn _find_rank(&mut self, _school_number: u32, _qevent: String) {}

        pub fn print_fields(&self) {
            println!(
                "year: {}, invitational: {}, school: {}, event: {}",
                &self.qyear, &self.qinv, &self.qschool, &self.qevent
            );
            self.find_school(&self.qschool)
        }
    }
}
