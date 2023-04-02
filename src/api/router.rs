use actix_web::{
    web,
    get,
    post,
    put,
    delete,
    Responder,
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;

use crate::{
    models::device::Device,
    database::mongo_database::MongoRepo
};


#[get("/")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[get("/device")]
pub async fn get_device(db: web::Data<MongoRepo>) -> HttpResponse {
    let devices = db.get_all_devices().await;
    match devices {
        Ok(devices) => HttpResponse::Ok().json(devices),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[post("/device")]
pub async fn create_device(db: web::Data<MongoRepo>, new_device: web::Json<Device>) -> HttpResponse {
    let device_data = Device {
        id: None,
        name: new_device.name.to_string(),
        buttons: new_device.buttons.to_vec(),
        device_id: new_device.device_id,
    };
    let device_detail = db.create_device(device_data).await;
    match device_detail {
        Ok(device_data) => HttpResponse::Ok().json(device_data),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/device/{id}")]
pub async fn get_device_by_id(db: web::Data<MongoRepo>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let device_detail = db.get_device(&id).await;
    match device_detail {
        Ok(device) => HttpResponse::Ok().json(device),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[put("/device/{id}")]
pub async fn update_device(db: web::Data<MongoRepo>, path: web::Path<String>, new_device: web::Json<Device>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let device_data = Device {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_device.name.to_string(),
        buttons: new_device.buttons.to_vec(),
        device_id: new_device.device_id,
    };
    let update_result = db.update_device(&id, device_data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_device_info = db.get_device(&id).await;
                return match updated_device_info {
                    Ok(device) => HttpResponse::Ok().json(device),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No device found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/device/{id}")]
pub async fn delete_device(db: web::Data<MongoRepo>, path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let result = db.delete_device(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("success");
            } else {
                return HttpResponse::NotFound().json("Device with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}