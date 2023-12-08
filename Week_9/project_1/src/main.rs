
use std::io::Write;

fn main() {
    
    let mut file = std::fs::File::create("project.txt").expect("Couldn't create");


    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams", "", "", ""];
    let non = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz", "", ""];

    file.write_all("Lager\t         Stout\t        Non-Alcoholic".as_bytes());

    for i in 0..lager.len() {
        file.write_all(
            format!(
            "{}\t{}\t{}",
            lager[i],
            stout.get(i).unwrap_or(&""),
            non.get(i).unwrap_or(&"")
        ).as_bytes(),
        );
    }
}
