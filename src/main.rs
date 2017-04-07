mod parser {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufRead;

    struct StatsRecord {
        header: Vec<String>
    }

    pub trait CrimeStatParser {
        fn parse(file_name: &str) -> Option<Vec<String>>;
    }

    pub struct CrimeStatsCsvParer;

    impl CrimeStatParser for CrimeStatsCsvParer {
        fn parse(file_name: &str) -> Option<Vec<String>> {
            let fh = File::open(file_name).unwrap();
            let mut file = BufReader::new(&fh);
            for (no, line) in file.lines().enumerate() {
                if no == 0 {
                    let l = line.unwrap();
                    let fields: Vec<String> = l.split(",").map(|x: &str| {
                        String::from(x)
                    }).collect();
                    return Some(fields);
                }
            }
            return None
        }
    }
}

fn main() {
    use parser::CrimeStatParser;
    let s = parser::CrimeStatsCsvParer::parse("data/crime-data-nsw.csv");
    match s {
        Some(fields) => {
            println!("Number of fields is {} and field no  1 : {} and 2:{}",fields.len(),fields[0],fields[1]);
        },
        None => println!("No fields found"),
    }
}

