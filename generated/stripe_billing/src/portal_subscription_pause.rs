#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalSubscriptionPause {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
#[doc(hidden)]
pub struct PortalSubscriptionPauseBuilder {
    enabled: Option<bool>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalSubscriptionPause {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalSubscriptionPause>,
        builder: PortalSubscriptionPauseBuilder,
    }

    impl Visitor for Place<PortalSubscriptionPause> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalSubscriptionPauseBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalSubscriptionPauseBuilder {
        type Out = PortalSubscriptionPause;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.enabled),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { enabled: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { enabled: self.enabled? })
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

    impl ObjectDeser for PortalSubscriptionPause {
        type Builder = PortalSubscriptionPauseBuilder;
    }

    impl FromValueOpt for PortalSubscriptionPause {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalSubscriptionPauseBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "enabled" => b.enabled = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
