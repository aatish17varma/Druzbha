// TODO: File representing stateful and stateless ALUs.


//A Stateful ALU


struct ALU(&mut Vec<i32> global_states){
    states: &mut Vec<i32>, 
    sequential_function: Box(fn(i32, Vec<i32>)) -> Vec<i32>),
    is_stateful: bool
}

impl StatefulAlu{
    //Add a constructor with an intial value for is_stateful hard coded

    fn operator(&self, packet: &packet::packet){
        self.sequential_function()
    }

}


fn main(){
    let vec: &mut Vec<i32> = Vec::new();
    
    fn temp_function(){

    }

    let temp_ALU = ALU{states: vec, sequential_function}

}
