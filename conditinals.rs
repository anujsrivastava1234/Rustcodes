pub fn run(){
  let age:u8=22;
  let check_id: bool=true;
  let knows_person_of_age=true;

  //IF/ELSE
  if age>=21 && check_id || knows_person_of_age{
    println!("Bartender: What would you like to drink?");
  }else if age<21 && check_id{
    println!("Bartender: Sorry, you hjave to leave");
  }else{
    println!("Bartender: I'll need to see your ID");
  }


  //ShortHand
  let is_of_age=if age>=21{true}else{false};
  println!("IS of age: {} ",is_of_age);
}