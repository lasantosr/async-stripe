use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct DeleteSubscriptionItemBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    clear_usage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_behavior: Option<DeleteSubscriptionItemProrationBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_date: Option<stripe_types::Timestamp>,
}
impl DeleteSubscriptionItemBuilder {
    fn new() -> Self {
        Self { clear_usage: None, proration_behavior: None, proration_date: None }
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
/// The default value is `create_prorations`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DeleteSubscriptionItemProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl DeleteSubscriptionItemProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use DeleteSubscriptionItemProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for DeleteSubscriptionItemProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeleteSubscriptionItemProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for DeleteSubscriptionItemProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DeleteSubscriptionItemProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for DeleteSubscriptionItemProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DeleteSubscriptionItemProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for DeleteSubscriptionItemProrationBehavior")
        })
    }
}
/// Deletes an item from the subscription.
/// Removing a subscription item from a subscription will not cancel the subscription.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct DeleteSubscriptionItem<'a> {
    inner: DeleteSubscriptionItemBuilder,
    item: &'a stripe_shared::SubscriptionItemId,
}
impl<'a> DeleteSubscriptionItem<'a> {
    /// Construct a new `DeleteSubscriptionItem`.
    pub fn new(item: &'a stripe_shared::SubscriptionItemId) -> Self {
        Self { item, inner: DeleteSubscriptionItemBuilder::new() }
    }
    /// Delete all usage for the given subscription item.
    /// Allowed only when the current plan's `usage_type` is `metered`.
    pub fn clear_usage(mut self, clear_usage: bool) -> Self {
        self.inner.clear_usage = Some(clear_usage);
        self
    }
    /// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    /// The default value is `create_prorations`.
    pub fn proration_behavior(
        mut self,
        proration_behavior: DeleteSubscriptionItemProrationBehavior,
    ) -> Self {
        self.inner.proration_behavior = Some(proration_behavior);
        self
    }
    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    /// This can be used to apply the same proration that was previewed with the [upcoming invoice](https://stripe.com/docs/api#retrieve_customer_invoice) endpoint.
    pub fn proration_date(mut self, proration_date: stripe_types::Timestamp) -> Self {
        self.inner.proration_date = Some(proration_date);
        self
    }
}
impl DeleteSubscriptionItem<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for DeleteSubscriptionItem<'_> {
    type Output = stripe_shared::DeletedSubscriptionItem;

    fn build(&self) -> RequestBuilder {
        let item = self.item;
        RequestBuilder::new(StripeMethod::Delete, format!("/subscription_items/{item}"))
            .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ListSubscriptionItemBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    subscription: &'a str,
}
impl<'a> ListSubscriptionItemBuilder<'a> {
    fn new(subscription: &'a str) -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None, subscription }
    }
}
/// Returns a list of your subscription items for a given subscription.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ListSubscriptionItem<'a> {
    inner: ListSubscriptionItemBuilder<'a>,
}
impl<'a> ListSubscriptionItem<'a> {
    /// Construct a new `ListSubscriptionItem`.
    pub fn new(subscription: &'a str) -> Self {
        Self { inner: ListSubscriptionItemBuilder::new(subscription) }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl ListSubscriptionItem<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::SubscriptionItem>>
    {
        stripe_client_core::ListPaginator::new_list("/subscription_items", self.inner)
    }
}

impl StripeRequest for ListSubscriptionItem<'_> {
    type Output = stripe_types::List<stripe_shared::SubscriptionItem>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/subscription_items").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrieveSubscriptionItemBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveSubscriptionItemBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the subscription item with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrieveSubscriptionItem<'a> {
    inner: RetrieveSubscriptionItemBuilder<'a>,
    item: &'a stripe_shared::SubscriptionItemId,
}
impl<'a> RetrieveSubscriptionItem<'a> {
    /// Construct a new `RetrieveSubscriptionItem`.
    pub fn new(item: &'a stripe_shared::SubscriptionItemId) -> Self {
        Self { item, inner: RetrieveSubscriptionItemBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveSubscriptionItem<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveSubscriptionItem<'_> {
    type Output = stripe_shared::SubscriptionItem;

    fn build(&self) -> RequestBuilder {
        let item = self.item;
        RequestBuilder::new(StripeMethod::Get, format!("/subscription_items/{item}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct UsageRecordSummariesSubscriptionItemBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> UsageRecordSummariesSubscriptionItemBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// For the specified subscription item, returns a list of summary objects.
/// Each object in the list provides usage information that’s been summarized from multiple usage records and over a subscription billing period (e.g., 15 usage records in the month of September).
///
/// The list is sorted in reverse-chronological order (newest first).
/// The first list item represents the most current usage period that hasn’t ended yet.
/// Since new usage records can still be added, the returned summary information for the subscription item’s ID should be seen as unstable until the subscription billing period ends.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct UsageRecordSummariesSubscriptionItem<'a> {
    inner: UsageRecordSummariesSubscriptionItemBuilder<'a>,
    subscription_item: &'a stripe_shared::SubscriptionItemId,
}
impl<'a> UsageRecordSummariesSubscriptionItem<'a> {
    /// Construct a new `UsageRecordSummariesSubscriptionItem`.
    pub fn new(subscription_item: &'a stripe_shared::SubscriptionItemId) -> Self {
        Self { subscription_item, inner: UsageRecordSummariesSubscriptionItemBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl UsageRecordSummariesSubscriptionItem<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::UsageRecordSummary>>
    {
        let subscription_item = self.subscription_item;

        stripe_client_core::ListPaginator::new_list(
            format!("/subscription_items/{subscription_item}/usage_record_summaries"),
            self.inner,
        )
    }
}

impl StripeRequest for UsageRecordSummariesSubscriptionItem<'_> {
    type Output = stripe_types::List<stripe_shared::UsageRecordSummary>;

    fn build(&self) -> RequestBuilder {
        let subscription_item = self.subscription_item;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/subscription_items/{subscription_item}/usage_record_summaries"),
        )
        .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateSubscriptionItemBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_thresholds: Option<ItemBillingThresholdsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<&'a [DiscountsDataParam<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_behavior: Option<CreateSubscriptionItemPaymentBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plan: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data: Option<CreateSubscriptionItemPriceData<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_behavior: Option<CreateSubscriptionItemProrationBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_date: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<u64>,
    subscription: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_rates: Option<&'a [&'a str]>,
}
impl<'a> CreateSubscriptionItemBuilder<'a> {
    fn new(subscription: &'a str) -> Self {
        Self {
            billing_thresholds: None,
            discounts: None,
            expand: None,
            metadata: None,
            payment_behavior: None,
            plan: None,
            price: None,
            price_data: None,
            proration_behavior: None,
            proration_date: None,
            quantity: None,
            subscription,
            tax_rates: None,
        }
    }
}
/// Use `allow_incomplete` to transition the subscription to `status=past_due` if a payment is required but cannot be paid.
/// This allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice.
/// For example, SCA regulation may require 3DS authentication to complete payment.
/// See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more.
/// This is the default behavior.
///
/// Use `default_incomplete` to transition the subscription to `status=past_due` when payment is required and await explicit confirmation of the invoice's payment intent.
/// This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice.
/// Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method.
///
/// Use `pending_if_incomplete` to update the subscription using [pending updates](https://stripe.com/docs/billing/subscriptions/pending-updates).
/// When you use `pending_if_incomplete` you can only pass the parameters [supported by pending updates](https://stripe.com/docs/billing/pending-updates-reference#supported-attributes).
///
/// Use `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's invoice cannot be paid.
/// For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not update the subscription and returns an error instead.
/// This was the default behavior for API versions prior to 2019-03-14.
/// See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionItemPaymentBehavior {
    AllowIncomplete,
    DefaultIncomplete,
    ErrorIfIncomplete,
    PendingIfIncomplete,
}
impl CreateSubscriptionItemPaymentBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionItemPaymentBehavior::*;
        match self {
            AllowIncomplete => "allow_incomplete",
            DefaultIncomplete => "default_incomplete",
            ErrorIfIncomplete => "error_if_incomplete",
            PendingIfIncomplete => "pending_if_incomplete",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionItemPaymentBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionItemPaymentBehavior::*;
        match s {
            "allow_incomplete" => Ok(AllowIncomplete),
            "default_incomplete" => Ok(DefaultIncomplete),
            "error_if_incomplete" => Ok(ErrorIfIncomplete),
            "pending_if_incomplete" => Ok(PendingIfIncomplete),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionItemPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionItemPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionItemPaymentBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionItemPaymentBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateSubscriptionItemPaymentBehavior")
        })
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateSubscriptionItemPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: CreateSubscriptionItemPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateSubscriptionItemPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateSubscriptionItemPriceData<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        product: &'a str,
        recurring: CreateSubscriptionItemPriceDataRecurring,
    ) -> Self {
        Self {
            currency,
            product,
            recurring,
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateSubscriptionItemPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: CreateSubscriptionItemPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl CreateSubscriptionItemPriceDataRecurring {
    pub fn new(interval: CreateSubscriptionItemPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionItemPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl CreateSubscriptionItemPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionItemPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionItemPriceDataRecurringInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionItemPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionItemPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionItemPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionItemPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionItemPriceDataRecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateSubscriptionItemPriceDataRecurringInterval",
            )
        })
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreateSubscriptionItemPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionItemPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionItemPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionItemPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionItemPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionItemPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateSubscriptionItemPriceDataTaxBehavior")
        })
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
/// The default value is `create_prorations`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateSubscriptionItemProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl CreateSubscriptionItemProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateSubscriptionItemProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateSubscriptionItemProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateSubscriptionItemProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateSubscriptionItemProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateSubscriptionItemProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateSubscriptionItemProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateSubscriptionItemProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateSubscriptionItemProrationBehavior")
        })
    }
}
/// Adds a new item to an existing subscription. No existing items will be changed or replaced.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateSubscriptionItem<'a> {
    inner: CreateSubscriptionItemBuilder<'a>,
}
impl<'a> CreateSubscriptionItem<'a> {
    /// Construct a new `CreateSubscriptionItem`.
    pub fn new(subscription: &'a str) -> Self {
        Self { inner: CreateSubscriptionItemBuilder::new(subscription) }
    }
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// When updating, pass an empty string to remove previously-defined thresholds.
    pub fn billing_thresholds(mut self, billing_thresholds: ItemBillingThresholdsParam) -> Self {
        self.inner.billing_thresholds = Some(billing_thresholds);
        self
    }
    /// The coupons to redeem into discounts for the subscription item.
    pub fn discounts(mut self, discounts: &'a [DiscountsDataParam<'a>]) -> Self {
        self.inner.discounts = Some(discounts);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// Use `allow_incomplete` to transition the subscription to `status=past_due` if a payment is required but cannot be paid.
    /// This allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice.
    /// For example, SCA regulation may require 3DS authentication to complete payment.
    /// See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more.
    /// This is the default behavior.
    ///
    /// Use `default_incomplete` to transition the subscription to `status=past_due` when payment is required and await explicit confirmation of the invoice's payment intent.
    /// This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice.
    /// Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method.
    ///
    /// Use `pending_if_incomplete` to update the subscription using [pending updates](https://stripe.com/docs/billing/subscriptions/pending-updates).
    /// When you use `pending_if_incomplete` you can only pass the parameters [supported by pending updates](https://stripe.com/docs/billing/pending-updates-reference#supported-attributes).
    ///
    /// Use `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's invoice cannot be paid.
    /// For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not update the subscription and returns an error instead.
    /// This was the default behavior for API versions prior to 2019-03-14.
    /// See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more.
    pub fn payment_behavior(
        mut self,
        payment_behavior: CreateSubscriptionItemPaymentBehavior,
    ) -> Self {
        self.inner.payment_behavior = Some(payment_behavior);
        self
    }
    /// The identifier of the plan to add to the subscription.
    pub fn plan(mut self, plan: &'a str) -> Self {
        self.inner.plan = Some(plan);
        self
    }
    /// The ID of the price object.
    pub fn price(mut self, price: &'a str) -> Self {
        self.inner.price = Some(price);
        self
    }
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    pub fn price_data(mut self, price_data: CreateSubscriptionItemPriceData<'a>) -> Self {
        self.inner.price_data = Some(price_data);
        self
    }
    /// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    /// The default value is `create_prorations`.
    pub fn proration_behavior(
        mut self,
        proration_behavior: CreateSubscriptionItemProrationBehavior,
    ) -> Self {
        self.inner.proration_behavior = Some(proration_behavior);
        self
    }
    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    /// This can be used to apply the same proration that was previewed with the [upcoming invoice](https://stripe.com/docs/api#retrieve_customer_invoice) endpoint.
    pub fn proration_date(mut self, proration_date: stripe_types::Timestamp) -> Self {
        self.inner.proration_date = Some(proration_date);
        self
    }
    /// The quantity you'd like to apply to the subscription item you're creating.
    pub fn quantity(mut self, quantity: u64) -> Self {
        self.inner.quantity = Some(quantity);
        self
    }
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    pub fn tax_rates(mut self, tax_rates: &'a [&'a str]) -> Self {
        self.inner.tax_rates = Some(tax_rates);
        self
    }
}
impl CreateSubscriptionItem<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateSubscriptionItem<'_> {
    type Output = stripe_shared::SubscriptionItem;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/subscription_items").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct UpdateSubscriptionItemBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_thresholds: Option<ItemBillingThresholdsParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<&'a [DiscountsDataParam<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    off_session: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_behavior: Option<UpdateSubscriptionItemPaymentBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plan: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data: Option<UpdateSubscriptionItemPriceData<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_behavior: Option<UpdateSubscriptionItemProrationBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_date: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_rates: Option<&'a [&'a str]>,
}
impl<'a> UpdateSubscriptionItemBuilder<'a> {
    fn new() -> Self {
        Self {
            billing_thresholds: None,
            discounts: None,
            expand: None,
            metadata: None,
            off_session: None,
            payment_behavior: None,
            plan: None,
            price: None,
            price_data: None,
            proration_behavior: None,
            proration_date: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
/// Use `allow_incomplete` to transition the subscription to `status=past_due` if a payment is required but cannot be paid.
/// This allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice.
/// For example, SCA regulation may require 3DS authentication to complete payment.
/// See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more.
/// This is the default behavior.
///
/// Use `default_incomplete` to transition the subscription to `status=past_due` when payment is required and await explicit confirmation of the invoice's payment intent.
/// This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice.
/// Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method.
///
/// Use `pending_if_incomplete` to update the subscription using [pending updates](https://stripe.com/docs/billing/subscriptions/pending-updates).
/// When you use `pending_if_incomplete` you can only pass the parameters [supported by pending updates](https://stripe.com/docs/billing/pending-updates-reference#supported-attributes).
///
/// Use `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's invoice cannot be paid.
/// For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not update the subscription and returns an error instead.
/// This was the default behavior for API versions prior to 2019-03-14.
/// See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionItemPaymentBehavior {
    AllowIncomplete,
    DefaultIncomplete,
    ErrorIfIncomplete,
    PendingIfIncomplete,
}
impl UpdateSubscriptionItemPaymentBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionItemPaymentBehavior::*;
        match self {
            AllowIncomplete => "allow_incomplete",
            DefaultIncomplete => "default_incomplete",
            ErrorIfIncomplete => "error_if_incomplete",
            PendingIfIncomplete => "pending_if_incomplete",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionItemPaymentBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionItemPaymentBehavior::*;
        match s {
            "allow_incomplete" => Ok(AllowIncomplete),
            "default_incomplete" => Ok(DefaultIncomplete),
            "error_if_incomplete" => Ok(ErrorIfIncomplete),
            "pending_if_incomplete" => Ok(PendingIfIncomplete),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionItemPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionItemPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionItemPaymentBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSubscriptionItemPaymentBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateSubscriptionItemPaymentBehavior")
        })
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct UpdateSubscriptionItemPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: UpdateSubscriptionItemPriceDataRecurring,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateSubscriptionItemPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpdateSubscriptionItemPriceData<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        product: &'a str,
        recurring: UpdateSubscriptionItemPriceDataRecurring,
    ) -> Self {
        Self {
            currency,
            product,
            recurring,
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct UpdateSubscriptionItemPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: UpdateSubscriptionItemPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpdateSubscriptionItemPriceDataRecurring {
    pub fn new(interval: UpdateSubscriptionItemPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionItemPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl UpdateSubscriptionItemPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionItemPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionItemPriceDataRecurringInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionItemPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionItemPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionItemPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionItemPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSubscriptionItemPriceDataRecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateSubscriptionItemPriceDataRecurringInterval",
            )
        })
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpdateSubscriptionItemPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionItemPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionItemPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionItemPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionItemPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSubscriptionItemPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateSubscriptionItemPriceDataTaxBehavior")
        })
    }
}
/// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
/// The default value is `create_prorations`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateSubscriptionItemProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}
impl UpdateSubscriptionItemProrationBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateSubscriptionItemProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
        }
    }
}

