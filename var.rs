pub fn run(){
  let name="brad";
  let mut age=37;
  println!("My name is {} and I Am {} ",name,age);
  age=38;
  println!("My name is {} and I Am {} ",name,age);

  //Define const
  const ID:i32=001;
  println!("ID: {}",ID);
  
  //Assign multiple nvars
  let (my_name,my_age)=("Brad", 37);
  println!("{} is {}",my_name,my_age);
}