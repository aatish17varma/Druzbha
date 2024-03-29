// TODO: File representing a PHV container
/*
- A glorified HashMap -> ["field1": 3232323, ]


is there a need for a "FieldName" type which is basically a string
*/



// use std::collections::HashMap;

// struct Container{
//  map: HashMap<String, i64>,
// }

// impl Container{
//     //it is just generally better to use &str
//     fn operator(&self, field_name : &str) -> i64 {
//         assert!(self.map.contains_key(field_name));
//         return self.map[field_name];
//     }

//     fn operator_reference(&self, field_name : &str) -> &i64 {
//         assert!(self.map.contains_key(field_name));
//         return &self.map[field_name];
//     }

//     fn mergeContainers(&self, new_container: &Container) -> &Container{

//     }

//     fn print(self) -> String {
//         let text : &str = "";
//         for(name, value) in self.map {
//             &[text, &format!("({n},{v}), ", n=name, v=value)];
//         }
//         return text.to_string(); 
//     }

//     // fn field_list(&self) -> Vec<String> {

//     // }
// }

// fn main(){

// }



// TODO: File representing a PHV container
/*
- A glorified HashMap -> ["field1": 3232323, ]


is there a need for a "FieldName" type which is basically a string
*/

use std::ops::{Index, IndexMut, AddAssign};
use std::fmt;
use std::collections::HashMap;

#[derive(Clone)]

pub struct PhvContainer{
  pub map: HashMap<String, i32>,
}

impl PhvContainer{

  // Constructor that initializes new PhvContainer with new
  // HashMap.
  pub fn new () -> Self {
    let h : HashMap <String, i32> = HashMap::new();
    PhvContainer { map : h }
  }

  // Alternate constructor that takes in a HashMap for use
  // in initializing PhvContainer.
  pub fn with_map (h : &HashMap <String, i32>) -> Self{
    PhvContainer { map : h.clone() }
  }

   pub fn field_list(&self) -> Vec<String> {
     let mut field_names : Vec <String> = Vec::new();
     for (name, &value) in self.map.iter() {
       field_names.push (name.clone());
     }
     field_names
   }
}

// Overloads the index operator: [ ]. Enables packet 
// fields to be attained by using pc [fn], where fn
// is the field name and pc is the PhvContainer.
impl Index<&str> for PhvContainer{
  type Output=i32;
  fn index (&self, idx : &str) -> &i32{

    assert!(self.map.contains_key(idx));
    &self.map[idx]
  }
}

// Overloads the index operator: [ ]. Enables packet
// field values to be mutated by returning a mutable
// reference. pc [fn] = value where fn is the fieldname,
// value is the new value, and is the PhvContainer
impl IndexMut<&str> for PhvContainer{
  fn index_mut (&mut self, idx : &str) -> &mut i32 {
  
    // Matches against an option
    match self.map.get_mut (idx) {

      Some (_) => self.map.get_mut(idx).unwrap(),
      None     =>  {
        // Will get an error if return None, so insert
        // 0 as a "placeholder" before returning mutable
        // reference
        self.map.insert(idx.to_string(), 0);
        self.map.get_mut(idx).unwrap()
      },
    }
  }
}

// Overloads the += operator. It adds the fields from the
// given PhvContainer into the current PhvContainer assuming
// that there are no conflicting values.
impl AddAssign for PhvContainer {
  fn add_assign (&mut self, t_container : Self){

     let mut field_names : HashMap <String,i32> = self.map.clone();
     // Check that the current container's key/value pairs differ
     // from the key/value pairs in t_container. If the same key
     // is in both containers, verify that the values corresponding
     // to that key are the same. If not, then exit.
     for (name, &value) in t_container.map.iter() {
       if self.map.contains_key (name){
         if self[name] != t_container[name] {
           panic!("Values of containers for key {} do not match", name)
         }
       }
     }
     // Repeat for other container
     for (name, &value) in self.map.iter() {
       if t_container.map.contains_key (name){
         if self[name] != t_container[name] {
           panic!("Values of containers for key {} do not match", name)
         }
       }
     }
    // Add all values to field_names which will replace current
    // HashMap
    for (name, &value) in t_container.map.iter() {
      field_names.insert(name.clone(), value.clone());
    }
    *self = PhvContainer { map : field_names };
  }
}

// Uses the fmt functon as part of the Display trait. Allows
// PhvContainer to be printed with println!("{}", p) where p is a 
// PhvContainer. Using Display trait also enables the to_string 
// method to be implemented automatically. Prints the HashMap
// keys along with its corresponding values
impl fmt::Display for PhvContainer {
  fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
      
    let mut s : String = String::from("");
    s.push_str("(");

    for (name, &value) in self.map.iter() {
      s.push_str(name);
      s.push_str(" : ");
      s.push_str(&value.to_string());
      s.push_str(", ");
    }

    s.push_str(")");
    write!(f, "{}", s)
  }
}

