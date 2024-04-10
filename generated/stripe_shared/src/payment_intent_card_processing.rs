#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentCardProcessing {
    pub customer_notification: Option<stripe_shared::PaymentIntentProcessingCustomerNotification>,
}
#[doc(hidden)]
pub struct PaymentIntentCardProcessingBuilder {
    customer_notification:
        Option<Option<stripe_shared::PaymentIntentProcessingCustomerNotification>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentIntentCardProcessing {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentCardProcessing>,
        builder: PaymentIntentCardProcessingBuilder,
    }

    impl Visitor for Place<PaymentIntentCardProcessing> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentCardProcessingBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentCardProcessingBuilder {
        type Out = PaymentIntentCardProcessing;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "customer_notification" => Deserialize::begin(&mut self.customer_notification),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { customer_notification: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { customer_notification: self.customer_notification? })
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

    impl ObjectDeser for PaymentIntentCardProcessing {
        type Builder = PaymentIntentCardProcessingBuilder;
    }

    impl FromValueOpt for PaymentIntentCardProcessing {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentCardProcessingBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "customer_notification" => {
                        b.customer_notification = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
