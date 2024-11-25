use crate::auth_utils::models::Credentials;



    pub enum Status {
        Connected,
        Interrupted
    }

    pub fn connect_to_database()->Status{
        Status::Connected
    }

    pub fn get_user(cred:Credentials){
         println!("{}, {}", cred.username, cred.password)
    }

