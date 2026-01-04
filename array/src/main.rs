fn main() {
  let array:[i32;5] = [1,2,3,4,5];
    println!("array[0] = {} ,array[1] = {} ,array[2] = {} ,array[3] = {} ,array[4] = {} ",array[0],array[1],array[2],array[3],array[4]);
   let slice=&array[0..4];
    println!("Original array = {:?}", array);
    println!("Slice = {:?}", slice);

}
