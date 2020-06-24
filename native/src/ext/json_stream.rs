use crate::prelude::*;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait JsonStreamWriteExt {
    async fn write_json<T: Serialize + Send + Sync + 'static>(&mut self, value: &T) -> crate::Result<()>;
}

#[async_trait]
impl<R: AsyncWrite + Unpin + Send + Sync + 'static> JsonStreamWriteExt for R {
    async fn write_json<T: Serialize + Send + Sync + 'static>(&mut self, value: &T) -> crate::Result<()> {
        let json_data =
            serde_json::to_vec(value).context("Failed to encode the provided value to JSON for json-stream")?;

        self.write_u64(json_data.len() as u64)
            .await
            .context("Failed to write json-stream data byte size")?;

        self.write_all(&json_data)
            .await
            .context("Failed to write json-stream data")?;

        Ok(())
    }
}

#[async_trait]
pub trait JsonStreamReadExt {
    async fn read_json<T: DeserializeOwned + Send + Sync + 'static>(&mut self) -> crate::Result<T>;
}

#[async_trait]
impl<R: AsyncRead + Unpin + Send + Sync + 'static> JsonStreamReadExt for R {
    async fn read_json<T: DeserializeOwned + Send + Sync + 'static>(&mut self) -> crate::Result<T> {
        let json_data_size = self
            .read_u64()
            .await
            .context("Failed to read json-stream data byte size")?;

        let mut json_data = vec![0_u8; json_data_size as usize];
        let read_len = self
            .read_exact(&mut json_data)
            .await
            .context("Failed to read json-stream data")?;

        if read_len != json_data_size as usize {
            return Err(crate::Error::new("Failed to read json-stream data completely"));
        }

        serde_json::from_slice::<T>(&json_data).context("Failed to parse json-stream data as JSON")
    }
}
