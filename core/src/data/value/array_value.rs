use {
    super::{Interval, Value},
    crate::data::point::Point,
    chrono::{NaiveDate, NaiveDateTime, NaiveTime},
    rust_decimal::Decimal,
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, net::IpAddr},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArrayValue {
    Bool(Vec<bool>),
    I8(Vec<i8>),
    I16(Vec<i16>),
    I32(Vec<i32>),
    I64(Vec<i64>),
    I128(Vec<i128>),
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    U64(Vec<u64>),
    U128(Vec<u128>),
    F32(Vec<f32>),
    F64(Vec<f64>),
    Decimal(Vec<Decimal>),
    Str(Vec<String>),
    Bytea(Vec<Vec<u8>>),
    Inet(Vec<IpAddr>),
    Date(Vec<NaiveDate>),
    Timestamp(Vec<NaiveDateTime>),
    Time(Vec<NaiveTime>),
    Interval(Vec<Interval>),
    Uuid(Vec<u128>),
    Map(Vec<HashMap<String, Value>>),
    List(Vec<Vec<Value>>),
    Array(Vec<ArrayValue>),
    Point(Vec<Point>),
    Null,
}