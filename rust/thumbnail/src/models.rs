use serde::Deserialize;

pub struct ThumbnailGeneratorEnvironment {
    pub s3_bucket_name: String,
    pub s3_directory: String,
    pub s3_client: Option<aws_sdk_s3::Client>,
}

// todo: figure out how to distribute this across packages without copy paste
#[derive(Debug, Deserialize)]
pub struct ThumbnailLambdaInput {
    pub combat_pages: [String; 9],
    pub thumb_name: String,
}
