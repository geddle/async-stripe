#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoSelfieReportError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<GelatoSelfieReportErrorCode>,
    /// A human-readable message giving the reason for the failure.
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}
#[doc(hidden)]
pub struct GelatoSelfieReportErrorBuilder {
    code: Option<Option<GelatoSelfieReportErrorCode>>,
    reason: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for GelatoSelfieReportError {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoSelfieReportError>,
        builder: GelatoSelfieReportErrorBuilder,
    }

    impl Visitor for Place<GelatoSelfieReportError> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoSelfieReportErrorBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoSelfieReportErrorBuilder {
        type Out = GelatoSelfieReportError;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "code" => Deserialize::begin(&mut self.code),
                "reason" => Deserialize::begin(&mut self.reason),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { code: Deserialize::default(), reason: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(code), Some(reason)) = (self.code, self.reason.take()) else {
                return None;
            };
            Some(Self::Out { code, reason })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for GelatoSelfieReportError {
        type Builder = GelatoSelfieReportErrorBuilder;
    }

    impl FromValueOpt for GelatoSelfieReportError {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoSelfieReportErrorBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "code" => b.code = FromValueOpt::from_value(v),
                    "reason" => b.reason = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// A short machine-readable string giving the reason for the verification failure.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GelatoSelfieReportErrorCode {
    SelfieDocumentMissingPhoto,
    SelfieFaceMismatch,
    SelfieManipulated,
    SelfieUnverifiedOther,
}
impl GelatoSelfieReportErrorCode {
    pub fn as_str(self) -> &'static str {
        use GelatoSelfieReportErrorCode::*;
        match self {
            SelfieDocumentMissingPhoto => "selfie_document_missing_photo",
            SelfieFaceMismatch => "selfie_face_mismatch",
            SelfieManipulated => "selfie_manipulated",
            SelfieUnverifiedOther => "selfie_unverified_other",
        }
    }
}

impl std::str::FromStr for GelatoSelfieReportErrorCode {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GelatoSelfieReportErrorCode::*;
        match s {
            "selfie_document_missing_photo" => Ok(SelfieDocumentMissingPhoto),
            "selfie_face_mismatch" => Ok(SelfieFaceMismatch),
            "selfie_manipulated" => Ok(SelfieManipulated),
            "selfie_unverified_other" => Ok(SelfieUnverifiedOther),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for GelatoSelfieReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GelatoSelfieReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GelatoSelfieReportErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for GelatoSelfieReportErrorCode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<GelatoSelfieReportErrorCode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(GelatoSelfieReportErrorCode::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(GelatoSelfieReportErrorCode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for GelatoSelfieReportErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for GelatoSelfieReportErrorCode"))
    }
}
