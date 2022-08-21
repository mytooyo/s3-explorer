use std::error;

pub enum AwsErrorKind {
    AccessDenied,
    ExpiredToken,
    NotFoundObject,
}

enum _Error {
    Simple(AwsErrorKind),
}

pub struct AwsError {
    _error: _Error,
}

impl AwsError {
    pub fn new(kind: AwsErrorKind) -> Self {
        AwsError {
            _error: _Error::Simple(kind),
        }
    }

    // pub fn custom<T>(kind: AwsErrorKind, error: T) -> Self
    // where
    //     T: Into<Box<dyn error::Error + Send + Sync>>,
    // {
    //     AwsError {
    //         _error: _Error::Custom((kind, error.into())),
    //     }
    // }

    pub fn name(&self) -> String {
        let kind = match &self._error {
            _Error::Simple(kind) => kind,
        };
        kind.name().to_string()
    }
}

impl AwsErrorKind {
    pub fn name(&self) -> &'static str {
        match self {
            AwsErrorKind::AccessDenied => "AccessDenied",
            AwsErrorKind::ExpiredToken => "ExpiredToken",
            AwsErrorKind::NotFoundObject => "NotFoundObject",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            AwsErrorKind::AccessDenied => "Access denied.",
            AwsErrorKind::ExpiredToken => "The provided token has expired.",
            AwsErrorKind::NotFoundObject => "Not found object in bucket.",
        }
    }
}

impl std::fmt::Display for AwsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self._error {
            _Error::Simple(s) => f.write_str(s.description()),
            // _Error::Custom(c) => f.write_str(c.0.description()),
        }
    }
}

impl std::fmt::Debug for AwsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

impl std::error::Error for AwsError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self._error {
            _Error::Simple(_) => None,
            // _Error::Custom(c) => c.1.source(),
        }
    }
}
