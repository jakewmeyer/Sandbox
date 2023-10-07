use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct TokenBucket {
    capacity: u8,
    available_tokens: u8,
    last_update: Instant,
    fill_rate: u8,
    take_rate: u8,
}

impl Default for TokenBucket {
    fn default() -> Self {
        Self::new(1, 1, 1)
    }
}

impl TokenBucket {
    pub fn new(capacity: u8, fill_rate: u8, take_rate: u8) -> Self {
        Self {
            capacity,
            available_tokens: capacity,
            last_update: Instant::now(),
            fill_rate,
            take_rate,
        }
    }

    pub fn take(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs();
        let tokens_to_add = (elapsed as u8).saturating_mul(self.fill_rate);
        // Check if we have at least one token to add
        // to prevent fractional token tracking
        if tokens_to_add >= 1 {
            self.available_tokens = (self.available_tokens + tokens_to_add).min(self.capacity);
            self.last_update = now;
        }
        if self.available_tokens >= self.take_rate {
            self.available_tokens -= self.take_rate;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_bucket() {
        let mut bucket = TokenBucket::new(1, 1, 1);
        assert!(bucket.take());
        assert!(!bucket.take());
    }

    #[test]
    fn test_token_bucket_fill_rate() {
        let mut bucket = TokenBucket::new(1, 1, 1);
        assert!(bucket.take());
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert!(bucket.take());
        assert!(!bucket.take());
    }
}
