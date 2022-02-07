pub fn run()
{
  //Print to console
  println!("Hello from the print rs file");


  //Basic Formatting
  println!("{} is from {}","Brad","Mass");


  //Positional Argumenets

  println!("{0} is from {1} and {0} likes to {2}","Brad","Mass","code");

  //Named Arguments
  println!("{name} likes to play {activity}",name="Jhon",activity="Baseball");

  //PlaceHolder Traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10);

  //PlaceHolder for debug trait
  println!("{:?}",(12,true,"Hello"));

  //Basic Math
  println!("10+10={}",10+10);
}