# Changelog

## 0.6.0

- Added the required application-scoped customer fingerprint to typed customer
  responses for possible duplicate-record detection.

## 0.5.0

- Breaking: replaced payout `balance_transactions` ID strings with typed
  contribution models containing the source transaction's original amount and
  the exact amount allocated to the payout.
- Added complete payment balance-transaction allocation history together with
  available, pending, and spent amount partitions.

## 0.4.0

- Added typed search across customers, financial accounts, orders, payouts, and
  products, including filters, facets, sorting, cursor pagination, totals, and
  freshness metadata.
- Added typed verification purposes for OTP initiation requests.

## 0.3.0

- Breaking: replaced payout maps and generic payloads with named request,
  response, settings, page, error, and destination models.
- Made `ghs` the explicit supported payout-destination field and kept payout
  timestamps behind the strongly typed `Timestamp` value.
- Added fluent resource semantics and removed server-internal purchase-intent
  activity response models.

## 0.2.0

- Breaking: replaced generic maps with named models for balances, purchase intents, products, payment methods, payments, and orders.
- Breaking: exposed API timestamps through the strongly typed `Timestamp` value.

## 0.1.1

- Tightened financial-account and payment-method response models to exclude internal platform fields.

## 0.1.0

- Initial typed server SDK.
