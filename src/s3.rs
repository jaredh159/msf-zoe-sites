use aws_config::{self, BehaviorVersion};
use aws_sdk_s3::{Client as S3Client, primitives::ByteStream, types::ObjectCannedAcl};
use chrono::Utc;

pub async fn upload_audio_file(file_path: &str, key: &str) -> Result<(), String> {
  let file_content = std::fs::read(file_path).map_err(|e| e.to_string())?;
  put_object(
    &format!("msf-audios/{}", key),
    ByteStream::from(file_content),
    "audio/mpeg",
    Some(ObjectCannedAcl::PublicRead),
  )
  .await
}

pub async fn backup_database() -> Result<(), String> {
  let db_content = std::fs::read("teachings.db").map_err(|e| e.to_string())?;
  let timestamp = Utc::now().format("%Y%m%d-%H%M%S").to_string();
  put_object(
    &format!("db-backups/teachings-{}.db", timestamp),
    ByteStream::from(db_content),
    "application/x-sqlite3",
    None,
  )
  .await
}

async fn get_s3_client() -> S3Client {
  let _ = dotenvy::dotenv();
  let mut config_builder = aws_config::defaults(BehaviorVersion::latest());
  if let Ok(endpoint) = std::env::var("AWS_ENDPOINT_URL") {
    config_builder = config_builder.endpoint_url(&endpoint);
  }
  let config = config_builder.load().await;
  S3Client::new(&config)
}

async fn put_object(
  key: &str,
  body: ByteStream,
  content_type: &str,
  acl: Option<ObjectCannedAcl>,
) -> Result<(), String> {
  let client = get_s3_client().await;
  let mut request = client
    .put_object()
    .bucket("msf-assets")
    .key(key)
    .body(body)
    .content_type(content_type);
  if let Some(acl) = acl {
    request = request.acl(acl);
  }
  request.send().await.map_err(|e| e.to_string())?;
  Ok(())
}
