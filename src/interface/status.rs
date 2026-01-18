use bevy::prelude::*;

/// HTTP status code.
#[derive(Component, Clone, Copy, Debug)]
pub struct StatusCode(pub(crate) u16);

impl StatusCode {
    /// Create a new status code.
    #[inline]
    pub const fn new(code: u16) -> Self {
        Self(code)
    }

    /// Get the status code as a u16 value.
    #[inline]
    pub const fn code(&self) -> u16 {
        self.0
    }

    /// Check if status is within 100-199.
    #[inline]
    pub const fn is_informational(&self) -> bool {
        100 <= self.0 && self.0 < 200
    }

    /// Check if status is within 200-299.
    #[inline]
    pub const fn is_successful(&self) -> bool {
        200 <= self.0 && self.0 < 300
    }

    /// Check if status is within 300-399.
    #[inline]
    pub const fn is_redirection(&self) -> bool {
        300 <= self.0 && self.0 < 400
    }

    /// Check if status is within 400-499.
    #[inline]
    pub const fn is_client_error(&self) -> bool {
        400 <= self.0 && self.0 < 500
    }

    /// Check if status is within 500-599.
    #[inline]
    pub const fn is_server_error(&self) -> bool {
        500 <= self.0 && self.0 < 600
    }

    /// Check if status is outside the range of 100-599.
    #[inline]
    pub const fn is_invalid(&self) -> bool {
        self.0 < 100 || self.0 > 599
    }
}
