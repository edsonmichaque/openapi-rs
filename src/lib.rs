use std::collections::HashMap;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAPI {
    pub openapi: String,
    pub info: Option<String>,
    pub paths: HashMap<String, Path>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub title: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub term_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    pub version: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub url: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub url: Option<String>,
    pub name: String,
    pub identifier: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub reference: String,
    pub summary: String,
    pub description: String,
    pub server: Option<vec<Server>>,
    pub get: Option<Operation>,
    pub post: Option<Operation>,
    pub delete: Option<Operation>,
    pub patch: Option<Operation>,
    pub trace: Option<Operation>,
    pub head: Option<Operation>,
    pub put: Option<Operation>,
    pub options: Option<Operation>,
}

#[serde(untagged)]
pub enum Operation {
    Value(OperationValue),
    Ref(Reference),
}

#[serde(untagged)]
pub enum Response {
    Value(ResponseValue),
    Ref(Reference),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseValue {
    pub required: bool,
    pub description: String,
    pub content: HashMap<String, MediaType>,
    pub links: HashMap<String, Link>,
    pub headers: HashMap<String, Header>,
}

#[serde(untagged)]
pub enum Link {
    Value(LinkValue),
    Ref(Reference),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkValue {}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationValue {
    pub summary: String,
    pub description: String,
    pub tags: Option<vec<String>>,
    pub operation_id: Option<String>,
    pub parameters: Option<Parameter>,
    pub request_body: Option<RequestBody>,
}

#[serde(untagged)]
pub enum Parameter {
    Value(ParameterValue),
    Ref(Reference),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterValue {}

#[serde(untagged)]
pub enum RequestBody {
    Value(RequestBodyValue),
    Ref(Reference),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBodyValue {
    pub required: bool,
    pub description: String,
    pub content: HashMap<String, MediaType>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaType {
    pub schema: Option<Schema>,
    pub example: String,
    pub examples: HashMap<String, Example>,
    pub encoding: HashMap<String, Encoding>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Encoding {
    pub contentType: String,
    pub headers: Option<Header>,
    pub stype: String,
    pub explode: bool,
    pub allow_reserved: bool,
}

#[serde(untagged)]
pub enum Header {
    Value(HeaderValue),
    Ref(Reference),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderValue {}

#[serde(untagged)]
pub enum Example {
    Value(ExampleValue),
    Ref(Reference),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleValue {
    pub summary: String,
    pub description: String,
    pub value: String,
    pub external_value: String,
}

#[derive(Deserialize, Serialize)]
pub enum Schema {
    Value(SchemaValue),
    Ref(Reference),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaValue {
    pub r#type: String,
    pub properties: HashMap<String, Box<Schema>>,
    pub required: Option<vec<String>>,
    pub format: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub r#ref: String,
    pub summary: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub schemas: HashMap<String, Schema>,
    pub responses: HashMap<String, Response>,
    pub parameters: HashMap<String, Parameter>,
    pub examples: HashMap<String, Example>,
    pub request_bodies: HashMap<String, RequestBody>,
    pub headers: HashMap<String, Header>,
    pub security_schemas: HashMap<String, Schema>,
    pub links: HashMap<String, Link>,
    pub callbacks: HashMap<String, Schema>,
    pub path_items: HashMap<String, Schema>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub url: String,
    pub description: Option<String>,
    pub variables: HashMap<String, ServerVariable>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerVariable {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
