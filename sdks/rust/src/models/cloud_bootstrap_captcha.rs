/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudBootstrapCaptcha {
    #[serde(rename = "turnstile", skip_serializing_if = "Option::is_none")]
    pub turnstile: Option<Box<crate::models::CloudBootstrapCaptchaTurnstile>>,
}

impl CloudBootstrapCaptcha {
    pub fn new() -> CloudBootstrapCaptcha {
        CloudBootstrapCaptcha {
            turnstile: None,
        }
    }
}


