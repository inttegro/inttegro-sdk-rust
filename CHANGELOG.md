# Changelog

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
