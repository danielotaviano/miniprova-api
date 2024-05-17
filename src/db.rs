use diesel::{r2d2::ConnectionManager, PgConnection};
use lazy_static::lazy_static;
use r2d2::{Pool, PooledConnection};
use std::ops::DerefMut;
use std::sync::Mutex;

pub struct DatabaseManager {
    pool: Mutex<Option<Pool<ConnectionManager<PgConnection>>>>,
}

impl DatabaseManager {
    pub async fn start_connection(&mut self) -> () {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        *self.pool.lock().unwrap().deref_mut() = Some(pool);

        println!("Database Connected");
    }

    pub fn get_database(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.lock().unwrap().as_ref().unwrap().get().unwrap()
    }
}

lazy_static! {
    pub static ref DB_MANAGER: Mutex<DatabaseManager> = Mutex::new(DatabaseManager {
        pool: Mutex::new(None)
    });
}
