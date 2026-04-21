use crate::error::Result;
use std::thread;
use std::time::Duration;

pub struct TTYEncoder;

impl TTYEncoder {
    /// Injects text with throttling to avoid overwhelming the target process's stdin buffer.
    /// Splits text into 64-byte chunks and waits 5ms between each chunk.
    pub fn throttle_inject<F>(text: &str, mut inject_fn: F) -> Result<()>
    where
        F: FnMut(&str) -> Result<()>,
    {
        let chunk_size = 64;
        let mut chars = text.chars().peekable();
        
        while chars.peek().is_some() {
            let chunk_str: String = chars.by_ref().take(chunk_size).collect();
            inject_fn(&chunk_str)?;
            
            // Throttle to 5ms per chunk
            thread::sleep(Duration::from_millis(5));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_throttle_inject() -> Result<()> {
        let mut result = String::new();
        let input = "a".repeat(200);
        
        TTYEncoder::throttle_inject(&input, |chunk| {
            result.push_str(chunk);
            Ok(())
        })?;

        assert_eq!(result, input);
        Ok(())
    }
}
