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

#[cfg(test)] 
mod test {  
    use crate::deploy::transaction::get_transaction_data;
    use crate::deploy::registry::RainNetworks;
    use crate::deploy::dis::DISpair;
    use super::replace_dis_pair;



    #[tokio::test]
    async fn test_replace_no_dis() { 

        let tx_hash = String::from("0xea76ed73832498c4293aa06aeca2899f2b5adca15d703b03690185ed829f3e71") ;  
        let network = RainNetworks::Mumbai ;  
        let tx_data = get_transaction_data(&network, &tx_hash).await.unwrap() ; 

        let from_dis = DISpair {
            interpreter : None ,
            store : None , 
            deployer : None
        } ; 

        let to_dis = DISpair {
            interpreter : None ,
            store : None , 
            deployer : None
        } ; 

        let replaced_data = replace_dis_pair(
            &tx_data,
            &from_dis,
            &to_dis
        ).unwrap() ;

        assert_eq!(tx_data, replaced_data);
    }

    #[tokio::test]
   async fn test_replace_only_from_dis() { 

        let tx_hash = String::from("0xc215bf3dc7440687ca20e028158e58640eeaec72d6fe6738f6d07843835c2cde") ;  
        let network = RainNetworks::Mumbai ;  
        let tx_data = get_transaction_data(&network, &tx_hash).await.unwrap() ; 

        let from_dis = DISpair {
            interpreter : Some(String::from("0x5f02c2f831d3e0d430aa58c973b8b751f3d81b38")),
            store : Some(String::from("0xa5d9c16ddfd05d398fd0f302edd9e9e16d328796")),  
            deployer : Some(String::from("0xd3870063bcf25d5110ab9df9672a0d5c79c8b2d5")),
        } ; 

        let to_dis = DISpair {
            interpreter : None ,
            store : None , 
            deployer : None
        } ; 

        let replaced_data = replace_dis_pair(
            &tx_data,
            &from_dis,
            &to_dis
        ).unwrap() ;

        assert_eq!(tx_data, replaced_data);
    }

    #[tokio::test]
    async fn test_replace_only_to_dis() { 

        let tx_hash = String::from("0xc215bf3dc7440687ca20e028158e58640eeaec72d6fe6738f6d07843835c2cde") ;  
        let network = RainNetworks::Mumbai ;  
        let tx_data = get_transaction_data(&network, &tx_hash).await.unwrap() ; 

        let from_dis = DISpair {
            interpreter : None ,
            store : None , 
            deployer : None
        } ; 

        let to_dis = DISpair {
            interpreter : Some(String::from("0xfd1da7eee4a9391f6fcabb28617f41894ba84cdc")),
            store : Some(String::from("0x9b8571bd2742ec628211111de3aa940f5984e82b")),  
            deployer : Some(String::from("0x3d7d894afc7dbfd45bf50867c9b051da8eee85e9")),
        } ; 

        let replaced_data = replace_dis_pair(
            &tx_data,
            &from_dis,
            &to_dis
        ).unwrap() ;

        assert_eq!(tx_data, replaced_data);
    }

    #[tokio::test]
    async fn test_replace_from_to_dis() { 

        let tx_hash = String::from("0xebacdb3971924c9bbd2257334d436b4590d3d98f54969f6f942d6bd7a68da80b") ;   

        let network = RainNetworks::Mumbai ;  
        let tx_data = get_transaction_data(&network, &tx_hash).await.unwrap() ;  

        let to_network = RainNetworks::Fuji ; 
        let expexted_tx_hash = String::from("0xb0ae6ff12e9b810530e1b0844a448865cf4781950a90c99ba36f7f343e596717") ;   
        let expected_tx_data = get_transaction_data(&to_network, &expexted_tx_hash).await.unwrap() ;  


        let from_dis = DISpair {
            interpreter : Some(String::from("0x5f02c2f831d3e0d430aa58c973b8b751f3d81b38")) ,
            store : Some(String::from("0xa5d9c16ddfd05d398fd0f302edd9e9e16d328796")) , 
            deployer : None
        } ; 

        let to_dis = DISpair {
            interpreter : Some(String::from("0xfd1da7eee4a9391f6fcabb28617f41894ba84cdc")),
            store : Some(String::from("0x9b8571bd2742ec628211111de3aa940f5984e82b")),  
            deployer : None,
        } ; 

        let replaced_data = replace_dis_pair(
            &tx_data,
            &from_dis,
            &to_dis
        ).unwrap() ;

        assert_eq!(expected_tx_data, replaced_data);
    } 

    #[tokio::test]
    async fn test_replace_dis() { 

        let tx_hash = String::from("0xc215bf3dc7440687ca20e028158e58640eeaec72d6fe6738f6d07843835c2cde") ;   

        let network = RainNetworks::Mumbai ;  
        let tx_data = get_transaction_data(&network, &tx_hash).await.unwrap() ;  

        let to_network = RainNetworks::Fuji ; 
        let expexted_tx_hash = String::from("0x13b9895c7eb7311bbb22ef0a692b7b115c98c957514903e7c3a0e454e3389378") ;   
        let expected_tx_data = get_transaction_data(&to_network, &expexted_tx_hash).await.unwrap() ;  


        let from_dis = DISpair {
            interpreter : Some(String::from("0x5f02c2f831d3e0d430aa58c973b8b751f3d81b38")) ,
            store : Some(String::from("0xa5d9c16ddfd05d398fd0f302edd9e9e16d328796")) , 
            deployer : Some(String::from("0xd3870063bcf25d5110ab9df9672a0d5c79c8b2d5"))
        } ; 

        let to_dis = DISpair {
            interpreter : Some(String::from("0xfd1da7eee4a9391f6fcabb28617f41894ba84cdc")),
            store : Some(String::from("0x9b8571bd2742ec628211111de3aa940f5984e82b")),  
            deployer : Some(String::from("0x3d7d894afc7dbfd45bf50867c9b051da8eee85e9")),
        } ; 

        let replaced_data = replace_dis_pair(
            &tx_data,
            &from_dis,
            &to_dis
        ).unwrap() ;

        assert_eq!(expected_tx_data, replaced_data);
    } 



}