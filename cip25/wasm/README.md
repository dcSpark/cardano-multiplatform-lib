# cml-cip25

Multiplatform SDK for CIP25 (Cardano NFT Metadata)

This can be used for both parsing and creating CIP25-compatible metadata.

Parsing can either be done directly from metadata bytes:

```javascript
let schema = wasm.CIP25.CIP25Metadata.from_bytes(Buffer.from("a11902d1a26464617461a144baadf00da144cafed00da5646e616d656d4d65746164617461204e616d656566696c657382a3637372636473726331646e616d656966696c656e616d6531696d65646961547970656966696c657479706531a3637372636473726332646e616d656966696c656e616d6532696d65646961547970656966696c65747970653265696d6167657821687474733a2f2f736f6d652e776562736974652e636f6d2f696d6167652e706e67696d656469615479706567696d6167652f2a6b6465736372697074696f6e776465736372697074696f6e206f662074686973204e46546776657273696f6e02", "hex"));

// the above CBOR hex is for a V2 CIP25 schema. You should check which type it is first
// as as_label_metadata_v2() will return None (undefined) if it's a v1 schema
let v2Schema = schema.key_721().as_label_metadata_v2().data();
let policies = v2Schema.keys();
for (var i = 0; i < policies.len(); ++i) {
  let policy = policies.get(i);
  let assets = v2Schema.get(policy);
  let assetNames = assets.keys();
  for (var j = 0; j < assetNames.len(); ++j) {
    let assetName = assetNames.get(j);
    let details = assets.get(assetName);
    console.log(`CIP25 NFT ${policy.toString("hex")}.${asset_name.toString("hex")} found:`);
    console.log(`  name: ${details.name().to_str()}`);
    console.log(`  image: ${details.image().to_string()}`);
    let mediaType = details.media_type();
    if (mediaType != null) {
      console.log(`  media_type: ${mediaType.to_str()}`);
    }
    let description = details.media_type();
    if (description != null) {
      console.log(`  description: ${description.to_str()}`);
    }
    let files = details.files();
    if (files != null) {
      console.log(`  files:`);
      for (var k = 0; k < files.len(); ++k) {
        let file = files.get(k);
        console.log(`    file #${k + 1}:`);
        console.log(`      name: ${file.name().to_str()}`);
        console.log(`      media_type: ${file.media_type().to_str()}`);
        console.log(`      src: ${file.src().to_string()}`);
      }
    }
  }
}
```

We also support loose NFT parsing to try and parse key information out of potentially incorrectly formatted CIP25

```typescript
const details = wasm.CIP25.MiniMetadataDetails.loose_parse(Buffer.from("a1646e616d65694d6574617665727365", "hex"));
console.log(details.name());
```