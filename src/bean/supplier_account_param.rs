use std::collections::HashSet;

#[derive(Debug)]
pub struct SupplierAccountParam {
    pub supplier_codes: Option<Vec<String>>,

    pub r#type: Option<i16>,

    pub types: Option<Vec<i16>>,

    pub supplier_code_not_in: Option<Vec<String>>,

    pub data_states: Option<HashSet<i32>>,
    pub data_state: Option<i32>,

    pub offset: Option<i32>,

    pub limit: Option<i32>,
}
