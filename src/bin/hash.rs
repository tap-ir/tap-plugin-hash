//! Hash is a binary that take a file in input and calculate it's Sha1 hash 
extern crate tap_plugin_hash;

use std::env;
use std::fs::File;

use tap_plugin_hash::Hash;

fn main() 
{
   if env::args().len() != 2 
   {
     println!("hash input_file");
     return ;
   }

   let args: Vec<String> = env::args().collect();
   let file_path = &args[1];

   match File::open(file_path)
   {
      Err(_) => println!("Can't open file {}", file_path),
      Ok(file) => 
      {
         let hash = Hash{};
         println!("{}  {}", hash.hash(Box::new(file)), file_path);
      },
   }
}
