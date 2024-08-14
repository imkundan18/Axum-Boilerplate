use std::sync::Arc;

use anyhow::Ok;
use async_trait::async_trait;
use database::user::{model::User, repository::DynUserRepository};
use mongodb::results::InsertOneResult;
use tracing::{error, info};
//use utils::{AppError, AppResult};

use crate::dtos::user_dto::SignUpUserDto;

#[allow(clippy::module_name_repetitions)]
pub type DynUserService = Arc<dyn UserServiceTrait + Send + Sync>;

#[async_trait]
#[allow(clippy::module_name_repetitions)]
pub trait UserServiceTrait {
    // async fn get_current_user(&self, user_id: &str) -> AppResult<Option<User>>;

    async fn get_all_users(&self) -> Result<Vec<User>, anyhow::Error>/*AppResult<Vec<User>>*/;

    async fn signup_user(&self, request: SignUpUserDto) -> Result<InsertOneResult, anyhow::Error>/*AppResult<InsertOneResult>*/;

    async fn create_users(&self, request: SignUpUserDto) ->Result<InsertOneResult, anyhow::Error>/*AppResult<InsertOneResult>*/;
}

#[derive(Clone)]
pub struct UserService {
    repository: DynUserRepository,
}

impl UserService {
    pub fn new(repository: DynUserRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn signup_user(&self, request: SignUpUserDto) -> Result<InsertOneResult, anyhow::Error>/*AppResult<InsertOneResult>*/ {
        let email = request.email.unwrap();
        let name = request.name.unwrap();
        let password = request.password.unwrap();

        let existing_user = self.repository.get_user_by_email(&email).await?;

        if existing_user.is_some() {
            error!("user {:?} already exists", email);
            //return Err(AppError::Conflict(format!("email {email} is taken")));
        }

        let new_user = self
            .repository
            .create_user(&name, &email, &password)
            .await?;

        info!("created user {:?}", new_user);

        Ok(new_user)
    }

    // async fn get_current_user(&self, user_id: &str) -> AppResult<Option<User>> {
    //     let user = self.repository.get_user_by_id(user_id).await?;

    //     Ok(user)
    // }

    async fn get_all_users(&self) -> /*AppResult<Vec<User>>*/Result<Vec<User>, anyhow::Error> {
        let users = self.repository.get_all_users().await?;

        Ok(users)
    }

    async fn create_users(&self, req: SignUpUserDto) -> Result<InsertOneResult, anyhow::Error>/* ;AppResult<InsertOneResult> */{
        let name = req.name.unwrap();
        let email = req.email.unwrap();
        let password = req.password.unwrap();

        // let name = req.name.as_ref().unwrap().as_str();
        // let email = req.email.as_ref().unwrap().as_str();
        // let password = req.password.as_ref().unwrap().as_str();

        let newuser=self.repository.create_user(&name, &email, &password).await?;
        Ok(newuser)
    }
}
