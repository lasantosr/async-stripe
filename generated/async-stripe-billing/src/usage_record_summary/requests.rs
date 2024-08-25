use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ListSubscriptionItemUsageRecordSummaryBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListSubscriptionItemUsageRecordSummaryBuilder<'a> {
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
pub struct ListSubscriptionItemUsageRecordSummary<'a> {
    inner: ListSubscriptionItemUsageRecordSummaryBuilder<'a>,
    subscription_item: &'a stripe_shared::SubscriptionItemId,
}
impl<'a> ListSubscriptionItemUsageRecordSummary<'a> {
    /// Construct a new `ListSubscriptionItemUsageRecordSummary`.
    pub fn new(subscription_item: &'a stripe_shared::SubscriptionItemId) -> Self {
        Self { subscription_item, inner: ListSubscriptionItemUsageRecordSummaryBuilder::new() }
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
impl ListSubscriptionItemUsageRecordSummary<'_> {
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

impl StripeRequest for ListSubscriptionItemUsageRecordSummary<'_> {
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
