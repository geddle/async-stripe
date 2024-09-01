#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomFields {
    pub dropdown: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsDropdown>,
    /// String of your choice that your integration can use to reconcile this field.
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,
    pub label: stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsLabel,
    pub numeric: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsNumeric>,
    /// Whether the customer is required to complete the field before completing the Checkout Session.
    /// Defaults to `false`.
    pub optional: bool,
    pub text: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsText>,
    /// The type of the field.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: PaymentPagesCheckoutSessionCustomFieldsType,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionCustomFieldsBuilder {
    dropdown: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsDropdown>>,
    key: Option<String>,
    label: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsLabel>,
    numeric: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsNumeric>>,
    optional: Option<bool>,
    text: Option<Option<stripe_checkout::PaymentPagesCheckoutSessionCustomFieldsText>>,
    type_: Option<PaymentPagesCheckoutSessionCustomFieldsType>,
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

    impl Deserialize for PaymentPagesCheckoutSessionCustomFields {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCustomFields>,
        builder: PaymentPagesCheckoutSessionCustomFieldsBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCustomFields> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionCustomFieldsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCustomFieldsBuilder {
        type Out = PaymentPagesCheckoutSessionCustomFields;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "dropdown" => Deserialize::begin(&mut self.dropdown),
                "key" => Deserialize::begin(&mut self.key),
                "label" => Deserialize::begin(&mut self.label),
                "numeric" => Deserialize::begin(&mut self.numeric),
                "optional" => Deserialize::begin(&mut self.optional),
                "text" => Deserialize::begin(&mut self.text),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                dropdown: Deserialize::default(),
                key: Deserialize::default(),
                label: Deserialize::default(),
                numeric: Deserialize::default(),
                optional: Deserialize::default(),
                text: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(dropdown),
                Some(key),
                Some(label),
                Some(numeric),
                Some(optional),
                Some(text),
                Some(type_),
            ) = (
                self.dropdown.take(),
                self.key.take(),
                self.label.take(),
                self.numeric.take(),
                self.optional,
                self.text.take(),
                self.type_,
            )
            else {
                return None;
            };
            Some(Self::Out { dropdown, key, label, numeric, optional, text, type_ })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCustomFields {
        type Builder = PaymentPagesCheckoutSessionCustomFieldsBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionCustomFields {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionCustomFieldsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "dropdown" => b.dropdown = FromValueOpt::from_value(v),
                    "key" => b.key = FromValueOpt::from_value(v),
                    "label" => b.label = FromValueOpt::from_value(v),
                    "numeric" => b.numeric = FromValueOpt::from_value(v),
                    "optional" => b.optional = FromValueOpt::from_value(v),
                    "text" => b.text = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of the field.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}
impl PaymentPagesCheckoutSessionCustomFieldsType {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionCustomFieldsType::*;
        match self {
            Dropdown => "dropdown",
            Numeric => "numeric",
            Text => "text",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionCustomFieldsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionCustomFieldsType::*;
        match s {
            "dropdown" => Ok(Dropdown),
            "numeric" => Ok(Numeric),
            "text" => Ok(Text),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentPagesCheckoutSessionCustomFieldsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentPagesCheckoutSessionCustomFieldsType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentPagesCheckoutSessionCustomFieldsType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionCustomFieldsType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentPagesCheckoutSessionCustomFieldsType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionCustomFieldsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentPagesCheckoutSessionCustomFieldsType",
            )
        })
    }
}
