pub mod parse_file {

    use std::{fs::File, io::Read};
    use yaml_rust2::{yaml::Array, YamlEmitter, YamlLoader};
    use String;

    pub struct Query {
        pub qyear: u32,
        pub qinv: String,
        pub qschool: String,
        pub qevent: String,
        _rank: u32,
    }

    impl Query {
        pub fn build_query(year: u32, inv: String, mut school: String, event: String) -> Query {
            if school.contains("High School") != true {
                school.push_str(" High School");
            }
            Query {
                qyear: year,
                qinv: inv,
                qschool: school,
                qevent: event,
                _rank: 0,
            }
        }

        fn get_file(&self) -> File {
            let return_value: File =
                File::open("/home/chaas/neovim/scioly-bot/src/2024-nCA_states.yaml").expect("hi");
            println!("{:?}", return_value);
            return_value
        }

        fn find_school(&self) -> (i64, yaml_rust2::Yaml) {
            let mut test_file = String::new();
            let mut school_number = -1;
            let error_val = &YamlLoader::load_from_str("school not found").unwrap()[0];
            let _ = self.get_file().read_to_string(&mut test_file);
            let docs =
                YamlLoader::load_from_str(&test_file.as_str()).expect("Loading file didn't work");
            let doc = &docs[0];
            println!(
                "The first team in this file is: {:?}",
                &doc["Teams"][0]["school"].as_str().expect("oof")
            );
            let teams = doc["Teams"].clone().into_iter();
            for i in teams {
                let school_at_i = &i["school"].clone().into_string().expect("didn't work.");

                if school_at_i == &self.qschool {
                    school_number = i["number"].as_i64().expect("NaN");
                    println!("successfully found school_rank: {}", school_number);
                    return (school_number, doc.clone());
                } else {
                    println!("{:?}", &i["school"]);
                }
            }
            (school_number, error_val[0].clone())
        }

        pub fn find_rank(&self) -> i64 {
            let (number, the_doc) = self.find_school();
            let event = &self.qevent;
            println!(
                "in find_rank now, the number, first team, and event is: {:?}, {:?}, {:?}",
                &number,
                &the_doc["Teams"][0]["school"]
                    .as_str()
                    .expect("could not find or something"),
                &event
            );
            let placings = the_doc["Placings"].clone().into_iter();
            for i in placings {
                let team_at_i = &i["team"].clone().into_i64().expect("1");
                let event_at_i = &i["event"].clone().into_string().expect("2");
                if number != -1 && team_at_i == &number && &event_at_i == &event {
                    println!("{:?}", i["place"].clone().into_i64().expect("3"));
                    return i["place"].clone().into_i64().expect("3");
                }
            }
            -1
        }

        pub fn print_fields(&self) {
            println!(
                "year: {}, invitational: {}, school: {}, event: {}",
                &self.qyear, &self.qinv, &self.qschool, &self.qevent
            );
        }
    }
}
