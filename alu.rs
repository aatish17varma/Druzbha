// use std::ptr;
mod phv_container;
use std::fmt;


//Use a PHV Container as an object containing all states - *just for now*
//return the previous states, unmodified or not
struct ALU{
    states: phv_container::PhvContainer,
    sequential_function: Box<fn(&phv_container::PhvContainer) -> phv_container::PhvContainer>,
    is_stateful: bool,
}

impl ALU{
    //constructor with an intial value for is_stateful hard coded
    fn new(stateful: bool, passed_states: phv_container::PhvContainer, pass_func: Box<fn(&phv_container::PhvContainer) -> phv_container::PhvContainer>) -> Self{
        ALU{states: phv_container::PhvContainer::new(), sequential_function: pass_func, is_stateful: stateful}
    }
    //stateful constructor with vector of passed_states
    /*Think about using a phv instead of a vec<i32> of passed states */
    fn new_with_states_and_function(passed_states: phv_container::PhvContainer, stateful: bool, func_to_execute: Box<fn(&phv_container::PhvContainer) -> phv_container::PhvContainer>){
        ALU{states: passed_states, sequential_function: func_to_execute, is_stateful: stateful};
    }

    fn operator(&self) {
        (self.sequential_function)(&self.states);
        // <phv_container::PhvContainer as fmt::Display>::fmt(self.states, &mut fmt::Formatter);
    }

}


fn main(){
    println!("hello world");

}
