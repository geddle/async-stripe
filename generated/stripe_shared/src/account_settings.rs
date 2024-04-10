#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountSettings {
    pub bacs_debit_payments: Option<stripe_shared::AccountBacsDebitPaymentsSettings>,
    pub branding: stripe_shared::AccountBrandingSettings,
    pub card_issuing: Option<stripe_shared::AccountCardIssuingSettings>,
    pub card_payments: stripe_shared::AccountCardPaymentsSettings,
    pub dashboard: stripe_shared::AccountDashboardSettings,
    pub invoices: Option<stripe_shared::AccountInvoicesSettings>,
    pub payments: stripe_shared::AccountPaymentsSettings,
    pub payouts: Option<stripe_shared::AccountPayoutSettings>,
    pub sepa_debit_payments: Option<stripe_shared::AccountSepaDebitPaymentsSettings>,
    pub treasury: Option<stripe_shared::AccountTreasurySettings>,
}
#[doc(hidden)]
pub struct AccountSettingsBuilder {
    bacs_debit_payments: Option<Option<stripe_shared::AccountBacsDebitPaymentsSettings>>,
    branding: Option<stripe_shared::AccountBrandingSettings>,
    card_issuing: Option<Option<stripe_shared::AccountCardIssuingSettings>>,
    card_payments: Option<stripe_shared::AccountCardPaymentsSettings>,
    dashboard: Option<stripe_shared::AccountDashboardSettings>,
    invoices: Option<Option<stripe_shared::AccountInvoicesSettings>>,
    payments: Option<stripe_shared::AccountPaymentsSettings>,
    payouts: Option<Option<stripe_shared::AccountPayoutSettings>>,
    sepa_debit_payments: Option<Option<stripe_shared::AccountSepaDebitPaymentsSettings>>,
    treasury: Option<Option<stripe_shared::AccountTreasurySettings>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for AccountSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountSettings>,
        builder: AccountSettingsBuilder,
    }

    impl Visitor for Place<AccountSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountSettingsBuilder {
        type Out = AccountSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bacs_debit_payments" => Deserialize::begin(&mut self.bacs_debit_payments),
                "branding" => Deserialize::begin(&mut self.branding),
                "card_issuing" => Deserialize::begin(&mut self.card_issuing),
                "card_payments" => Deserialize::begin(&mut self.card_payments),
                "dashboard" => Deserialize::begin(&mut self.dashboard),
                "invoices" => Deserialize::begin(&mut self.invoices),
                "payments" => Deserialize::begin(&mut self.payments),
                "payouts" => Deserialize::begin(&mut self.payouts),
                "sepa_debit_payments" => Deserialize::begin(&mut self.sepa_debit_payments),
                "treasury" => Deserialize::begin(&mut self.treasury),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bacs_debit_payments: Deserialize::default(),
                branding: Deserialize::default(),
                card_issuing: Deserialize::default(),
                card_payments: Deserialize::default(),
                dashboard: Deserialize::default(),
                invoices: Deserialize::default(),
                payments: Deserialize::default(),
                payouts: Deserialize::default(),
                sepa_debit_payments: Deserialize::default(),
                treasury: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                bacs_debit_payments: self.bacs_debit_payments.take()?,
                branding: self.branding.take()?,
                card_issuing: self.card_issuing.take()?,
                card_payments: self.card_payments.take()?,
                dashboard: self.dashboard.take()?,
                invoices: self.invoices.take()?,
                payments: self.payments.take()?,
                payouts: self.payouts.take()?,
                sepa_debit_payments: self.sepa_debit_payments.take()?,
                treasury: self.treasury.take()?,
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

    impl ObjectDeser for AccountSettings {
        type Builder = AccountSettingsBuilder;
    }

    impl FromValueOpt for AccountSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = AccountSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bacs_debit_payments" => {
                        b.bacs_debit_payments = Some(FromValueOpt::from_value(v)?)
                    }
                    "branding" => b.branding = Some(FromValueOpt::from_value(v)?),
                    "card_issuing" => b.card_issuing = Some(FromValueOpt::from_value(v)?),
                    "card_payments" => b.card_payments = Some(FromValueOpt::from_value(v)?),
                    "dashboard" => b.dashboard = Some(FromValueOpt::from_value(v)?),
                    "invoices" => b.invoices = Some(FromValueOpt::from_value(v)?),
                    "payments" => b.payments = Some(FromValueOpt::from_value(v)?),
                    "payouts" => b.payouts = Some(FromValueOpt::from_value(v)?),
                    "sepa_debit_payments" => {
                        b.sepa_debit_payments = Some(FromValueOpt::from_value(v)?)
                    }
                    "treasury" => b.treasury = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
