use crate::errors::AppError;
use crate::routes::convert;
use crate::{moduls, Pool};
use actix_web::{web. HttpResponse};
use futures::Future;

#[derive(Debug, Serialize, Deserialize)]
struct UserInput {
    username: String, 
}







fn crate_user(
    item: web::Json<UserInput>,
    pool: web::Data<Pool>,

) -> impl Future<Item = HttpResponse, Error = AppError>{
    web::block(move ||{
        let conn = &poll.get().unwrap();
        let username = item.into_inner().username;
        models::crate_user(conn, username.as_str())
    })
    .then(convert)

}
