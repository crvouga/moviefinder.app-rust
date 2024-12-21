use super::InteractionName;

impl InteractionName {
    pub fn to_postgres_enum(&self) -> String {
        match self {
            Self::Liked => "liked".to_string(),
            Self::Disliked => "disliked".to_string(),
            Self::Interested => "interested".to_string(),
            Self::NotInterested => "not-interested".to_string(),
            Self::Seen => "seen".to_string(),
            Self::NotSeen => "not-seen".to_string(),
        }
    }
}
