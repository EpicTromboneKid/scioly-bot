pub mod parse_file {
    use std::env::current_dir;
    use std::fs;
    use std::{fs::File, io::Read};
    use yaml_rust2::{yaml::Array, YamlEmitter, YamlLoader};
    use String;

    pub struct Query {
        pub qyear: u32,
        pub qinv: String,
        pub qschool: String,
        pub qevent: String,
    }

    impl Query {
        pub fn build_query(year: u32, inv: String, mut school: String, mut event: String) -> Query {
            if school.contains("High School") != true {
                school.push_str(" High School");
            }
            if event.as_str().eq_ignore_ascii_case("Chemistry")
                || event.clone().as_str().eq_ignore_ascii_case("Chem")
            {
                event = "Chemistry Lab".to_string();
            }

            Query {
                qyear: year,
                qinv: inv,
                qschool: school,
                qevent: event,
            }
        }

        fn get_file(&self) -> File {
            let current_dir = current_dir().unwrap();
            let mut path = current_dir.to_str().expect("bruh").to_string();
            let mut file_path = String::new();

            println!("this is the path: {:?}", &path);
            path.push_str("/duosmium/data/results");
            println!("this is the new path: {:?}", &path);

            for file in fs::read_dir(&path).expect("fake directory oof") {
                file_path = file.unwrap().path().display().to_string();
                println!("this is the current file path: {:?}", &file_path);
                if file_path.contains(&self.qyear.to_string()) && file_path.contains(&self.qinv) {
                    println!("Found file: {:?}", &file_path);
                    break;
                }
            }

            let return_value: File = File::open(file_path).expect("hi");
            println!("{:?}", return_value);
            return_value
        }

        fn find_school(&self) -> (i64, yaml_rust2::Yaml) {
            let mut test_file = String::new();
            let mut school_number = -1;
            let error_val = &YamlLoader::load_from_str("school not found").unwrap()[0];
            let _ = self.get_file().read_to_string(&mut test_file);
            let docs = YamlLoader::load_from_str(&test_file).expect("Loading file didn't work");
            let doc = &docs[0];
            println!(
                "The first team in this file is: {:?}",
                &doc["Teams"][0]["school"].as_str().expect("oof")
            );
            let teams = doc["Teams"].clone().into_iter();
            println!("{:?}", &self.qschool);
            for i in teams {
                let school_at_i = &i["school"].clone().into_string().expect("didn't work.");

                if school_at_i
                    .as_str()
                    .eq_ignore_ascii_case(&self.qschool.as_str())
                {
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
                    .expect("could not find school in file :("),
                &event
            );
            let placings = the_doc["Placings"].clone().into_iter();
            for i in placings {
                let team_at_i = &i["team"].clone().into_i64().expect("1");
                let event_at_i = &i["event"].clone().into_string().expect("2");
                if number != -1
                    && team_at_i == &number
                    && event_at_i.as_str().eq_ignore_ascii_case(&event)
                {
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
