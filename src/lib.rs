use std::env;

pub fn read_csv() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("./data/nba-2022-2023-schedule.csv").unwrap();
    for result in rdr.records() {
        print!("{:?}", result.unwrap())
    }
    print!("Wow!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading() {
        read_csv()
    }
}
