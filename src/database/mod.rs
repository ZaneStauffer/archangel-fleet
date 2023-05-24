

pub mod db{

    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::Read;
    use std::fmt;
    use std::result;
    use struct_db::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[struct_db(
        fn_primary_key(p_key),  // required
        //fn_secondary_key(s_key),  // optional
        // ... other fn_secondary_key ...
    )]
    pub struct Agents{
        symbol: String,
        token: String
    }

    impl Agents{
        pub fn p_key(&self) -> Vec<u8>{
            self.symbol.as_bytes().to_vec()
        }
    }

    // Update the database
    pub fn update<T>(db: &Db, datum: T) -> std::result::Result<(), Box<dyn std::error::Error>>{
        // update calls
            
        Ok(())
    }

    pub fn insert<T>(db: &Db, datum: T) -> std::result::Result<(), Box<dyn std::error::Error>>{
        let txn = db.transaction()?;

        Ok(())
    }

    pub fn init_db(name: &str) -> Result<Db>{
        println!("> Configuring LATTICE storage network...");
        println!(">> LATTICE INITIALIZED <<");
        let mut db = Db::init_tmp(name).unwrap();
        // Initiate schemas (do this for each struct)
        db.define::<Agents>();

        Ok(db)
    }

}