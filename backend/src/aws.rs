use aws_config::Region;
use aws_credential_types::{provider::SharedCredentialsProvider, Credentials};
use aws_sdk_ec2::Client as Ec2Client;

pub struct AwsClientFactory {
    crypto_config: crate::crypto::CryptoConfig,
}

impl AwsClientFactory {
    pub fn new(crypto_config: crate::crypto::CryptoConfig) -> Self {
        Self { crypto_config }
    }

    pub async fn create_ec2_client(
        &self,
        access_key_id_encrypted: &str,
        secret_access_key_encrypted: &str,
        region: &str,
    ) -> Result<Ec2Client, String> {
        let access_key_id = self
            .crypto_config
            .decrypt(access_key_id_encrypted)
            .map_err(|e| format!("failed to decrypt access key: {}", e))?;

        let secret_access_key = self
            .crypto_config
            .decrypt(secret_access_key_encrypted)
            .map_err(|e| format!("failed to decrypt secret key: {}", e))?;

        let credentials = Credentials::new(
            access_key_id,
            secret_access_key,
            None,
            None,
            "dashboard",
        );

        let credentials_provider = SharedCredentialsProvider::new(credentials);

        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(Region::new(region.to_string()))
            .credentials_provider(credentials_provider)
            .load()
            .await;

        Ok(Ec2Client::new(&config))
    }
}
