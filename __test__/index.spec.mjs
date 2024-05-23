import test from 'ava'

import { v7 } from '../index.js'

const v7_regex = /^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[0-9a-f]{4}-[0-9a-f]{12}$/

test('v7', (t) => {
  const value = v7();
  t.is(typeof value, 'string')
  t.is(value.length, 36)
  t.regex(value, v7_regex)
});
