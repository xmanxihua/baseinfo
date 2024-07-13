mod bean;
mod entity;
mod controller;
mod service;

use axum::{
    routing::get,
    Router,
};
use axum::routing::post;
use sea_orm::{Database, PaginatorTrait, QueryFilter, Statement};
use crate::entity::prelude::{Supplier as SupplierDao, Supplier};
use crate::entity::supplier;
use sea_orm::EntityTrait;
use sea_orm::ColumnTrait;
use tracing_subscriber::EnvFilter;
use crate::controller::supplier_controller::SupplierController;

#[tokio::main]
async fn main() {

    // 初始化 tracing-subscriber
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    // 设置全局日志级别为 info
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
        //单独设置sea_orm
        .add_directive("sea_orm::driver=debug".parse().unwrap())
        //关闭sqlx自带的日志
        .add_directive("sqlx::query=off".parse().unwrap());
    let db = Database::connect("postgres://postgres:micun_db@0415@10.0.2.251:5432/baseinfo").await.unwrap();

    let supplier = SupplierDao::find_by_id(67).one(&db).await.unwrap();
    match supplier {
        None => {}
        Some(supplier) => println!("查询成功！：{:?}", supplier)
    }

    let count = SupplierDao::find().count(&db).await.unwrap();
    println!("数量：{}", count);

    let one = SupplierDao::find().filter(supplier::Column::SupplierCode.eq("10000146")).one(&db).await.unwrap();
    if let Some(supplier) = one {
        println!("查询成功！：{:?}", supplier);
    }


    let sql = "select * from supplier where supplier_name like concat('%',$1,'%')";
    let statement = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        sql, vec!["测试".into()]);
    let page = SupplierDao::find().from_raw_sql(statement)
        .paginate(&db,50);
    let data = page.fetch_page(0).await.unwrap();
        println!("分页查询1: {:?}", data);

    // let list = SupplierDao::find().filter(supplier::Column::SupplierName.like("%测试%"))
    //     .paginate(&db,50).fetch_page(0).await.unwrap();

    // println!("分页查询: {:?}", list);
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/baseinfo/supplier/submit",post(SupplierController::submit))
        ;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}