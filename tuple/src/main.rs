fn main() {
   let my_tuple:(i32,f64,char)=(500,6.4,'R');
 //Destructring
    let(x,y,z)=my_tuple;
    println!("x = {}, y = {}, z = {}",x,y,z);
    //Direct access;
    let first=my_tuple.0;
    let second=my_tuple.1;
    let third=my_tuple.2;
    println!("first = {}, second = {}, third = {}",first,second,third);
    let (a,b,c)=calculate_sum_sub_multiply(10,20);
 println!("a = {}, b = {}, c = {}", a,b,c);

}

//Another example to return tuple form function
fn calculate_sum_sub_multiply(v1:i32,v2:i32)->(i32,i32,i32){
 return (v1+v2,v1-v2,v1*v2);
}