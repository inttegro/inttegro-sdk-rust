#!/usr/bin/env node
// Split the canonical generator's flat Rust output into public resource modules.
// Run after regenerating src/generated.rs.
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, readdirSync, unlinkSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const sourcePath = join(root, 'src', 'generated.rs');
const outputRoot = join(root, 'src', 'generated');
const sourceHeader = '//! Generated, typed Inttegro domain and request values.\n// This file is generated from the canonical SDK contract. Do not edit manually.\n';
const modelHeader = '//! Generated, typed Inttegro domain or request value. Do not edit manually.\n\n';
const families = [
  ['purchase_intent', ['PurchaseIntent']],
  ['payment_method', ['PaymentMethod']],
  ['balance_transaction', ['BalanceTransaction']],
  ['message_template', ['MessageTemplate']],
  ['financial_account', ['FinancialAccount']],
  ['upload_request', ['UploadRequest', 'UploadReview', 'ReviewUpload']],
  ['file_link', ['FileLink']],
  ['secret_key', ['SecretKey']],
  ['bank_account', ['BankAccount']],
  ['country', ['Country']],
  ['checkout', ['Checkout']],
  ['broadcast', ['Broadcast']],
  ['customer', ['Customer']],
  ['payment', ['Payment']],
  ['payout', ['Payout']],
  ['product', ['Product']],
  ['refund', ['Refund']],
  ['order', ['Order']],
  ['chime', ['Chime']],
  ['balance', ['Balance']],
  ['wallet', ['Wallet']],
  ['price', ['Price']],
  ['file', ['File']],
  ['otp', ['OTP']],
  ['app', ['Application', 'App']],
];
const verbs = /^(?:Activate|Add|Archive|Cancel|Collect|Confirm|Connect|Create|Delete|Disable|Disactivate|Disconnect|Enable|Fulfill|Get|Lookup|Open|Page|Reconnect|Render|Request|Revoke|Review|Send|Set|Submit|Tokenize|Unarchive|Update|Verify)/;
const overrides = {
  AddressInput: 'shared',
  Amount: 'money', AmountParams: 'money', Currency: 'money',
  BalanceValue: 'balance', BillingDetailsInput: 'checkout',
  ContentSafetyStatus: 'shared', DeliveryChannel: 'shared',
  Error: 'shared', InvoiceSettings: 'checkout', InvoiceSettingsInput: 'checkout',
  LineItemInput: 'order', LineItemType: 'order',
  MobileMoneyNetwork: 'wallet', ResourceSupply: 'product',
  ShippingDetailsInput: 'checkout', ShippingInput: 'order', ShippingLineItemInput: 'order',
};

function familyFor(name) {
  if (overrides[name]) return overrides[name];
  let stem = name;
  while (verbs.test(stem)) stem = stem.replace(verbs, '');
  for (const [family, prefixes] of families) {
    if (prefixes.some((prefix) => stem.startsWith(prefix))) return family;
  }
  if (stem.includes('PaymentMethod')) return 'payment_method';
  if (stem.includes('MessageTemplate')) return 'message_template';
  if (stem.includes('SecretKey')) return 'secret_key';
  if (stem.includes('Payout')) return 'payout';
  if (stem.includes('Upload')) return 'upload_request';
  if (stem.includes('Order')) return 'order';
  if (stem.includes('Product')) return 'product';
  if (stem.includes('OTP')) return 'otp';
  if (stem.includes('BankAccount')) return 'bank_account';
  if (stem.includes('File')) return 'file';
  if (stem.includes('Price')) return 'price';
  if (stem.includes('Country')) return 'country';
  if (stem.startsWith('CurrencyBalance')) return 'balance';
  if (stem.startsWith('FinancialInstitution')) return 'financial_account';
  if (/^(?:Fee|LineItem)/.test(stem)) return 'order';
  if (/^(?:Invoice|Billing)/.test(stem)) return 'checkout';
  if (/^Schedule/.test(stem)) return 'chime';
  if (stem === 'ConfirmationRequest') return 'order';
  return 'shared';
}

function fileName(name) {
  return name
    .replace(/IDs\b/g, 'Ids')
    .replace(/([A-Z]+)([A-Z][a-z])/g, '$1_$2')
    .replace(/([a-z0-9])([A-Z])/g, '$1_$2')
    .toLowerCase() + '.rs';
}

function declarationStart(source, offset) {
  let start = source.lastIndexOf('\n', offset - 1) + 1;
  while (start > 0) {
    const previousEnd = start - 1;
    const previousStart = source.lastIndexOf('\n', previousEnd - 1) + 1;
    const line = source.slice(previousStart, previousEnd).trim();
    if (line === '' || line.startsWith('///') || line.startsWith('#[')) {
      start = previousStart;
    } else {
      break;
    }
  }
  return start;
}

function replaceableGeneratedFile(path, contents, expectedHeader) {
  if (!existsSync(path) || readFileSync(path, 'utf8') === contents) return true;
  if (!readFileSync(path, 'utf8').startsWith(expectedHeader)) return false;
  const relativePath = path.slice(root.length + 1);
  try {
    execFileSync('git', ['ls-files', '--error-unmatch', '--', relativePath], { cwd: root, stdio: 'ignore' });
    execFileSync('git', ['diff', '--quiet', '--', relativePath], { cwd: root, stdio: 'ignore' });
    execFileSync('git', ['diff', '--cached', '--quiet', '--', relativePath], { cwd: root, stdio: 'ignore' });
    return true;
  } catch {
    return false;
  }
}