impl std::str::FromStr for UpdateSubscriptionItemProrationBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateSubscriptionItemProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateSubscriptionItemProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateSubscriptionItemProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateSubscriptionItemProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateSubscriptionItemProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateSubscriptionItemProrationBehavior")
        })
    }
}
/// Updates the plan or quantity of an item on a current subscription.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct UpdateSubscriptionItem<'a> {
    inner: UpdateSubscriptionItemBuilder<'a>,
    item: &'a stripe_shared::SubscriptionItemId,
}
impl<'a> UpdateSubscriptionItem<'a> {
    /// Construct a new `UpdateSubscriptionItem`.
    pub fn new(item: &'a stripe_shared::SubscriptionItemId) -> Self {
        Self { item, inner: UpdateSubscriptionItemBuilder::new() }
    }
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    /// When updating, pass an empty string to remove previously-defined thresholds.
    pub fn billing_thresholds(mut self, billing_thresholds: ItemBillingThresholdsParam) -> Self {
        self.inner.billing_thresholds = Some(billing_thresholds);
        self
    }
    /// The coupons to redeem into discounts for the subscription item.
    pub fn discounts(mut self, discounts: &'a [DiscountsDataParam<'a>]) -> Self {
        self.inner.discounts = Some(discounts);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// Indicates if a customer is on or off-session while an invoice payment is attempted.
    pub fn off_session(mut self, off_session: bool) -> Self {
        self.inner.off_session = Some(off_session);
        self
    }
    /// Use `allow_incomplete` to transition the subscription to `status=past_due` if a payment is required but cannot be paid.
    /// This allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice.
    /// For example, SCA regulation may require 3DS authentication to complete payment.
    /// See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more.
    /// This is the default behavior.
    ///
    /// Use `default_incomplete` to transition the subscription to `status=past_due` when payment is required and await explicit confirmation of the invoice's payment intent.
    /// This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice.
    /// Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method.
    ///
    /// Use `pending_if_incomplete` to update the subscription using [pending updates](https://stripe.com/docs/billing/subscriptions/pending-updates).
    /// When you use `pending_if_incomplete` you can only pass the parameters [supported by pending updates](https://stripe.com/docs/billing/pending-updates-reference#supported-attributes).
    ///
    /// Use `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's invoice cannot be paid.
    /// For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not update the subscription and returns an error instead.
    /// This was the default behavior for API versions prior to 2019-03-14.
    /// See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more.
    pub fn payment_behavior(
        mut self,
        payment_behavior: UpdateSubscriptionItemPaymentBehavior,
    ) -> Self {
        self.inner.payment_behavior = Some(payment_behavior);
        self
    }
    /// The identifier of the new plan for this subscription item.
    pub fn plan(mut self, plan: &'a str) -> Self {
        self.inner.plan = Some(plan);
        self
    }
    /// The ID of the price object.
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    pub fn price(mut self, price: &'a str) -> Self {
        self.inner.price = Some(price);
        self
    }
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    pub fn price_data(mut self, price_data: UpdateSubscriptionItemPriceData<'a>) -> Self {
        self.inner.price_data = Some(price_data);
        self
    }
    /// Determines how to handle [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    /// The default value is `create_prorations`.
    pub fn proration_behavior(
        mut self,
        proration_behavior: UpdateSubscriptionItemProrationBehavior,
    ) -> Self {
        self.inner.proration_behavior = Some(proration_behavior);
        self
    }
    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    /// This can be used to apply the same proration that was previewed with the [upcoming invoice](https://stripe.com/docs/api#retrieve_customer_invoice) endpoint.
    pub fn proration_date(mut self, proration_date: stripe_types::Timestamp) -> Self {
        self.inner.proration_date = Some(proration_date);
        self
    }
    /// The quantity you'd like to apply to the subscription item you're creating.
    pub fn quantity(mut self, quantity: u64) -> Self {
        self.inner.quantity = Some(quantity);
        self
    }
    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    pub fn tax_rates(mut self, tax_rates: &'a [&'a str]) -> Self {
        self.inner.tax_rates = Some(tax_rates);
        self
    }
}
impl UpdateSubscriptionItem<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for UpdateSubscriptionItem<'_> {
    type Output = stripe_shared::SubscriptionItem;

    fn build(&self) -> RequestBuilder {
        let item = self.item;
        RequestBuilder::new(StripeMethod::Post, format!("/subscription_items/{item}"))
            .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ItemBillingThresholdsParam {
    /// Number of units that meets the billing threshold to advance the subscription to a new billing period (e.g., it takes 10 $5 units to meet a $50 [monetary threshold](https://stripe.com/docs/api/subscriptions/update#update_subscription-billing_thresholds-amount_gte)).
    pub usage_gte: i64,
}
impl ItemBillingThresholdsParam {
    pub fn new(usage_gte: i64) -> Self {
        Self { usage_gte }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct DiscountsDataParam<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
    /// ID of the promotion code to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
}
impl<'a> DiscountsDataParam<'a> {
    pub fn new() -> Self {
        Self { coupon: None, discount: None, promotion_code: None }
    }
}
impl<'a> Default for DiscountsDataParam<'a> {
    fn default() -> Self {
        Self::new()
    }
}
