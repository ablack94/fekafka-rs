
pub struct RequestHeaderV0 {
    pub request_api_key: i16,
    pub request_api_version: i16,
    pub correlation_id: i32,
}

pub struct RequestHeaderV1 {
    pub request_api_key: i16,
    pub request_api_version: i16,
    pub correlation_id: i32,
    pub client_id: String,
}

pub struct RequestHeaderV2 {
    pub request_api_key: i16,
    pub request_api_version: i16,
    pub correlation_id: i32,
    pub client_id: String,
    pub tagged_fields: Vec<u8>,
}

pub struct ResponseHeaderV0 {
    pub correlation_id: i32,
}

pub struct ResponseHeaderV1 {
    pub correlation_id: i32,
    pub tagged_fields: Vec<u8>,
}

