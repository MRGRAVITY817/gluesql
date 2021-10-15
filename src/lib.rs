//! # GlueSQL
//!
//! `gluesql` is a SQL database engine fully written in Rust.
//! You can simply use `gluesql` as an embedded SQL database using default storage
//! [sled](https://crates.io/crates/sled).
//! Or you can make your own SQL database using `gluesql`, it provides parser & execution layer as
//! a library.
//!
//! `gluesql` uses [sqlparser-rs](https://crates.io/crates/sqlparser) as a parser, and has own implementation of execution layer.
//! And the entire codes of execution layer are pure functional!
//!
//! ## Examples
//!
//! ```
//! use gluesql::*;
//!
//! #[cfg(feature = "sled-storage")]
//! fn main() {
//!     let storage = SledStorage::new("data/doc-db").unwrap();
//!     let mut glue = Glue::new(storage);
//!     
//!     let sqls = vec![
//!         "DROP TABLE IF EXISTS Glue;",
//!         "CREATE TABLE Glue (id INTEGER);",
//!         "INSERT INTO Glue VALUES (100);",
//!         "INSERT INTO Glue VALUES (200);",
//!         "SELECT * FROM Glue WHERE id > 100;",
//!     ];
//!
//!     for sql in sqls {
//!         let output = glue.execute(sql).unwrap();
//!         println!("{:?}", output)
//!     }
//! }
//!
//! #[cfg(not(feature = "sled-storage"))]
//! fn main() {}
//! ```
//!
//! ## Custom Storage
//! To get started, all you need to implement for `gluesql` is implementing three traits
//! (two for functions, one running tests).
//! There are also three optional traits (`AlterTable`, `Index`, `IndexMut`, and `Transaction`),
//! whether implementing it or not is all up to you.
//!
//! ### Store traits
//! * [Store](store/trait.Store.html)
//! * [StoreMut](store/trait.StoreMut.html)
//!
//! ### Optional Store traits
//! * [AlterTable](store/trait.AlterTable.html)
//! * [Index](store/trait.Index.html)
//! * [IndexMut](store/trait.IndexMut.html)
//! * [Transaction](store/trait.Transaction.html)
//!
//! ### Trait to run integration tests
//! * [Tester](tests/trait.Tester.html)
//!
//! ## Tests
//! `gluesql` provides integration tests as a module.
//! Developers who wants to make their own custom storages can import and run those tests.
//! `/tests/` might look quite empty, but actual test cases exist in `src/tests/`.
//!
//! Example code to see,
//! * [tests/sled_storage.rs](https://github.com/gluesql/gluesql/blob/main/tests/sled_storage.rs)
//!
//! After you implement `Tester` trait, the only thing you need to do is calling `generate_tests!` macro.

mod executor;
mod glue;
mod parse_sql;
mod storages;
mod utils;

pub mod ast;
pub mod data;
pub mod plan;
pub mod result;
pub mod store;
pub mod tests;
pub mod translate;

pub mod prelude {
    pub use crate::data::value::Value;
    pub use crate::glue::Glue;
    pub use crate::result::Error;
    pub use crate::result::Result;
    pub use crate::storages::{MemoryStorage, SledStorage};
}

pub use prelude::*;

pub mod test {
    pub use crate::ast::{ColumnDef, DataType, IndexOperator::Eq};
    pub use crate::data::{
        value::{
            Value::{Bool, Interval, Null, Str, F64, I64},
            ValueError,
        },
        IntervalError, Literal,
    };
    pub use crate::executor::{AlterError, EvaluateError, FetchError, Payload, ValidateError};
    pub use crate::parse_sql::parse_expr;
    pub use crate::store::{AlterTableError, GStore, GStoreMut, IndexError};
    pub use crate::translate::{translate_expr, TranslateError};
}
