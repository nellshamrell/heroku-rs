//! Helper functions for end users for Heroku response Headers
use hyper::header::{HeaderValue, ETAG, LAST_MODIFIED, USER_AGENT};
use hyper::HeaderMap;
use std::str::FromStr;

/// Checks to see if a received payload from Heroku contains
/// the Heroku-Hookshot header in the `UserAgent`.
pub fn has_heroku_hookshot(head: &HeaderMap) -> bool {
    head.get(USER_AGENT).map_or(false, |user_agent| {
        user_agent
            .to_str()
            .ok()
            .map_or(false, |s| s.starts_with("Heroku-Hookshot"))
    })
}

/// Extract an `ETag` from the Headers if it exists
pub fn etag(head: &HeaderMap) -> Option<&HeaderValue> {
    head.get(ETAG)
}

/// Extract the Last-Modified from the Headers if it exists
pub fn last_modified(head: &HeaderMap) -> Option<&HeaderValue> {
    head.get(LAST_MODIFIED)
}

/// Extract however many requests the authenticated user can
/// do from the Headers
pub fn rate_limit_remaining(head: &HeaderMap) -> Option<u32> {
    head.get("RateLimit-Remaining")
        .map(|limit| u32::from_str(limit.to_str().unwrap_or("")).ok())
        .unwrap_or(None)
}

/// Extract however many requests the authenticated user can
/// make from the Headers
pub fn rate_limit(head: &HeaderMap) -> Option<u32> {
    head.get("X-RateLimit-Limit")
        .map(|limit| u32::from_str(limit.to_str().unwrap_or("")).ok())
        .unwrap_or(None)
}

/// Extract when the requests limit for the authenticated user
/// is reset from the Headers
pub fn rate_limit_reset(head: &HeaderMap) -> Option<u32> {
    head.get("X-RateLimit-Reset")
        .map(|limit| u32::from_str(limit.to_str().unwrap_or("")).ok())
        .unwrap_or(None)
}