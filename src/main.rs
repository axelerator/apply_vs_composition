use std::{fs::File, io::Write};

use elm_rs::{Elm, ElmDecode, ElmEncode};

#[derive(Elm, ElmEncode)]
enum ToBackend {
    SaveNewDocumentSender { new_sender: Option<String>},
}

fn main() {
    // the target would typically be a file
    let mut target = vec![];
    // elm_rs provides a macro for conveniently creating an Elm module with everything needed
    elm_rs::export!("Bindings", &mut target, {
        // generates types and encoders for types implementing ElmEncoder
        encoders: [ToBackend],
        // generates types and decoders for types implementing ElmDecoder
        decoders: [],
        // generates types and functions for forming queries for types implementing ElmQuery
        queries: [],
        // generates types and functions for forming queries for types implementing ElmQueryField
        query_fields: [],
    })
    .unwrap();
    let output = String::from_utf8(target).unwrap();
    let target_filename = "client/src/Bindings.elm";
    let mut f = File::create(target_filename).unwrap();
    f.write_all(output.as_bytes()).unwrap();
}
