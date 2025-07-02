use async_trait::async_trait;
use uuid::Uuid;
use crate::core::entities::user::{CreateUser, UpdatePasswordUser, UpdateUser, UserDataCreated, UserCompleteData};

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create_user(&self, data: CreateUser) -> Result<UserDataCreated, sqlx::Error>;
    async fn update_user(&self, id: Uuid, data: UpdateUser) -> Result<UserDataCreated, sqlx::Error>;
    async fn update_password_user(&self, id: Uuid, data: UpdatePasswordUser) -> Result<UserDataCreated, sqlx::Error>;
    async fn find_all_users(&self) -> Result<Vec<UserDataCreated>, sqlx::Error>;
    async fn find_user_by_id(&self, id: Uuid) -> Result<UserDataCreated, sqlx::Error>;
    async fn find_user_by_email(&self, email: String) -> Result<UserCompleteData, sqlx::Error>;
    async fn delete_user(&self, id: Uuid) -> Result<bool, sqlx::Error>;
    async fn email_exists_for_other_user(&self, email: &str, id: Uuid) -> Result<bool, sqlx::Error>;
}
