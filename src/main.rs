use axum::{
    Router,
    routing::get,
};
use axum::routing::post;
use sea_orm::{Database, PaginatorTrait, QueryFilter};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;

use crate::bean::app_state_dyn::AppStateDyn;
use crate::controller::supplier_controller::SupplierController;
use crate::repository::supplier_repo::SupplierRepo;

mod bean;
mod dao;
mod controller;
mod service;
mod utils;
mod repository;

#[tokio::main]
async fn main() {

    // 初始化 tracing-subscriber
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    // 设置全局日志级别为 info
    // let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    //     //单独设置sea_orm
    //     .add_directive("sea_orm::driver=debug".parse().unwrap())
    //     //关闭sqlx自带的日志
    //     .add_directive("sqlx::query=off".parse().unwrap());
    let db = Database::connect("postgres://postgres:micun_db@0415@10.0.2.251:5432/baseinfo").await.unwrap();

    let supplier_repo = SupplierRepo {
        db: db.clone()
    };
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/baseinfo/supplier/submit", post(SupplierController::submit))
        .with_state(AppStateDyn {
            db: db.clone(),
            supplier_repo: supplier_repo.clone(),
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}