pub trait PageRequest {
    fn offset(&self) -> Option<u32>;

    fn limit(&self) -> Option<u32>;
}

#[derive(Debug)]
pub struct Page {
    pub page_size: u32,
    pub page_no: u32,
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
                if let Some(ref page) = self.page {
                    return page.offset();
                }
                None
            }

            fn limit(&self) -> Option<u32> {
                if let Some(ref page) = self.page {
                    return page.limit();
                }
                None
            }
        }
    };
}
