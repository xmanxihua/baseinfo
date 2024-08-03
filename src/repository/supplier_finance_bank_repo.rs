use crate::bean::supplier_finance_bank_entity::SupplierFinanceBankEntity;
use crate::dao::prelude::SupplierFinanceBank;
use crate::dao::supplier_finance_bank;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use crate::dao::supplier_finance_bank::{ActiveModel, Model};

pub struct SupplierFinanceBankRepo<'a> {
    pub db: &'a DatabaseConnection,
}

impl <'a>SupplierFinanceBankRepo<'a> {
    pub async fn save_or_update(&self, bank: &mut SupplierFinanceBankEntity) -> Result<i32, DbErr> {
        let model = SupplierFinanceBank::find()
            .filter(
                supplier_finance_bank::Column::SupplierCode.eq(bank.supplier_code.clone().unwrap()),
            )
            .one(self.db)
            .await?;
        if let Some(model) = model {
            SupplierFinanceBank::update::<ActiveModel>(bank.clone().into())
                .filter(supplier_finance_bank::Column::Id.eq(model.id))
                .exec(self.db)
                .await
                .map(|x| 1)
        } else {
            SupplierFinanceBank::insert::<ActiveModel>(bank.clone().into())
                .exec(self.db)
                .await
                .map(|x| {
                    bank.id = Some(x.last_insert_id);
                    1
                })
        }
    }
}
