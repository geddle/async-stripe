#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceTaxTransactionLineItemResourceReversal {
    /// The `id` of the line item to reverse in the original transaction.
    pub original_line_item: String,
}
#[doc(hidden)]
pub struct TaxProductResourceTaxTransactionLineItemResourceReversalBuilder {
    original_line_item: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceTaxTransactionLineItemResourceReversal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceTaxTransactionLineItemResourceReversal>,
        builder: TaxProductResourceTaxTransactionLineItemResourceReversalBuilder,
    }

    impl Visitor for Place<TaxProductResourceTaxTransactionLineItemResourceReversal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    TaxProductResourceTaxTransactionLineItemResourceReversalBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceTaxTransactionLineItemResourceReversalBuilder {
        type Out = TaxProductResourceTaxTransactionLineItemResourceReversal;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "original_line_item" => Deserialize::begin(&mut self.original_line_item),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { original_line_item: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { original_line_item: self.original_line_item.take()? })
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

    impl ObjectDeser for TaxProductResourceTaxTransactionLineItemResourceReversal {
        type Builder = TaxProductResourceTaxTransactionLineItemResourceReversalBuilder;
    }

    impl FromValueOpt for TaxProductResourceTaxTransactionLineItemResourceReversal {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TaxProductResourceTaxTransactionLineItemResourceReversalBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "original_line_item" => {
                        b.original_line_item = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