if (process.argv.includes('--check')) {
  if (existsSync(sourcePath)) throw new Error('src/generated.rs is still a monolith');
  const familyDirs = readdirSync(outputRoot, { withFileTypes: true }).filter((entry) => entry.isDirectory());
  const files = familyDirs.flatMap((family) => readdirSync(join(outputRoot, family.name)).filter((name) => name.endsWith('.rs') && name !== 'mod.rs' && !name.startsWith('client_')));
  if (files.length < 400) throw new Error(`Expected at least 400 split models, found ${files.length}`);
  console.log(`Verified ${files.length} resource-owned Rust model files`);
  process.exit(0);
}

if (!existsSync(sourcePath)) throw new Error('Regenerate src/generated.rs before splitting it');
const source = readFileSync(sourcePath, 'utf8');
if (!source.startsWith(sourceHeader)) throw new Error('Unexpected generated Rust header; refusing to split');
const matches = [...source.matchAll(/^pub (?:struct|enum|type) ([A-Za-z][A-Za-z0-9_]*)/gm)];
if (matches.length < 400) throw new Error(`Expected at least 400 Rust declarations, found ${matches.length}`);
const starts = matches.map((match) => declarationStart(source, match.index));
if (starts.some((start, index) => index > 0 && start <= starts[index - 1])) {
  throw new Error('Overlapping declaration ranges');
}
const preamble = source.slice(sourceHeader.length, starts[0]);
if (!/^\s*use crate::\{[\s\S]*?\};\s*use serde::\{Deserialize, Serialize\};\s*$/.test(preamble)) {
  throw new Error('Unexpected Rust imports or unassigned declarations');
}
const externalNames = [...preamble.matchAll(/\b[A-Z][A-Za-z0-9]*\b/g)].map((match) => match[0]);
const names = new Set([...matches.map((match) => match[1]), ...externalNames]);
names.delete('Deserialize'); names.delete('Serialize');
const outputs = new Map();
const familyModules = new Map();

for (let index = 0; index < matches.length; index++) {
  const name = matches[index][1];
  const body = source.slice(starts[index], starts[index + 1] ?? source.length).trim();
  const family = familyFor(name);
  const modelName = fileName(name).slice(0, -3);
  // `mod payment` inside `payment` triggers Clippy's module-inception lint.
  const moduleName = modelName === family ? 'model' : modelName;
  const path = join(outputRoot, family, moduleName + '.rs');
  if (outputs.has(path)) throw new Error(`Duplicate generated file ${path}`);
  if (!familyModules.has(family)) familyModules.set(family, []);
  familyModules.get(family).push(moduleName);
  const implementation = body
    .replace(/^\s*\/\/.*$/gm, '')
    .replace(/"(?:\\.|[^"\\])*"/g, '');
  const dependencies = [...names].filter((dependency) => dependency !== name && new RegExp(`\\b${dependency}\\b`).test(implementation)).sort();
  const crateImports = dependencies.length === 1
    ? `use crate::${dependencies[0]};\n`
    : dependencies.length > 1 ? `use crate::{${dependencies.join(', ')}};\n` : '';
  const imports = [crateImports, body.includes('Serialize') || body.includes('Deserialize') ? 'use serde::{Deserialize, Serialize};\n' : ''].join('');
  const contents = modelHeader + imports + (imports ? '\n' : '') + body + '\n';
  if (!replaceableGeneratedFile(path, contents, modelHeader)) throw new Error(`Refusing to overwrite modified model ${path}`);
  outputs.set(path, contents);
}

for (const [family, modules] of familyModules) {
  const path = join(outputRoot, family, 'mod.rs');
  const clients = existsSync(dirname(path))
    ? readdirSync(dirname(path)).filter((name) => name.startsWith('client_') && name.endsWith('.rs')).map((name) => name.slice(0, -3))
    : [];
  const contents = `//! ${family.replaceAll('_', ' ')} API models and domain values.\n\n` + [...modules, ...clients].sort().map((module) => `mod ${module};\npub use ${module}::*;`).join('\n') + '\n';
  if (!replaceableGeneratedFile(path, contents, '//!')) throw new Error(`Refusing to overwrite modified module ${path}`);
  outputs.set(path, contents);
}
const rootModule = join(outputRoot, 'mod.rs');
const rootContents = '//! Generated resource-owned Inttegro API values.\n\n' + [...familyModules.keys()].sort().map((family) => `pub mod ${family};\npub use ${family}::*;`).join('\n') + '\n';
if (!replaceableGeneratedFile(rootModule, rootContents, '//!')) throw new Error(`Refusing to overwrite modified module ${rootModule}`);
outputs.set(rootModule, rootContents);

for (const [path, contents] of outputs) {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, contents);
}
unlinkSync(sourcePath);
console.log(`Split ${matches.length} Rust declarations into ${familyModules.size} public resource modules; run cargo fmt afterward`);
