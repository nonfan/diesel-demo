use ch07_features_transaction::{establish_connection};

#[derive(Debug)]
enum MyError {
    DieselError(diesel::result::Error),
    PermissionDenied,
}

impl From<diesel::result::Error> for MyError {
    fn from(err: diesel::result::Error) -> Self {
        MyError::DieselError(err)
    }
}

fn main() {
    use diesel::prelude::*;
    let connection = &mut establish_connection();

    let result = connection.transaction::<_, MyError, _>(|conn| {
        // 在事务中执行数据库操作
        if 1 == 1 {
            return Err(MyError::PermissionDenied);
        }
        Ok(())
    });

    match result {
        Ok(_) => println!("Success"),
        Err(error) => println!("Error: {:?}", error),
    }
}
