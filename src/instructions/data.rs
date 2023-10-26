use serde::Serialize;

pub struct SelectData {
    pub table: String,
    pub filters: Option<Vec<String>>,
}

pub struct UpdateData<T: Serialize> {
    table: String,
    filters: Option<Vec<String>>,
    new_values: T,
}

pub struct DeleteData {
    table: String,
    filters: Option<Vec<String>>,
}

pub struct InsertData<T: Serialize> {
    table: String,
    value: T,
}

pub struct CreateTable {
    pub table: String,
}
