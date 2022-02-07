use std::mem;
pub fn run()
{
  let mut numbers:Vec<i32>=vec![1,2,3,4];
  
  // println!("{:?}",numbers);

 //Re-assign value
  numbers[2]=20;

  // println!("{:?}",numbers);
  
  //Add on to vector
  numbers.push(5);
  numbers.push(6);
  
  // println!("{:?}",numbers);
  
  
  //Pop off last value
  numbers.pop();
  println!("{:?}",numbers);

  //Get single val
  println!("Single vale: {}",numbers[0]);

  //Get Vector length
  println!("Array Length: {}",numbers.len());

  //Vector are stack allocated
  println!("Array occupies {} bytes",mem::size_of_val(&numbers));

  //Get Slice
  let slice:&[i32]=&numbers[1..3];
  println!("Slice : {:?}",slice);

  //Loop through vector values
  for x in numbers.iter(){
    println!("numbers: {}",x);
  }
    //Loop & mutate values
    for x in numbers.iter_mut(){
      *x *= 2;

    }
    println!("Numbers Vec: {:?}",numbers);
  

}