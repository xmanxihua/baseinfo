use crate::bean::supplier_account_entity::SupplierAccountEntity;
use crate::bean::supplier_account_param::SupplierAccountParam;
use crate::bean::supplier_param::SupplierParam;
use crate::dao::prelude::SupplierAccount as SupplierAccountDao;
use crate::dao::supplier_account;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
    QueryTrait,
};
use crate::bean::page_request::PageRequest;

#[derive(Debug)]
pub struct SupplierAccountRepo<'a> {
    pub db: &'a DatabaseConnection,
}

impl <'a> SupplierAccountRepo<'a> {
    pub async fn query_list(
        &self,
        supplier_account_param: SupplierAccountParam,
    ) -> Result<Vec<SupplierAccountEntity>, DbErr> {
        let mut select = SupplierAccountDao::find()
            .apply_if(supplier_account_param.data_state, |mut select, x| {
                select.filter(supplier_account::Column::DataState.eq(x))
            })
            .apply_if(supplier_account_param.data_states.as_ref(), |mut select, x| {
                select.filter(supplier_account::Column::DataState.is_in(x.clone()))
            })
            .apply_if(supplier_account_param.offset(), |mut select, x| {
                select.offset(x as u64)
            })
            .apply_if(
                supplier_account_param.supplier_code_not_in.as_ref(),
                |mut select, x| select.filter(supplier_account::Column::SupplierCode.is_not_in(x.clone())),
            )
            .apply_if(supplier_account_param.supplier_codes.clone(), |mut select, x| {
                select.filter(supplier_account::Column::SupplierCode.is_in(x))
            })
            .apply_if(supplier_account_param.r#type, |mut select, x| {
                let e = Expr::cust(format!("type @> '{}'", x));
                select.filter(e)
            })
            .apply_if(supplier_account_param.types.clone(), |mut select, x| {
                let e = Expr::cust(format!("type @> '{:?}'", x));
                select.filter(e)
            })
            .apply_if(supplier_account_param.limit(), |mut select, x| {
                select.limit(x as u64)
            });

        let list = select.all(self.db).await?;

        list.into_iter().map(|x| x.try_into().map_err(|e|DbErr::Custom(e))).collect()
    }
}
