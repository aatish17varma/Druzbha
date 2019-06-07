// TODO: File representing a PHV container
/*
- A glorified HashMap -> ["field1": 3232323, ]


is there a need for a "FieldName" type which is basically a string
*/



use std::collections::HashMap;

struct Container{
 map: HashMap<String, i64>,
}

impl Container{
    //it is just generally better to use &str
    fn operator(&self, field_name : &str) -> i64 {
        assert!(self.map.contains_key(field_name));
        return self.map[field_name];
    }

    fn operator_reference(&self, field_name : &str) -> &i64 {
        assert!(self.map.contains_key(field_name));
        return &self.map[field_name];
    }

    fn mergeContainers(&self, new_container: &Container) -> &Container{

    }

    fn print(self) -> String {
        let text : &str = "";
        for(name, value) in self.map {
            &[text, &format!("({n},{v}), ", n=name, v=value)];
        }
        return text.to_string(); 
    }

    // fn field_list(&self) -> Vec<String> {

    // }
}

fn main(){

}