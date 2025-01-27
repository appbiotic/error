use std::{collections::BTreeMap, fmt};

// TODO: Add short summarizing docs referring to primary source

#[cfg_attr(feature = "derive-new", derive(derive_new::new))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")
)]
#[derive(Debug, Clone)]
pub enum ErrorDetails {
    ErrorInfo(ErrorInfo),
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "derive-new", derive(derive_new::new))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ErrorInfo {
    // TODO: Add validation for [ErrorInfo::reason]
    #[cfg_attr(feature = "derive-new", new(into))]
    pub reason: String,
    // TODO: Add validation for [ErrorInfo::domain]
    #[cfg_attr(feature = "derive-new", new(into))]
    pub domain: String,
    // TODO: Add validation for [ErrorInfo::metadata] keys
    #[cfg_attr(feature = "derive-new", new(default))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "BTreeMap::is_empty")
    )]
    pub metadata: BTreeMap<String, String>,
}

#[derive(Clone, Debug, strum::IntoStaticStr, thiserror::Error)]
#[cfg_attr(feature = "derive-new", derive(derive_new::new))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(tag = "code", rename_all = "SCREAMING_SNAKE_CASE")
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    Cancelled(StatusDetails),

    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    Unknown(StatusDetails),

    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    InvalidArgument(StatusDetails),

    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    DeadlineExceeded(StatusDetails),

    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    NotFound(StatusDetails),

    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    AlreadyExists(StatusDetails),

    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    PermissionDenied(StatusDetails),

    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    Unauthenticated(StatusDetails),

    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    ResourceExhaused(StatusDetails),

    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    FailedPrecondition(StatusDetails),

    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    Aborted(StatusDetails),

    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    OutOfRange(StatusDetails),

    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    Unimplemented(StatusDetails),

    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    Internal(StatusDetails),

    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    Unavailable(StatusDetails),

    #[error("{}: {}", Into::<&'static str>::into(self), .0)]
    DataLoss(StatusDetails),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "derive-new", derive(derive_new::new))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct StatusDetails {
    #[cfg_attr(feature = "derive-new", new(into))]
    pub message: String,
    #[cfg_attr(feature = "derive-new", new(into_iter = "ErrorDetails"))]
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub error_details: Vec<ErrorDetails>,
}

impl fmt::Display for StatusDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

pub type StatusResult<T> = Result<T, Status>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[cfg_attr(feature = "derive-new", derive(derive_new::new))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ValidationError {
    #[error("{}: {message}", Into::<&'static str>::into(self))]
    InvalidFormat {
        #[cfg_attr(feature = "derive-new", new(into))]
        message: String,
    },
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::*;

    #[test]
    fn status_message() {
        let status = Status::new_unknown(StatusDetails::new(
            "Unsure about that",
            [ErrorDetails::new_error_info(ErrorInfo::new(
                "UNKNOWN_FAULT",
                "com.appbiotic.error",
            ))],
        ));

        assert_eq!("UNKNOWN: Unsure about that", status.to_string());
    }

    #[test]
    fn status_serialization() {
        let status = Status::new_unknown(StatusDetails::new(
            "Unsure about that",
            [ErrorDetails::new_error_info(ErrorInfo::new(
                "UNKNOWN_FAULT",
                "com.appbiotic.error",
            ))],
        ));
        let value = serde_json::to_value(&status).unwrap();
        let expected = json!({
            "code": "UNKNOWN",
            "message": "Unsure about that",
            "error_details": [
                {
                    "type": "ERROR_INFO",
                    "reason": "UNKNOWN_FAULT",
                    "domain": "com.appbiotic.error"
                },
            ],
        });
        assert_eq!(value, expected);
    }

    #[test]
    fn validation_error_message() {
        let error = ValidationError::new_invalid_format("did not match regex");
        assert_eq!("INVALID_FORMAT: did not match regex", error.to_string());
    }

    #[test]
    fn validation_error_serialization() {
        let error = ValidationError::new_invalid_format("did not match regex");
        let value = serde_json::to_value(&error).unwrap();
        let expected = json!({
            "type": "INVALID_FORMAT",
            "message": "did not match regex"
        });
        assert_eq!(value, expected);
    }
}
