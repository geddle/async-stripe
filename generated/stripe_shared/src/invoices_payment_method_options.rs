#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicesPaymentMethodOptions {
    /// If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    pub acss_debit: Option<stripe_shared::InvoicePaymentMethodOptionsAcssDebit>,
    /// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    pub bancontact: Option<stripe_shared::InvoicePaymentMethodOptionsBancontact>,
    /// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    pub card: Option<stripe_shared::InvoicePaymentMethodOptionsCard>,
    /// If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    pub customer_balance: Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalance>,
    /// If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    pub konbini: Option<stripe_shared::InvoicePaymentMethodOptionsKonbini>,
    /// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    pub us_bank_account: Option<stripe_shared::InvoicePaymentMethodOptionsUsBankAccount>,
}
#[doc(hidden)]
pub struct InvoicesPaymentMethodOptionsBuilder {
    acss_debit: Option<Option<stripe_shared::InvoicePaymentMethodOptionsAcssDebit>>,
    bancontact: Option<Option<stripe_shared::InvoicePaymentMethodOptionsBancontact>>,
    card: Option<Option<stripe_shared::InvoicePaymentMethodOptionsCard>>,
    customer_balance: Option<Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalance>>,
    konbini: Option<Option<stripe_shared::InvoicePaymentMethodOptionsKonbini>>,
    us_bank_account: Option<Option<stripe_shared::InvoicePaymentMethodOptionsUsBankAccount>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for InvoicesPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesPaymentMethodOptions>,
        builder: InvoicesPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<InvoicesPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicesPaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicesPaymentMethodOptionsBuilder {
        type Out = InvoicesPaymentMethodOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => Deserialize::begin(&mut self.acss_debit),
                "bancontact" => Deserialize::begin(&mut self.bancontact),
                "card" => Deserialize::begin(&mut self.card),
                "customer_balance" => Deserialize::begin(&mut self.customer_balance),
                "konbini" => Deserialize::begin(&mut self.konbini),
                "us_bank_account" => Deserialize::begin(&mut self.us_bank_account),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acss_debit: Deserialize::default(),
                bancontact: Deserialize::default(),
                card: Deserialize::default(),
                customer_balance: Deserialize::default(),
                konbini: Deserialize::default(),
                us_bank_account: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                acss_debit: self.acss_debit?,
                bancontact: self.bancontact?,
                card: self.card?,
                customer_balance: self.customer_balance.take()?,
                konbini: self.konbini?,
                us_bank_account: self.us_bank_account.take()?,
            })
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

    impl ObjectDeser for InvoicesPaymentMethodOptions {
        type Builder = InvoicesPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for InvoicesPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicesPaymentMethodOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "acss_debit" => b.acss_debit = Some(FromValueOpt::from_value(v)?),
                    "bancontact" => b.bancontact = Some(FromValueOpt::from_value(v)?),
                    "card" => b.card = Some(FromValueOpt::from_value(v)?),
                    "customer_balance" => b.customer_balance = Some(FromValueOpt::from_value(v)?),
                    "konbini" => b.konbini = Some(FromValueOpt::from_value(v)?),
                    "us_bank_account" => b.us_bank_account = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
