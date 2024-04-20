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
    use crate::Error;
    use poise::serenity_prelude::Error as serenError;
    use rayon::prelude::*;
    use rust_search::SearchBuilder;
    use std::env::current_dir;
    use std::{fs, panic::*};
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

        fn get_filepath(&self) -> Result<String, Error> {
            let current_dir = current_dir()?;
            let mut path = match current_dir.to_str() {
                Some(x) => x.to_string(),
                None => panic_any("path not found"),
            };

            println!("this is the path: {:?}", &path);
            path.push_str("/duosmium/data/results/");
            println!("this is the new path: {:?}", &path);
            let mut files: Vec<String> = SearchBuilder::default()
                .location(&path)
                .depth(1)
                .search_input(&self.qyear.to_string())
                .build()
                .collect();
            files.remove(0);
            println!("files: {files:?}");

            for fakefile in files {
                println!("this is the file! {:?}", &fakefile);
                let file: String = std::fs::read_to_string(&fakefile)?;
                let yaml_file = &YamlLoader::load_from_str(&file)?[0];
                if fakefile.contains(&self.qinv)
                    && yaml_file["Tournament"]["division"]
                        .as_str()
                        .expect("no division provided?")
                        .to_string()
                        .eq_ignore_ascii_case(&self.qdiv)
                {
                    println!("found file! {:?}", fakefile);
                    return Ok(file);
                }
            }
            Err(Into::into("DNE".to_string()))
        }

        fn find_school(&self) -> Result<(i64, yaml_rust2::Yaml), Error> {
            let school_number;
            let test_file = self.get_filepath()?;
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
            Err(Box::new(serenError::Other("couldn't find school")))
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
