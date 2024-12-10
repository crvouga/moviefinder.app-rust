#[cfg(test)]
mod tests {
    use crate::core::phone_number::country_code::country_code_db::{
        impl_json_file, interface::PhoneNumberCountryCodeDb,
    };

    struct Fixture {
        country_code_db: Box<dyn PhoneNumberCountryCodeDb>,
    }

    async fn fixtures() -> Vec<Fixture> {
        let mut fixtures: Vec<Fixture> = vec![];

        fixtures.push(Fixture {
            country_code_db: Box::new(impl_json_file::ImplJsonFile::new()),
        });

        fixtures
    }

    #[tokio::test]
    async fn test_get_all() {
        for f in fixtures().await {
            let country_codes = f.country_code_db.get_all().await;
            assert!(country_codes.len() > 0);
        }
    }
}
