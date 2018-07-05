pub mod sqlite;

use controller;

pub trait Persist {
    fn store_node(&self, &controller::Node);
    // how should we handle nested objects??
    //fn store_sensor(&self, &controller::Sensor);
    //fn store_reading(&self, &controller::Reading);
}
