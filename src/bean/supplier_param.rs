use crate::bean::page_request::{Page, PageRequest};
use std::collections::HashSet;
use crate::impl_page_request;

#[derive(Debug, Default)]
pub struct SupplierParam {
    pub supplier_name: Option<String>,

    pub supplier_type: Option<i32>,

    pub supplier_code: Option<String>,

    pub supplier_codes: Option<HashSet<String>>,

    pub one_time: Option<bool>,

    pub sign_contract: Option<bool>,

    pub product_code: Option<String>,

    pub source_system: Option<String>,

    pub product_codes: Option<HashSet<String>>,

    pub account_user_code: Option<String>,

    pub id_not_in: Option<Vec<i32>>,

    pub supplier_name_like: Option<String>,

    pub data_states: Option<HashSet<i16>>,

    pub data_state: Option<i16>,

    pub page: Option<Page>,
}

impl_page_request!(SupplierParam);
