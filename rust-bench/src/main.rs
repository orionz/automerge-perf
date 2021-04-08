use automerge_frontend::{Frontend, InvalidChangeRequest, LocalChange, Path, Value};
use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::time::Instant;

#[derive(Deserialize)]
struct EditTrace {
    edits: Vec<Edit>,
    #[serde(rename(deserialize = "finalText"))]
    final_text: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Edit {
    // idx to insert at, # of chars to delete (always 0), char to insert
    Insert((u32, u32, char)),
    // idx to delete at, # of chars to delete
    Delete((u32, u32)),
}

fn benchmark(edits: &[Edit], step: usize, expected: &str) {
    // Trying to set initial state with JSON breaks when we later try to insert a char (bug?)
    //let initial_state_json: serde_json::Value = serde_json::from_str(r#"{"text": ""}"#).unwrap();
    //let value = Value::from_json(&initial_state_json);
    let mut frontend = Frontend::new();
    frontend
        .change::<_, InvalidChangeRequest>(None, |doc| {
            doc.add_change(LocalChange::set(
                Path::root().key("text"),
                Value::Text(Vec::new()),
            ))?;
            Ok(())
        })
        .unwrap();

    let chunks: Vec<_> = edits.chunks(step).collect();
    //let total_chunks = chunks.len();

    let start = Instant::now();
    for (_i, chunk) in chunks.into_iter().enumerate() {
        //println!("Finished chunk: {}/{}", i, total_chunks);
        frontend
            .change::<_, InvalidChangeRequest>(None, |doc| {
                for edit in chunk {
                    match edit {
                        Edit::Insert((insert_idx, _, to_insert)) => {
                            doc.add_change(LocalChange::insert(
                                Path::root().key("text").index(*insert_idx),
                                to_insert.to_string().as_str().into(),
                            ))?;
                        }
                        Edit::Delete((delete_idx, n_chars_to_delete)) => {
                            for i in 0..*n_chars_to_delete {
                                doc.add_change(LocalChange::delete(
                                    Path::root().key("text").index(*delete_idx + i),
                                ))?;
                            }
                        }
                    };
                }
                Ok(())
            })
            .unwrap();
    }
    let elapsed = start.elapsed().as_millis();
    let final_state = frontend.state().to_json();
    let final_text = final_state
        .as_object()
        .unwrap()
        .get("text")
        .unwrap()
        .as_str()
        .unwrap();
    if final_text != expected {
        panic!(
            "Final text (len={}) was not equal to expected text (len={})",
            final_text.len(),
            expected.len()
        );
    }
    println!("mode=rust step={:<4} time={}ms", step, elapsed,);
}

fn main() {
    let f = File::open("../editing-trace.json").unwrap();
    //let f = File::open("../editing-trace-small.json").unwrap();
    let edits: EditTrace = serde_json::from_reader(f).unwrap();
    benchmark(&edits.edits, 2000, &edits.final_text);
    benchmark(&edits.edits, 100, &edits.final_text);
    benchmark(&edits.edits, 1, &edits.final_text);
}
