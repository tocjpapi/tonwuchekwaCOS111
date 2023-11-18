use std::io;

fn main() {
    let mut count = 0;
    loop {
        count+=1;
     
     let mut name = String ::new();
     println!("What is your name?");
     io::stdin().read_line(&mut name).expect("Please use letters or Strings");

     let mut papers = String ::new();
     println!("How many papers have you published?");
     io::stdin().read_line(&mut papers).expect("Please use letters or Strings");
     let papers:f32 = papers.trim().parse().expect("Invalid Float");


     let incentive1 = 500_000;

     let incentive2 = 800_000;

     let incentive3 = 1_000_000;

     let incentive4 = 100_000;

     if papers >= 3.0 && papers <= 5.0 {
        println!("Dear {} you have an incentive of {}",name , incentive1);
     }

     else if papers > 5.0 && papers < 10.0{
         println!("Dear {} you have an incentive of {}",name , incentive2);
     }

     else if papers >= 10.0 {
         println!("Dear {} you have an incentive of {}",name , incentive3);
     }

     else if papers < 3.0 {
        println!("Dear {} you have an incentive of {}",name , incentive4);
     }
     



        if count == 500 {
            break;
        }
        
    }
}
