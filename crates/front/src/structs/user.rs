use garde::Validate;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct UserInfo {
    pub name: String,
}

impl UserInfo {
    pub fn new(name: String) -> Self {
        UserInfo { name }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Validate)]
#[garde(transparent)]
pub struct UserName(#[garde(length(min = 1, max = 20))] pub String);

#[derive(Debug, Default, PartialEq, Clone, Validate)]
#[garde(transparent)]
pub struct UserEmail(#[garde(email)] pub String);

#[derive(Debug, Default, PartialEq, Clone, Validate)]
#[garde(transparent)]
pub struct UserPassword(#[garde(length(min = 8), custom(is_strong_password))] pub String);

fn is_strong_password(value: &str, _: &&&()) -> garde::Result {
    if !value.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(garde::Error::new("required lowercase"));
    }

    if !value.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(garde::Error::new("required uppercase"));
    }

    if !value.chars().any(|c| c.is_ascii_digit()) {
        return Err(garde::Error::new("number"));
    }

    Ok(())
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UserLogin {
    pub email: UserEmail,
    pub password: String,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UserSignUp {
    pub name: UserName,
    pub email: UserEmail,
    pub password: UserPassword,
}

pub struct ValidationMap(HashMap<String, (bool, String)>);

impl ValidationMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: String, value: (bool, String)) {
        self.0.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<(bool, String)> {
        self.0.get(key).cloned()
    }

    pub fn is_valid(&self) -> bool {
        self.0.iter().all(|(_, v)| v.0)
    }
}

impl Default for ValidationMap {
    fn default() -> Self {
        Self::new()
    }
}
