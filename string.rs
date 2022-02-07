pub fn run()
{
  let mut  hello=String::from("Hello ");
  
  //Get length
  println!("Length: {}",hello.len());

  //Psuh char
  hello.push('W');

  //Psuh string
  hello.push_str("orld!");

  //Capacity in bytes
  println!("Capacity: {}",hello.capacity());

  //Is empty
  println!("Is Empty: {}",hello.is_empty());

  //Contian

  println!("Constains 'World' {}",hello.contains("World"));

  //Replace
  println!("Replace : {}",hello.replace("World", "There"));

  //loop through string by whitespace
  for word in hello.split_whitespace(){
    println!("{}",word);
  }

  //Create string with capacity
  let mut s=String::with_capacity(10);
  s.push('a');
  s.push('b');

  println!("{}",s);

  //Assertion testing
  assert_eq!(2,s.len());
  assert_eq!(10,s.capacity());

  // println!("{}",hello);


}