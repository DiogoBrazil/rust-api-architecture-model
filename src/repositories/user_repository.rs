use async_trait::async_trait;
use log::info;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use crate::core::contracts::repository::users::UserRepository;
use crate::core::entities::user::{
    CreateUser, UpdatePasswordUser, UpdateUser, UserCompleteData, UserDataCreated
};
use crate::config::querys::user::UserQueries;

#[derive(Clone)]
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create_user(&self, user: CreateUser) -> Result<UserDataCreated, sqlx::Error> {
        let id = Uuid::new_v4();
        let date = chrono::Utc::now().naive_utc();

        info!("[Repository] Executing SQL query to create user with email: {} and ID: {}", user.email, id);
        let user_created: UserDataCreated = sqlx::query_as(UserQueries::CREATE_USER)
            .bind(id)
            .bind(user.full_name)
            .bind(user.email)
            .bind(user.password)
            .bind(date)
            .bind(date)
            .fetch_one(&self.pool)
            .await?;

        info!("[Repository] User successfully inserted into database with ID: {}", user_created.id);
        Ok(user_created)
    }

    async fn find_user_by_id(&self, id: Uuid) -> Result<UserDataCreated, sqlx::Error> {
        let user: UserDataCreated = sqlx::query_as(UserQueries::FIND_BY_ID)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn find_user_by_email(&self, email: String) -> Result<UserCompleteData, sqlx::Error> {
        info!("[Repository] Executing SQL query to find user by email: {}", email);
        let user: UserCompleteData = sqlx::query_as(UserQueries::FIND_BY_EMAIL)
            .bind(email)
            .fetch_one(&self.pool)
            .await?;

        info!("[Repository] User found with ID: {}", user.id);
        Ok(user)
    }

    async fn find_all_users(&self) -> Result<Vec<UserDataCreated>, sqlx::Error> {
        let users: Vec<UserDataCreated> = sqlx::query_as(UserQueries::FIND_ALL)
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    async fn update_user(&self, id: Uuid, data: UpdateUser) -> Result<UserDataCreated, sqlx::Error> {
        let updated_at = chrono::Utc::now().naive_utc();

        let user: UserDataCreated = sqlx::query_as(UserQueries::UPDATE_USER)
            .bind(id)
            .bind(data.full_name)
            .bind(data.email)
            .bind(updated_at)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn update_password_user(&self, id: Uuid, data: UpdatePasswordUser) -> Result<UserDataCreated, sqlx::Error> {
        let updated_at = chrono::Utc::now().naive_utc();

        let user: UserDataCreated = sqlx::query_as(UserQueries::UPDATE_PASSWORD)
            .bind(id)
            .bind(data.new_password)
            .bind(updated_at)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn delete_user(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(UserQueries::DELETE_USER)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn email_exists_for_other_user(&self, email: &str, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(UserQueries::EMAIL_EXIST_FOR_OTHER_USER)
            .bind(email)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        let exists: bool = result.get("exists");
        Ok(exists)
    }
}
