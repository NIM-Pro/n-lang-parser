use database::Connection;
use database::Users::Entity;

fn main() {
    let con = Connection::new("mysql:127.0.0.1")?;
    let users = con.GetUsersByAge(19, 25)?;
    for user in users {
        println!("{} {}", user.info.first_name, user.info.last_name);
    }
}
