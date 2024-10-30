use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        // Use the `parse_str` function from the `uuid` crate to ensure `id` is a valid UUID
        if Uuid::parse_str(&id).is_ok() {
            Ok(Self(id))
        }else {
            Err(format!("{} is not a valid UUID.", id))
        }
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        // Use the `uuid` crate to generate a random version 4 UUID
        Self(Uuid::new_v4().to_string())
    }
}

// Implement AsRef<str> for LoginAttemptId
impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::LoginAttemptId;

    #[test]
    fn non_valid_uuid_is_rejected() {
        let login_attemtp_id: String = "cdcd".to_string();
        assert!(LoginAttemptId::parse(login_attemtp_id).is_err());
    }

    #[test]
    fn valid_uuid_is_accepted() {
        let login_attemtp_id: String = "db1f8d2b-4a23-42b3-9d6c-41dc26fd5f65".to_string();
        assert!(LoginAttemptId::parse(login_attemtp_id).is_ok());
    }

    #[test]
    fn autogenereted_uuid_is_accepted() {
        let login_attemtp_id: String = LoginAttemptId::default().as_ref().to_string();
        assert!(LoginAttemptId::parse(login_attemtp_id).is_ok());
    }

}