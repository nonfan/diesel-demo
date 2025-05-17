use ch08_features_custom_types::{custom_email_type::Email, establish_connection, models::User, schema};
use diesel::prelude::*;

fn main() {
    use schema::users::dsl::users;

    let conn = &mut establish_connection();

    let email = Email::new("test@test.com");
    let new_user = User { id: 1, email };
    // let result = diesel::insert_into(users)
    //     .values(&new_user)
    //     .get_result::<User>(conn);
    //
    // match result {
    //     Ok(data) => println!("user: {:?}", data.email),
    //     Err(_) => println!("couldn't insert user"),
    // }
}
