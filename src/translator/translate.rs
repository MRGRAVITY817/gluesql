use crate::translator::{CommandType, Filter, Update};
use nom_sql::{DeleteStatement, SelectStatement, SqlQuery, UpdateStatement};

pub fn translate(sql_query: SqlQuery) -> CommandType {
    match sql_query {
        SqlQuery::CreateTable(statement) => CommandType::Create(statement),
        SqlQuery::Insert(statement) => CommandType::Insert(statement),
        SqlQuery::Select(SelectStatement {
            tables,
            where_clause,
            ..
        }) => {
            let table_name = tables
                .into_iter()
                .nth(0)
                .expect("SelectStatement->tables should have something")
                .name;
            let filter = Filter::from(where_clause);

            CommandType::Select(table_name, filter)
        }
        SqlQuery::Delete(DeleteStatement {
            table,
            where_clause,
        }) => {
            let table_name = table.name;
            let filter = Filter::from(where_clause);

            CommandType::Delete(table_name, filter)
        }
        SqlQuery::Update(UpdateStatement {
            table,
            fields,
            where_clause,
        }) => {
            let table_name = table.name;
            let update = Update::from(fields);
            let filter = Filter::from(where_clause);

            CommandType::Update(table_name, update, filter)
        }
        _ => {
            panic!("[translate.rs] query not supported");
        }
    }
}