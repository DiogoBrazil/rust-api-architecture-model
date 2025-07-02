pub struct UserQueries;

impl UserQueries {
    pub const CREATE_USER: &'static str = r#"
        INSERT INTO users (id, full_name, email, password, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, full_name, email, created_at, updated_at
    "#;

    pub const FIND_BY_ID: &'static str = r#"
        SELECT id, full_name, email, created_at, updated_at
        FROM users
        WHERE id = $1
    "#;

    pub const FIND_BY_EMAIL: &'static str = r#"
        SELECT id, full_name, email, password, created_at, updated_at
        FROM users
        WHERE email = $1
    "#;

    pub const FIND_ALL: &'static str = r#"
        SELECT id, full_name, email, created_at, updated_at
        FROM users
        ORDER BY created_at DESC
    "#;

    pub const UPDATE_USER: &'static str = r#"
        UPDATE users
        SET full_name = $2, email = $3, updated_at = $4
        WHERE id = $1
        RETURNING id, full_name, email, created_at, updated_at
    "#;

    pub const UPDATE_PASSWORD: &'static str = r#"
            UPDATE users
            SET password = $2, updated_at = $3
            WHERE id = $1
            RETURNING id, full_name, email, created_at, updated_at
        "#;

    pub const DELETE_USER: &'static str = r#"
        DELETE FROM users WHERE id = $1
    "#;

    pub const EMAIL_EXIST_FOR_OTHER_USER: &'static str = r#"
        SELECT EXISTS(SELECT 1 FROM users WHERE email = $1 AND id != $2) as exists
    "#;
}
