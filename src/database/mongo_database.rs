use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    Client, Collection,
};
use futures::stream::TryStreamExt;

use crate::models::{
    device::Device,
};

pub struct MongoRepo {
    device_col: Collection<Device>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("event-server");
        let device_col = db.collection("devices");
        MongoRepo {
            device_col: device_col,
        }
    }

    pub async fn create_device(&self, new_device: Device) -> Result<InsertOneResult, Error> {
        let new_doc = Device {
            id: None,
            name: new_device.name,
            buttons: new_device.buttons,
            device_id: new_device.device_id,
        };
        let device = self
            .device_col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error Creating Device");
        Ok(device)
    }

    pub async fn get_device(&self, id: &String) -> Result<Device, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let device_detail = self
            .device_col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting device from database");
        Ok(device_detail.unwrap())
    }

    pub async fn update_device(&self, id: &String, new_device: Device) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set": {
                "id": new_device.id,
                "name": new_device.name,
                "buttons": new_device.buttons,
                "device_id": new_device.device_id,
            }
        };
        let updated_doc = self
            .device_col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating device info");
        Ok(updated_doc)
    }

    pub async fn delete_device(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let device_detail = self
            .device_col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error Deleting Device");
        Ok(device_detail)
    }

    pub async fn get_all_devices(&self) -> Result<Vec<Device>, Error> {
        let mut cursors = self
            .device_col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of devices");
        let mut devices: Vec<Device> = Vec::new();
        while let Some(device) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through devices")
        {
            devices.push(device)
        }
        Ok(devices)
    }

    // pub fn create_automation(&self, new_automation: Automation) -> Result<InsertOneResult, Error> {
    //     let new_doc = Automation {

    //     };
    //     let automation = self 
    //         .automation_col
    //         .insert_one(new_doc, None)
    //         .await
    //         .ok()
    //         .expect("Error Creating Automation");
    //     Ok(automation)
    // }

    // pub fn get_automation(&self, id: &String) -> Result<Automation, Error> {
    //     let obj_id = ObjectId::parse_str(id).unwrap();
    //     let filter = doc! {"_id": obj_id};
    //     let automation_detail = self
    //         .automation_col
    //         .find_one(filter, None)
    //         .ok()
    //         .expect("Error getting automation from database");
    //     Ok(automation_detail.unwrap())
    // }

    // pub fn update_automation(&self, id: &String, new_automation: Automation) -> Result<UpdateResult, Error> {
    //     let obj_id = ObjectId::parse_str(id).unwrap();
    //     let filter = doc! {"_id": obj_id};
    //     let new_doc = doc! {
    //         "$set": {},
    //     };
    //     let updated_doc = self
    //         .automation_col
    //         .update_one(filter, new_doc, None)
    //         .ok()
    //         .expect("Error updating automation");
    //     Ok(updated_doc)
    // }

    // pub fn delete_automation(&self, id: &String) -> Result<DeleteResult, Error> {

    // }
}