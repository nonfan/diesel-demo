use diesel::pg::{Pg, PgValue};
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::VarChar;
use diesel::AsExpression;
use diesel::deserialize::{FromSql, Result as DeserializeResult};

// 自定义类型
#[derive(Debug, PartialEq, AsExpression)]
#[diesel(sql_type = VarChar)]
pub struct Email(String);
impl Email {
    pub fn new(val: &str) -> Self {
        Email(val.into())
    }
}
impl ToSql<VarChar, Pg> for Email {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        <String as ToSql<VarChar, Pg>>::to_sql(&self.0, out)
    }
}
// 从数据库读取
impl FromSql<VarChar, Pg> for Email {
    fn from_sql(bytes: PgValue<'_>) -> DeserializeResult<Self> {
        let s = std::str::from_utf8(bytes.as_bytes())?;
        Ok(Email::new(s))
    }
}
