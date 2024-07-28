use std::str::FromStr;
use sea_orm::{DeriveValueType, Order};

pub trait PageRequest {
    fn offset(&self) -> Option<u32>;

    fn limit(&self) -> Option<u32>;
}

#[derive(Debug, Clone)]
pub struct Page {
    pub page_size: u32,
    pub page_no: u32,
    pub sorts: Vec<Sort>,
}

#[derive(Debug, Clone)]
pub struct Sort {
    pub field: String,
    pub direction: SortDirection,
}


#[derive(Clone, Debug)]
pub enum SortDirection {
    ASC,
    DESC,
}

impl FromStr for SortDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "asc" => Ok(SortDirection::ASC),
            "desc" => Ok(SortDirection::DESC),
            _ => Err("错误的排序类型".into())
        }
    }
}

impl Into<Order> for SortDirection {
    fn into(self) -> Order {
        match self {
            SortDirection::ASC => Order::Asc,
            SortDirection::DESC => Order::Desc
        }
    }
}

impl PageRequest for Page {
    fn offset(&self) -> Option<u32> {
        let mut page_no = self.page_no;
        if page_no > 0 {
            page_no -= 1;
        }
        Some(page_no * self.page_size)
    }

    fn limit(&self) -> Option<u32> {
        Some(self.page_size)
    }
}

#[macro_export]
macro_rules! impl_page_request {
    ($name:ident) => {
        impl PageRequest for $name {
            fn offset(&self) -> Option<u32> {
                if let Some(page) = self.page.clone() {
                    return page.offset();
                }
                None
            }

            fn limit(&self) -> Option<u32> {
                if let Some(page) = self.page.clone() {
                    return page.limit();
                }
                None
            }
        }
    };
}
