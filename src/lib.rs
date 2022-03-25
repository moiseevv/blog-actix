#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};






type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct Blog {
    port: u16,
}

impl Blog {
    pub fn new(port: u16)-> Self{
        Blog{port}
    }
    pub fn run(&self, database_url: String)-> std::io::Result<()>{
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);

        let pool = r2d2::Pool::builder()
            .builder(manager)
            .expect("Failer to create pool.");
            
        println!(" starting hppt server : 127.0.0.1:{}", sefl.port);
        HttpServer::new(move || {
            App::new()
                .data(pool.clone())
                .wrap(middleware::Loger::default())
                .congigure(rootes::users::configure)
        })
        .bind("127.0.0.1", self.port)?
        .run()

    }
