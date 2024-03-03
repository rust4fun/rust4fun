use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct UserId(Uuid);

impl From<Uuid> for UserId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl UserId {
    pub fn id(self) -> Uuid {
        self.0
    }

    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct ArticleId(Uuid);

impl From<Uuid> for ArticleId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl ArticleId {
    pub fn id(self) -> Uuid {
        self.0
    }

    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct SphereId(Uuid);

impl From<Uuid> for SphereId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl SphereId {
    pub fn id(self) -> Uuid {
        self.0
    }

    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct PlanetId(Uuid);

impl From<Uuid> for PlanetId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl PlanetId {
    pub fn id(self) -> Uuid {
        self.0
    }

    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct PlanetMessageId(Uuid);

impl From<Uuid> for PlanetMessageId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl PlanetMessageId {
    pub fn id(self) -> Uuid {
        self.0
    }

    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}
