use persistance;
use rusqlite;
use std::path::Path;

const CREATE_TABLES: &'static str = include_str!("create_tables.sql");

pub struct SqlitePersist {
    conn: rusqlite::Connection,
}

impl SqlitePersist {
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<SqlitePersist, rusqlite::Error> {
        let conn = rusqlite::Connection::open(filename)?;
        let per = SqlitePersist { conn };

        per.initialize_db();

        Ok(per)
    }

    fn initialize_db(&self) {
        let res = self.conn.execute_batch(CREATE_TABLES);
        println!("{:?}", res);
    }
}

impl persistance::Persist for SqlitePersist {
    //TODO: add error handling

    fn store_node(&self, node: &persistance::Node) {
        self.conn.execute(
            "REPLACE INTO nodes (id, name, version) VALUES (?1, ?2, ?3)",
            &[&node.id, &node.name, &node.version],
        );
    }

    fn store_sensor(&self, sensor: &persistance::Sensor) {
        self.conn.execute(
            "REPLACE INTO sensors (id, node_id, type, description) VALUES (?1, ?2, ?3, ?4)",
            &[
                &sensor.id,
                &sensor.node_id,
                &sensor.sensor_type,
                &sensor.description,
            ],
        );
    }

    fn store_reading(&self, reading: &persistance::Reading) {
        self.conn.execute("INSERT INTO readings (node_id, sensor_id, timestamp, value, kind) VALUES (?1, ?2, ?3, ?4, ?5)", 
        &[&reading.node_id, &reading.sensor_id, &reading.timestamp, &reading.value, &reading.kind]);
    }
}
