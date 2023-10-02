use serde::Serialize;

use super::data::*;

pub enum Instructions<T: Serialize> {
    Select(SelectData),
    Update(UpdateData<T>),
    Delete(DeleteData),
    Insert(InsertData<T>),
    CreateTable(CreateTable),
}
