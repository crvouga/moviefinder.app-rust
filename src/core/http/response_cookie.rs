use std::fmt;

#[derive(Debug, Default, Clone)]
pub struct ResponseCookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub expires: Option<String>,
    pub max_age: Option<u64>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<SameSite>,
}

#[derive(Debug, Clone)]
pub enum SameSite {
    Lax,
    Strict,
    None,
}

impl fmt::Display for ResponseCookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.name, self.value)?;

        if let Some(ref domain) = self.domain {
            write!(f, "; Domain={}", domain)?;
        }
        if let Some(ref path) = self.path {
            write!(f, "; Path={}", path)?;
        }
        if let Some(ref expires) = self.expires {
            write!(f, "; Expires={}", expires)?;
        }
        if let Some(max_age) = self.max_age {
            write!(f, "; Max-Age={}", max_age)?;
        }
        if self.secure {
            write!(f, "; Secure")?;
        }
        if self.http_only {
            write!(f, "; HttpOnly")?;
        }
        if let Some(ref same_site) = self.same_site {
            write!(f, "; SameSite={}", same_site)?;
        }

        Ok(())
    }
}

impl fmt::Display for SameSite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            SameSite::Lax => "Lax",
            SameSite::Strict => "Strict",
            SameSite::None => "None",
        };
        write!(f, "{}", value)
    }
}
