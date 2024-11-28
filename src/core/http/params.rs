use crate::core::params::{Params, ParamsHashMap};

use super::{content_type::ContentType, json_data::JsonData, method::Method, request::Request};

impl Request {
    pub fn params(&self) -> ParamsHashMap {
        match (self.method.clone(), self.content_type()) {
            (Method::Get, _) => self.url.query_params.params.clone(),
            (Method::Post, ContentType::ApplicationJson) => {
                let body_string = String::from_utf8_lossy(&self.body).to_string();
                JsonData::from_string(&body_string).params
            }
            (_, _) => self.form_data.params.clone(),
        }
    }
}
