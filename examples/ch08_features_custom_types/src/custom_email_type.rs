use diesel::pg::{Pg, PgValue};
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::VarChar;
use diesel::AsExpression;
use diesel::deserialize::{FromSql, Result as DeserializeResult};
use diesel::expression::AsExpression;

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
use diesel::expression::bound::Bound;

// 让 Email 支持 Diesel 查询表达式
impl AsExpression<VarChar> for Email {
    type Expression = Bound<VarChar, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

// 引用类型支持表达式
impl<'a> AsExpression<VarChar> for &'a Email {
    type Expression = Bound<VarChar, &'a Email>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}
