use crate::bean::page_request::Page;
use crate::bean::page_request::PageRequest;
use crate::impl_page_request;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct SupplierAccountParam {
    pub supplier_codes: Option<Vec<String>>,

    pub r#type: Option<i16>,

    pub types: Option<Vec<i16>>,

    pub supplier_code_not_in: Option<Vec<String>>,

    pub data_states: Option<HashSet<i32>>,
    pub data_state: Option<i32>,

    pub page: Option<Page>,
}

impl_page_request!(SupplierAccountParam);
