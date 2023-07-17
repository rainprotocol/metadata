use clap::Parser;

#[derive(Parser,Clone)]
pub struct DISpair {
    pub interpreter : Option<String> ,
    pub store : Option<String> ,
    pub deployer : Option<String> ,
}  

impl DISpair {
    pub fn new(i : Option<String>, s : Option<String>, d : Option<String>) -> DISpair {
        DISpair { interpreter: i, store: s, deployer: d }
    }
}
 
// Replace all the origin network DISpair contracts instances by 
// DISpair instances of target network 
pub fn replace_dis_pair(
    tx_data : &String ,
    from_dis : &DISpair , 
    to_dis : &DISpair
) -> Option<String> { 

   let mut ret_str = tx_data.to_lowercase() ;  

   // Both the counterparties should be provided
   if from_dis.interpreter.as_ref().is_some() && to_dis.interpreter.as_ref().is_some() {
        if tx_data.contains(&from_dis.interpreter.as_ref().unwrap()[2..].to_lowercase()){
            ret_str = ret_str.replace(&from_dis.interpreter.as_ref().unwrap()[2..].to_lowercase(), &to_dis.interpreter.as_ref().unwrap()[2..].to_lowercase()) ; 
        }
   } 
   // Both the counterparties should be provided
   if from_dis.store.is_some() && to_dis.store.is_some() {
        if tx_data.contains(&from_dis.store.as_ref().unwrap()[2..].to_lowercase()){
            ret_str = ret_str.replace(&from_dis.store.as_ref().unwrap()[2..].to_lowercase(), &to_dis.store.as_ref().unwrap()[2..].to_lowercase()) ; 
        }
   }
   // Both the counterparties should be provided
   if from_dis.deployer.is_some() && to_dis.deployer.is_some() { 
        if tx_data.contains(&from_dis.deployer.as_ref().unwrap()[2..].to_lowercase()){
            ret_str = ret_str.replace(&from_dis.deployer.as_ref().unwrap()[2..].to_lowercase(), &to_dis.deployer.as_ref().unwrap()[2..].to_lowercase()) ; 
        }
   }
    
    Some(ret_str)
}

