use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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
