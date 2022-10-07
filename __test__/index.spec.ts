import test from 'ava'

// import { plus100 } from '../index'

// test('sync function from native code', (t) => {
//   const fixture = 42
//   t.is(plus100(fixture), fixture + 100)
// })

import { clickInput } from '../index.js'

test('clickInput  native', async (t) => {
  //   t.is(sum(1, 2), 3)
  await clickInput(1100, 200, 'ffss')
  t.true(true)
})
