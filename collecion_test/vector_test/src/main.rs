fn main() {
   let v: Vec<i32> = Vec::new();

   let v2  = vec![1, 2, 3];
   let mut v3 = Vec::new();
   v3.push(1);
   v3.push(2);
   v3.push(3);

   println!("this vector {}", &v3[2]);

   let mut v4 = vec![1, 2, 3, 4, 5];

   let third : &i32 = &v4[2];
   println!("The element is {}", third);

   v4.push(6);

   let third : Option<&i32> = v4.get(2);

   match third {
      Some(third) => println!("The element is {}", third),
      None => println!("There is no third element"),
   }

   let mut v5 = vec![100, 32, 57];

   for i in &v5 {
      println!("{i}");
   }

   let mut v6 = vec![100, 32, 57];
   for i in &mut v6 {
      *i += 50;
      println!("{i}");
   }
   
}
