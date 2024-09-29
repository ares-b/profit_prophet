use crate::field_protocol::FieldValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct PostgresData {
    pub table_name: String,
    pub schema: String,
    pub rows: HashMap<String, FieldValue>,
}

impl PostgresData {
    pub fn to_sql_insert(&self) -> String {
        let columns = self.rows.keys().cloned().collect::<Vec<_>>().join(", ");
        let values = self.rows.values().map(|v| match v {
            FieldValue::String(s) => format!("'{}'", s),
            FieldValue::F64(n) => n.to_string(),
            FieldValue::I64(n) => n.to_string(),
            FieldValue::Bool(b) => b.to_string(),
        }).collect::<Vec<_>>().join(", ");

        format!(
            "INSERT INTO {}.{} ({}) VALUES ({});",
            self.schema, self.table_name, columns, values
        )
    }
}
