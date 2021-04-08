// Apply the paper editing trace to an Automerge.Text object, one char at a time
const { edits, finalText: expected } = require('./editing-trace')

const Automerge1 = require("automerge1")
const AutomergeWASM = require("automerge-wasm")

const MAX = edits.length

benchmark(Automerge1, 'js', 2000, MAX)
benchmark(AutomergeWASM, 'wasm', 2000, MAX)
benchmark(Automerge1, 'js', 100, MAX)
benchmark(AutomergeWASM, 'wasm', 100, MAX)
benchmark(Automerge1, 'js', 1, MAX)
benchmark(AutomergeWASM, 'wasm', 1, MAX)

function benchmark(automerge, mode, step, max) {
  let state = automerge.from({text: new automerge.Text()})
  const start = new Date()
  for (let i = 0; i < max; i += step) {
    //if (i % 1000 === 0) console.log(`Processed ${i} edits in ${new Date() - start} ms`)
    state = automerge.change(state, doc => {
      for (let j = i; j < Math.min(max, i + step); j++) {
        if (edits[j][1] > 0) doc.text.deleteAt(edits[j][0], edits[j][1])
        if (edits[j].length > 2) doc.text.insertAt(edits[j][0], ...edits[j].slice(2))
      }
    })
  }
  const time = new Date() - start;
  const result = state.text.elems.map(e => e.value).join('')
  if (expected !== result) {
    throw new RangeError(`ERROR: final text(len=${result.length}) did not match expectation (text=${expected.length})`)
  }
  console.log(`mode=${mode.padEnd(4)} step=${step.toString().padEnd(4)} time=${(time + 'ms').padEnd(8)}`)
}


