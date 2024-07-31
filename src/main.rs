use std::io::Read;
use std::ops::{Deref, DerefMut};

use axum::{Router, routing::get};
use axum::extract::{FromRequest, FromRequestParts};
use axum::response::IntoResponse;
use axum::routing::post;
use rand::{Rng, RngCore};
use sea_orm::{ColumnTrait, DatabaseConnection};
use sea_orm::{Database, PaginatorTrait, QueryFilter};
use sea_orm::EntityTrait;
use serde::Deserialize;
use tracing_subscriber::EnvFilter;

use crate::bean::app_state_dyn::AppStateDyn;
use crate::controller::supplier_controller::SupplierController;
use crate::repository::supplier_account_repo::SupplierAccountRepo;
use crate::repository::supplier_repo::SupplierRepo;
use crate::service::supplier_service::SupplierService;

mod bean;
mod constants;
mod controller;
mod dao;
mod repository;
mod service;
mod sso;
mod utils;
mod test;

#[tokio::main]
async fn main() {
    // 初始化 tracing-subscriber
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    // 设置全局日志级别为 info
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"))
        //单独设置sea_orm
        .add_directive("sea_orm::driver=debug".parse().unwrap())
        //关闭sqlx自带的日志
        .add_directive("sqlx::query=off".parse().unwrap());

    let db: &'static DatabaseConnection = get_db().await;
    let mut db_holder =
    unsafe {
        Box::from_raw(db as *const DatabaseConnection as *mut DatabaseConnection)
    };

    let supplier_account_repo: &'static SupplierAccountRepo =
        Box::leak(Box::new(SupplierAccountRepo { db }));
    let mut supplier_account_repo_holder =
    unsafe { Box::from_raw(
            supplier_account_repo as *const SupplierAccountRepo as *mut SupplierAccountRepo
        )
    };

    let supplier_repo: &'static SupplierRepo = Box::leak(Box::new(SupplierRepo {
        db,
        supplier_account_repo,
    }));
    let mut supplier_repo_holder = unsafe {
        Box::from_raw(supplier_repo as *const SupplierRepo as *mut SupplierRepo)
    };

    let supplier_service: &'static SupplierService = Box::leak(Box::new(SupplierService {
        supplier_repo: &supplier_repo,
        db,
    }));
    let mut supplier_service_holder =
    unsafe {
            Box::from_raw(supplier_service as *const SupplierService as *mut SupplierService)
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/baseinfo/supplier/submit",
            post(SupplierController::submit),
        )
        .with_state(AppStateDyn {
            db,
            supplier_repo: &supplier_repo,
            supplier_service: &supplier_service,
        })
        .layer(axum::middleware::from_fn(sso::auth::auth_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn get_db() -> &'static mut DatabaseConnection {
    let db = Box::new(
        Database::connect("postgres://postgres:micun_db@0415@10.0.2.251:5432/baseinfo")
            .await
            .unwrap(),
    );
    Box::leak(db)
}

#[derive(Debug, Deserialize)]
struct Params {
    satoken: String,
}




