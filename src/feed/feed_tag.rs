use serde::{Deserialize, Serialize};

use crate::{
    core::{
        base::base32,
        query::{QueryFilter, QueryOp},
        ui::chip::{Chip, ChipSize},
    },
    media::person::person_::Person,
    media::{genre::genre::Genre, media_db::interface::MediaQueryField},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum FeedTag {
    Genre(Genre),
    Person(Person),
}

impl PartialOrd for FeedTag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.label().partial_cmp(&other.label())
    }
}

impl Ord for FeedTag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.label().cmp(&other.label())
    }
}

impl From<FeedTag> for QueryFilter<MediaQueryField> {
    fn from(feed_tag: FeedTag) -> QueryFilter<MediaQueryField> {
        match feed_tag {
            FeedTag::Genre(genre) => {
                QueryFilter::Clause(MediaQueryField::GenreId, QueryOp::Eq, genre.id.to_string())
            }
            FeedTag::Person(person) => QueryFilter::Clause(
                MediaQueryField::PersonId,
                QueryOp::Eq,
                person.id.to_string(),
            ),
        }
    }
}

impl FeedTag {
    pub fn chip(&self) -> Chip {
        let base_chip = Chip::default()
            .id(&self.encode())
            .label(&self.label())
            .size(ChipSize::Medium);
        match self {
            FeedTag::Genre(_genre) => base_chip,
            FeedTag::Person(person) => base_chip.image(&person.profile.to_middle_res()),
        }
    }

    pub fn label(&self) -> String {
        match self {
            FeedTag::Genre(genre) => genre.name.clone(),
            FeedTag::Person(person) => person.name.clone(),
        }
    }

    pub fn encode(&self) -> String {
        let json = serde_json::to_string(&self).unwrap_or("".to_owned());
        base32::encode(base32::Alphabet::Crockford, &json.as_bytes())
    }

    pub fn decode(encoded: &str) -> Option<Self> {
        let json = String::from_utf8(base32::decode(base32::Alphabet::Crockford, encoded)?).ok()?;
        serde_json::from_str(&json).ok()
    }
}

#[cfg(test)]
mod tests {
    use crate::media::genre::genre_id::GenreId;

    use super::*;

    #[test]
    fn test_encode_decode() {
        let route = FeedTag::Genre(Genre {
            id: GenreId::new("id".to_string()),
            name: "name".to_string(),
        });
        let encoded = route.encode();
        let decoded = FeedTag::decode(&encoded).unwrap();
        assert_eq!(decoded, route);
    }
}
