// This file was code-generated using an experimental CDDL to rust tool:
// https://github.com/dcSpark/cddl-codegen

// Compiled self-check for the dep-side extern-interface export
// (`extern-interface/<dep>/**`). Machine-generated from the SAME projection as that
// export, so the two cannot drift. Every exported name is asserted to be a real,
// correctly-typed surface in THIS crate: opaque rows implement `Serialize` (and
// `Deserialize` where the dep generates one), raw-bytes rows `RawBytesEncoding`, and
// transparent rows (aliases, c-style enums, named collections) must simply exist. A
// hand-edited or stale export — or a projection bug — therefore fails THIS crate's own
// build, naming the type. Do not edit.
// Rows carry NO per-row comments by design: a spec change can delete any row, and a
// comment stranded on a deleted row is what the edit-preservation overlay turns into a
// build-breaking sentinel on the next regen. All commentary lives in this fixed banner;
// each row's type path is its own traceability.
#[allow(dead_code)]
fn _assert_serialize<T: cbor_event::se::Serialize>() {}
#[allow(dead_code)]
fn _assert_deserialize<T: cml_core::serialization::Deserialize>() {}
#[allow(dead_code)]
fn _extern_interface_self_check() {
    _assert_serialize::<crate::generated::CIP25ChunkableString>();
    _assert_deserialize::<crate::generated::CIP25ChunkableString>();
    _assert_serialize::<crate::generated::CIP25FilesDetails>();
    _assert_deserialize::<crate::generated::CIP25FilesDetails>();
    _assert_serialize::<crate::generated::CIP25LabelMetadata>();
    _assert_deserialize::<crate::generated::CIP25LabelMetadata>();
    _assert_serialize::<crate::generated::CIP25Metadata>();
    _assert_deserialize::<crate::generated::CIP25Metadata>();
    _assert_serialize::<crate::generated::CIP25MetadataDetails>();
    _assert_deserialize::<crate::generated::CIP25MetadataDetails>();
    _assert_serialize::<crate::generated::CIP25String64>();
    _assert_deserialize::<crate::generated::CIP25String64>();
}
