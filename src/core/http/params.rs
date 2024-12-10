use crate::core::unstructured_data::{UnstructuredData, UnstructuredDataHashMap};

use super::{content_type::ContentType, json_data::JsonData, method::Method, request::Request};

impl Request {
    pub fn params(&self) -> UnstructuredDataHashMap {
        match self.method.clone() {
            Method::Get => self.url.query_params.params.clone(),
            Method::Post | Method::Patch | Method::Put => match self.content_type() {
                ContentType::ApplicationJson => {
                    let body_string = String::from_utf8_lossy(&self.body).to_string();
                    JsonData::from_string(&body_string).params
                }
                _ => self.form_data.params.clone(),
            },
            _ => self.form_data.params.clone(),
        }
    }
}
