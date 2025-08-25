use aws_config::{self, BehaviorVersion};
use aws_sdk_s3::{Client as S3Client, primitives::ByteStream, types::ObjectCannedAcl};
use chrono::Utc;

pub async fn upload_audio_file(
  file_path: &str,
  key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let _ = dotenvy::dotenv();

  let mut config_builder = aws_config::defaults(BehaviorVersion::latest());
  if let Ok(endpoint) = std::env::var("AWS_ENDPOINT_URL") {
    config_builder = config_builder.endpoint_url(&endpoint);
  }
  let config = config_builder.load().await;
  let client = S3Client::new(&config);

  let file_content = std::fs::read(file_path)?;
  client
    .put_object()
    .bucket("msf-assets")
    .key(&format!("msf-audios/{}", key))
    .body(ByteStream::from(file_content))
    .content_type("audio/mpeg")
    .acl(ObjectCannedAcl::PublicRead)
    .send()
    .await?;

  Ok(())
}
