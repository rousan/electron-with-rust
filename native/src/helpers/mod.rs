use crate::prelude::*;
use regex::Regex;
use std::path::{Path, PathBuf};
use tokio::io::ErrorKind;
use tokio::time::{Duration, Instant};
use uuid::Uuid;

const STREAM_PROGRESS_CALL_THROTTLE_DURATION: Duration = Duration::from_millis(100);

pub fn gen_uuid() -> String {
    Uuid::new_v4()
        .to_hyphenated()
        .encode_lower(&mut Uuid::encode_buffer())
        .to_string()
}

pub async fn pipe<R, W, F>(reader: &mut R, writer: &mut W, progress: F) -> crate::Result<()>
where
    R: AsyncRead + Unpin + Send + Sync,
    W: AsyncWrite + Unpin + Send + Sync,
    F: Fn(u64) + Send + Sync,
{
    let mut buf = vec![0_u8; 16 * 1024];

    let mut written = 0;
    let mut called_progress: Option<Instant> = None;
    loop {
        let len = match reader.read(&mut buf).await {
            Ok(0) => break,
            Ok(len) => len,
            Err(ref err) if err.kind() == ErrorKind::Interrupted => continue,
            Err(err) => return Err(err.context("Failed to read data from the reader while piping")),
        };

        writer
            .write_all(&buf[..len])
            .await
            .context("Failed to write data to the writer while piping")?;

        written += len as u64;

        match called_progress {
            Some(i) => {
                if i.elapsed() > STREAM_PROGRESS_CALL_THROTTLE_DURATION {
                    progress(written);
                    called_progress = Some(Instant::now());
                }
            }
            None => {
                progress(written);
                called_progress = Some(Instant::now());
            }
        }
    }

    progress(written);

    Ok(())
}

pub fn generate_file_path_with_available_name(parent_path: impl AsRef<Path>, name: &str) -> crate::Result<PathBuf> {
    let parent_path = parent_path.as_ref();
    let ext = Path::new(name).extension().and_then(|ext| ext.to_str());
    let re = Regex::new(format!(r"\.{}$", regex::escape(ext.unwrap_or(""))).as_str())
        .context("Failed to create regex for generating available file name")?;

    let mut curr_file_path = parent_path.join(name);
    let mut counter = 1_u64;

    loop {
        if !curr_file_path.exists() {
            return Ok(curr_file_path);
        }

        match ext {
            Some(ext) => {
                let new_name = re.replace(name, format!("({}).{}", counter, ext).as_str()).to_string();
                curr_file_path = parent_path.join(new_name.as_str());
            }
            None => {
                curr_file_path = parent_path.join(format!("{}({})", name, counter));
            }
        }

        counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_file_path_with_available_name() {
        let parent_path = "/Users/rousan/Desktop/electron-with-rust-outputs";

        println!("{:?}", generate_file_path_with_available_name(parent_path, "abc.txt"));
        println!(
            "{:?}",
            generate_file_path_with_available_name(parent_path, "abc.txt.txt")
        );
        println!("{:?}", generate_file_path_with_available_name(parent_path, "abc"));
        println!("{:?}", generate_file_path_with_available_name(parent_path, ".abc.txt"));
        println!("{:?}", generate_file_path_with_available_name(parent_path, ".abc"));
    }
}
