use controller;
use persistance;
use rusqlite;

const CREATE_TABLES: &'static str = include_str!("create_tables.sql");

pub struct SqlitePersist {
    conn: rusqlite::Connection,
}

impl SqlitePersist {
    pub fn new() -> Result<SqlitePersist, rusqlite::Error> {
        let conn = rusqlite::Connection::open("mysensors.sqlite")?;
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
    fn store_node(&self, node: &controller::Node) {
        self.conn
            .execute("INSERT INTO nodes (id, name, version) VALUES (?1, ?2, ?3)",
            &[&node.id, &node.name, &node.version]);
    }
}
