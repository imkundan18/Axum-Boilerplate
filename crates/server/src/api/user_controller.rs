
use axum::{
    routing::{get, post, put, delete}, Extension, Json, Router
};
use axum::extract::Path;
use database::user::model::User;
use mongodb::results::{InsertOneResult,UpdateResult,DeleteResult};
use utils::AppResult;

use crate::{
    dtos::user_dto::SignUpUserDto, extractors::validation_extractor::ValidationExtractor,
    services::Services,
};

pub struct UserController;
impl UserController {
    pub fn app() -> Router {
        Router::new()
            .route("/", get(Self::all))
            .route("/create", post(Self::create))
            //.route("/update/:id", put(Self::update_data))
            //.route("/delete/:id", delete(Self::delete_data))
            .route("/signup", post(Self::signup))        
    }
    
    
    pub async fn all(Extension(services): Extension<Services>) -> AppResult<Json<Vec<User>>> {
        let users = services.user.get_all_users().await?;
        Ok(Json(users))
    }

    pub async fn signup(
        Extension(services): Extension<Services>,
        ValidationExtractor(req): ValidationExtractor<SignUpUserDto>,
    ) -> AppResult<Json<InsertOneResult>> {
        let created_user = services.user.signup_user(req).await?;

        Ok(Json(created_user))
    }

    pub async fn create(Extension(services):Extension<Services>,Json(req):Json<SignUpUserDto>) -> AppResult<Json<InsertOneResult>>{
        let request=req;
        eprint!("request: {:?}", request);
        let created_user = services.user.create_users(request).await?;
        Ok(Json(created_user))
    }
    pub async fn update_data(Extension(services):Extension<Services>,Json(req):Json<SignUpUserDto>, Path(id):Path<String>) -> AppResult<Json<UpdateResult>>{
        let request=req;
        let update_user = services.user.update_users(request,id).await?;
        Ok(Json(update_user))

    }pub async fn delete_data(Extension(services):Extension<Services>,Path(id):Path<String>) -> AppResult<DeleteResult>{
        let delete_user = services.user.delete_users(id).await?;
        Ok(delete_user)
    }



}
