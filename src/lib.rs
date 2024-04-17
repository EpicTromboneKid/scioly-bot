pub type Error = Box<dyn std::error::Error + Send + Sync>;

//mod parse_input {
//  use crate::Error;
//fn parse_input(
//  in_year: u32,
//in_invy: String,
//in_school: String,
//        in_event: String,
//      in_div: String,
//) -> Result<(u32, String, String, String, String), Error> {
//}
//}

pub mod parse_file {
    //use crate::parse_input;
    use crate::Error;
    use jwalk::WalkDir;
    use std::env::current_dir;
    use std::{fs::File, io::Read};
    use yaml_rust2::YamlLoader;

    use String;
    pub struct Query {
        pub qyear: i32,
        pub qinv: String,
        pub qschool: String,
        pub qevent: String,
        pub qdiv: String,
    }

    impl Query {
        pub fn build_query(
            year: i32,
            inv: String,
            school: String,
            mut event: String,
            div: String,
        ) -> Query {
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
                qdiv: div,
            }
        }

        fn get_file(&self) -> Result<File, Error> {
            let current_dir = current_dir()?;
            let mut path = match current_dir.to_str() {
                Some(x) => x.to_string(),
                None => panic!("path not found"),
            };
            let mut file_path = String::new();

            println!("this is the path: {:?}", &path);
            path.push_str("/duosmium/data/results");
            println!("this is the new path: {:?}", &path);

            for file in WalkDir::new(&path).sort(true) {
                file_path = file?.path().display().to_string();
                println!("this is the file! {}", &file_path)
            }

            let return_value: File = File::open(file_path)?;
            println!("{:?}", return_value);
            Ok(return_value)
        }

        fn find_school(&self) -> Result<(i64, yaml_rust2::Yaml), Error> {
            let mut test_file = String::new();
            let mut school_number = -1;
            let error_val = &YamlLoader::load_from_str("school not found").unwrap()[0];
            let _ = self.get_file()?.read_to_string(&mut test_file);
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
                    .to_lowercase()
                    .contains(&self.qschool.as_str().to_lowercase())
                {
                    school_number = i["number"].as_i64().expect("NaN");
                    println!("successfully found school_rank: {}", school_number);
                    return Ok((school_number, doc.clone()));
                } else {
                    println!("{:?}", &i["school"]);
                }
            }
            Ok((school_number, error_val[0].clone()))
        }

        pub fn find_rank(&self) -> Result<i64, Error> {
            let (number, the_doc) = self.find_school()?;
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
                    return Ok(i["place"].clone().into_i64().expect("3"));
                }
            }
            Err(Into::into("placing not found"))
        }

        pub fn print_fields(&self) {
            println!(
                "year: {}, invitational: {}, school: {}, event: {}",
                &self.qyear, &self.qinv, &self.qschool, &self.qevent
            );
        }
    }
}
