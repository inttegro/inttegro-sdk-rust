#!/usr/bin/env node
// Keep API clients beside the generated models of their public resource module.
// Run after regenerating src/resources.rs and splitting src/generated.rs.
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, unlinkSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const input = join(root, 'src', 'resources.rs');
const outputRoot = join(root, 'src', 'generated');
const header = '//! Typed API resources.\n\n';
const families = {
  Apps: 'app', BalanceTransactions: 'balance_transaction', Balances: 'balance',
  Broadcasts: 'broadcast', Chimes: 'chime', Customers: 'customer',
  FileLinks: 'file_link', FileReferences: 'file', Files: 'file',
  FinancialAccounts: 'financial_account', Keys: 'secret_key',
  MessageTemplates: 'message_template', Orders: 'order', Otp: 'otp',
  PaymentMethods: 'payment_method', Payouts: 'payout', Prices: 'price',
  Products: 'product', PurchaseIntents: 'purchase_intent', Refunds: 'refund',
  Schedules: 'chime', Specifications: 'country', UploadRequests: 'upload_request',
};

function snake(name) {
  return name.replace(/([A-Z]+)([A-Z][a-z])/g, '$1_$2').replace(/([a-z0-9])([A-Z])/g, '$1_$2').toLowerCase();
}

function replaceableFile(path, contents, expectedHeader) {
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
  if (existsSync(input)) throw new Error('src/resources.rs is still monolithic');
  for (const [name, family] of Object.entries(families)) {
    const module = `client_${snake(name)}`;
    const path = join(outputRoot, family, module + '.rs');
    const moduleSource = readFileSync(join(outputRoot, family, 'mod.rs'), 'utf8');
    if (!existsSync(path) || !moduleSource.includes(`mod ${module};\npub use ${module}::*;`)) {
      throw new Error(`Resource client ${name} is not exported by ${family}`);
    }
  }
  console.log('Verified 23 resource-owned Rust API clients');
  process.exit(0);
}

if (!existsSync(input)) throw new Error('Regenerate src/resources.rs before splitting it');
const source = readFileSync(input, 'utf8');
if (!source.startsWith(header)) throw new Error('Unexpected Rust resource header');
const matches = [...source.matchAll(/^pub struct ([A-Za-z][A-Za-z0-9_]*)/gm)];
if (matches.length !== Object.keys(families).length) throw new Error(`Expected 23 clients, found ${matches.length}`);
const starts = matches.map((match) => {
  let start = source.lastIndexOf('\n', match.index - 1) + 1;
  while (start > 0) {
    const end = start - 1;
    const previous = source.lastIndexOf('\n', end - 1) + 1;
    const line = source.slice(previous, end).trim();
    if (line === '' || line.startsWith('///') || line.startsWith('#[')) start = previous;
    else break;
  }
  return start;
});
const preamble = source.slice(header.length, starts[0]);
if (!preamble.includes('use crate::generated::*;') || !preamble.includes('use crate::{')) {
  throw new Error('Unexpected Rust resource imports or unassigned declarations');
}
const outputs = new Map();
const newModules = new Map();
for (let index = 0; index < matches.length; index++) {
  const name = matches[index][1];
  const family = families[name];
  if (!family) throw new Error(`Unclassified resource client ${name}`);
  const module = `client_${snake(name)}`;
  const path = join(outputRoot, family, module + '.rs');
  if (outputs.has(path)) throw new Error(`Duplicate resource client ${path}`);
  const body = source.slice(starts[index], starts[index + 1] ?? source.length).trim();
  const contents = '//! Typed Inttegro API resource operations.\n\n' +
    '#[allow(unused_imports)]\nuse crate::generated::*;\n' +
    '#[allow(unused_imports)]\n' + preamble.match(/use crate::\{[\s\S]*?\};/)[0] + '\n\n' + body + '\n';
  if (!replaceableFile(path, contents, '//! Typed Inttegro API resource operations.')) throw new Error(`Refusing to overwrite modified client ${path}`);
  outputs.set(path, contents);
  if (!newModules.has(family)) newModules.set(family, []);
  newModules.get(family).push(module);
}
for (const [family, modules] of newModules) {
  const path = join(outputRoot, family, 'mod.rs');
  const source = readFileSync(path, 'utf8');
  const additions = modules.sort().filter((module) => !source.includes(`mod ${module};\npub use ${module}::*;`)).map((module) => `mod ${module};\npub use ${module}::*;`).join('\n');
  const contents = additions ? source + additions + '\n' : source;
  if (!replaceableFile(path, contents, '//!')) throw new Error(`Refusing to overwrite modified module ${path}`);
  outputs.set(path, contents);
}
for (const [path, contents] of outputs) {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, contents);
}
unlinkSync(input);
console.log(`Split ${matches.length} Rust API clients into existing public resource modules; run cargo fmt afterward`);
