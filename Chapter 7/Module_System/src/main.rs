
use module_system::auth_utils::models::Credentials;
use module_system::authenticate;


fn main() {
    let cred = Credentials {
        username : String::from("Harsh"),
        password : String::from("12345")
    };

    authenticate(cred);
}
