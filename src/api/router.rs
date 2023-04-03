use actix_web::web;

use crate::api::default_routes::{
    health
};

use crate::api::device_routes::{
    get_device,
    get_device_by_id,
    create_device,
    update_device,
    delete_device
};

// default router
pub fn init_default_router(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
}

// device router
pub fn init_device_router(cfg: &mut web::ServiceConfig) {
    cfg.service(get_device);
    cfg.service(get_device_by_id);
    cfg.service(create_device);
    cfg.service(update_device);
    cfg.service(delete_device);
}

// automation router


// event router