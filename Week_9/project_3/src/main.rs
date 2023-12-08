use std::io::Write;

fn main() {

    println!("*********Welcome to the efcc file writing engine*********");
    let mut file = std::fs::File::create("EFCC.txt").expect("EFCC.txt");

    let name = ["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry = ["Internal Affairs", "Justice", "Defense","Power & Steel","Petroleum"];
    let geo = ["South West", "North East", "South South","South West","South East"];
    let no = [1, 2, 3, 4, 5];

    writeln!(file,"{:<20} {:<20} {:<30} {:<10}","S/N","NAME OF COMMISIONER","MINISTRY","GEOPOLITICAL ZONE");

    for i in 0..name.len() {
        writeln!(file,"{:<20} {:<20} {:<30} {:<10}", no[i], name[i], ministry[i],geo[i]).expect("Couldn't write to file");
    }

    println!("*********File has been created*********");
}
