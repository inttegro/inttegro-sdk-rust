use crate::{
    Order, OrderStatus, Payment, PaymentMethod, PaymentNextAction, PaymentStatus, Product,
    PurchaseIntent, PurchaseIntentStatus,
};

impl Payment {
    /// Whether the payment completed successfully.
    pub fn is_paid(&self) -> bool {
        self.status == PaymentStatus::Paid
    }

    /// Whether the payment is waiting for customer or merchant action.
    pub fn requires_action(&self) -> bool {
        self.status == PaymentStatus::RequiresAction
    }

    /// Whether the payment has reached a final state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            PaymentStatus::Paid
                | PaymentStatus::Canceled
                | PaymentStatus::Expired
                | PaymentStatus::Failed
        )
    }

    /// Action details when the payment currently requires action.
    pub fn required_action(&self) -> Option<&PaymentNextAction> {
        self.requires_action()
            .then_some(self.next_action.as_ref())
            .flatten()
    }
}

impl Order {
    /// Whether the order has recorded payment, including a subsequently completed order.
    pub fn is_paid(&self) -> bool {
        self.status == OrderStatus::Paid || self.paid_at.is_some()
    }

    /// Whether the order is waiting for payment.
    pub fn requires_payment(&self) -> bool {
        self.status == OrderStatus::RequiresPayment
    }

    /// Whether the order has reached a final state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            OrderStatus::Paid
                | OrderStatus::Completed
                | OrderStatus::Canceled
                | OrderStatus::Expired
        )
    }

    /// Nested payment action details, when the order's payment requires action.
    pub fn required_payment_action(&self) -> Option<&PaymentNextAction> {
        self.payment.as_ref().and_then(Payment::required_action)
    }
}

impl PurchaseIntent {
    /// Whether the purchase intent is currently active.
    pub fn is_active(&self) -> bool {
        self.status == PurchaseIntentStatus::Active
    }

    /// Whether the purchase intent can create at most one order.
    pub fn is_single_use(&self) -> bool {
        self.usage.single_use == Some(true)
    }

    /// ID of the order that consumed a single-use purchase intent.
    pub fn used_order_id(&self) -> Option<&str> {
        if !self.is_single_use() {
            return None;
        }
        self.usage
            .order
            .as_ref()
            .map(|order| order.id.as_str())
            .filter(|id| !id.is_empty())
    }
}

impl Product {
    /// Whether the product is archived.
    pub fn is_archived(&self) -> bool {
        self.archived_at.is_some()
    }

    /// Whether the product is currently published and available.
    pub fn is_published(&self) -> bool {
        self.active && !self.is_archived()
    }

    /// Whether the product has a recorded first publication.
    pub fn was_ever_published(&self) -> bool {
        self.published_at.is_some()
    }
}

impl PaymentMethod {
    /// Whether the payment method is archived.
    pub fn is_archived(&self) -> bool {
        self.archived_at.is_some()
    }

    /// Whether payment-method ownership has been verified.
    pub fn is_verified(&self) -> bool {
        self.verified_at.is_some()
    }

    /// Whether the payment method may be reused in new payment flows.
    pub fn is_reusable(&self) -> bool {
        self.active && !self.is_archived() && self.ephemeral != Some(true)
    }
}
