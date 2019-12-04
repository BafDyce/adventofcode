#[macro_use]
extern crate serde_derive;

use clap::{App, Arg, ArgMatches};
use serde_json::Value;
use std::{
    collections::HashMap,
    fmt::Debug,
    fs::OpenOptions,
    io::{self, BufRead, Read},
};

// general stuff, should not be necessary to touch this
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PuzzleOptions<T>
where
    T: Clone + Debug + PartialEq,
{
    pub day: i32,
    pub input_name: String,
    pub verbose: bool,
    pub skip_p1: bool,
    pub data: Option<T>,
    pub config: HashMap<String, String>,
}

impl<T> PuzzleOptions<T>
where
    T: Clone + Debug + PartialEq,
{
    fn with_input_parsed<F>(mut self, parse_input_function: F) -> Result<Self, io::Error>
    where
        F: Fn(Vec<String>, &HashMap<String, String>, bool) -> T,
    {
        // first try as path
        let input = read_input_file(&self.input_name).or(
            // if that doesnt work (i.e. file not found), try to get it from _inputs dir
            read_input_file(&format!(
                "../../_inputs/day{:02}/{}.input",
                self.day, self.input_name
            )),
        )?;

        if self.verbose {
            println!("raw input: {:?}", input);
        }

        // dont fail if we
        match read_config_file(&format!(
            "../../_inputs/day{:02}/{}.config",
            self.day, self.input_name
        )) {
            Ok(config) => {
                if self.verbose {
                    println!("config (file): {:#?}", config);
                }
                for (kk, vv) in config.into_iter() {
                    if !self.config.contains_key(&kk) {
                        self.config.insert(kk, {
                            match vv {
                                Value::String(vv) => vv.to_owned(),
                                Value::Bool(vv) => format!("{}", vv),
                                Value::Number(vv) => format!("{}", vv),
                                other => panic!(format!("Value type not supported ({:?})", other)),
                            }
                        });
                    }
                }
            }
            Err(ee) => {
                println!(
                    "[WARN] Failed to load config file. Will continue either way: {}",
                    ee
                );
            }
        }

        self.data = Some(parse_input_function(input, &self.config, self.verbose));
        if self.verbose {
            println!("parsed input: {:#?}", self.data.as_ref().unwrap());
        }

        Ok(self)
    }
}

fn args<'a, 'b>(day: i32) -> App<'a, 'b> {
    App::new(format!("aoc-2019-{:02}", day))
        .author("Fabian Würfl <bafdyce@tuta.io>")
        .arg(Arg::with_name("input")
            .long("input")
            .help("name of the config file")
            .takes_value(true)
            .default_value("real1")
        )
        .arg(Arg::with_name("skip_p1")
            .long("skip--first-part")
            .help("Skip first part and directly compute second")
        )
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("verbose output")
        )
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .help("Specify key value pair for puzzle configuration. This overrides values defined in the input file config")
            .number_of_values(2)
            .multiple(true)
            .value_delimiter("=")
        )
}

fn read_config_file(path: &str) -> Result<HashMap<String, Value>, io::Error> {
    let file = OpenOptions::new().read(true).write(false).create(false).open(path)?;
    let mut contents = String::new();
    io::BufReader::new(file).read_to_string(&mut contents)?;

    Ok(serde_json::from_str::<HashMap<String, Value>>(&contents).unwrap())
}

fn read_input_file(path: &str) -> Result<Vec<String>, io::Error> {
    let file = OpenOptions::new().read(true).write(false).create(false).open(path)?;
    io::BufReader::new(file).lines().collect()
}

pub fn import_magic<T, F>(day: i32, parse_input_function: F) -> Result<PuzzleOptions<T>, io::Error>
where
    T: Debug + Clone + PartialEq,
    F: Fn(Vec<String>, &HashMap<String, String>, bool) -> T,
{
    let args = args(day).get_matches();
    import_from_arg_matches(day, parse_input_function, args)
}

pub fn import_magic_with_params<T, F>(day: i32, parse_input_function: F, params: &[&str])
-> Result<PuzzleOptions<T>, io::Error>
where
    T: Debug + Clone + PartialEq,
    F: Fn(Vec<String>, &HashMap<String, String>, bool) -> T,
{
    let args = args(day).get_matches_from(params);
    import_from_arg_matches(day, parse_input_function, args)
}

fn import_from_arg_matches<T, F>(day: i32, parse_input_function: F, args: ArgMatches)
-> Result<PuzzleOptions<T>, io::Error>
where
    T: Debug + Clone + PartialEq,
    F: Fn(Vec<String>, &HashMap<String, String>, bool) -> T,
{
    PuzzleOptions {
        day,
        input_name: args.value_of("input").unwrap().to_owned(),
        verbose: args.is_present("verbose"),
        skip_p1: args.is_present("skip_p1"),
        data: None,
        config: match args.values_of("config") {
            Some(config_options) => config_options
                .collect::<Vec<&str>>()
                .as_slice()
                .chunks(2)
                .map(|list| (list[0].to_owned(), list[1].to_owned()))
                .collect(),
            _ => HashMap::new(),
        },
    }
    .with_input_parsed(parse_input_function)
}