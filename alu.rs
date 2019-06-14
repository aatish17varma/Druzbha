// use std::ptr;
mod phv_container;
// use std::fmt;
use std::collections::HashMap;


//Use a PHV Container as an object containing all states - *just for now*
//return the previous states, unmodified or not
struct ALU<'a>{
    states: &'a mut phv_container::PhvContainer,
    sequential_function: Box<fn(&mut phv_container::PhvContainer) -> &mut phv_container::PhvContainer>,
    is_stateful: bool,
}

impl<'a> ALU<'a>{
    //constructor with an intial value for is_stateful hard coded
    // fn new(stateful: bool, passed_states: &mut phv_container::PhvContainer, pass_func: Box<fn(&mut phv_container::PhvContainer) -> &mut phv_container::PhvContainer>) -> Self{
    //     ALU{states: &mut phv_container::PhvContainer::new(), sequential_function: pass_func, is_stateful: stateful}
    // }
    //stateful constructor with vector of passed_states
    /*Think about using a phv instead of a vec<i32> of passed states */
    fn new_with_states_and_function(passed_states: &mut phv_container::PhvContainer, stateful: bool, func_to_execute: Box<fn(&mut phv_container::PhvContainer) -> &mut phv_container::PhvContainer>){
        ALU{states: passed_states, sequential_function: func_to_execute, is_stateful: stateful};
    }

    fn operator(&mut self) -> &mut phv_container::PhvContainer {
        return (self.sequential_function)(self.states);
        // <phv_container::PhvContainer as fmt::Display>::fmt(self.states, &mut fmt::Formatter);
    }

}

fn main(){
    println!("hello world");
    let mut hash = HashMap::new();
    hash.insert("state1".to_string(), 1);
    hash.insert("state2".to_string(), 2);
    let phv  = &mut phv_container::PhvContainer{map: hash};

    fn seq(phv : &mut phv_container::PhvContainer) -> &mut phv_container::PhvContainer{
        //simple stateful function that adds one to the first state value. 
       phv.map.insert("state1".to_string(), phv.map["state1"] + 1);
        
       return phv;
    }



    println!("Value of {state} is {value} ", state = "state1", value = phv.map["state1"]);
    
    {

    let mut alu : ALU = ALU{states: phv, sequential_function: Box::new(seq), is_stateful: true};
    
    alu.operator(); 

    }

    println!("Value of {state} is {value} ", state = "state1", value = phv.map["state1"]);

    


   
}
