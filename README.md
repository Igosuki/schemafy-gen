# schemafy-gen

[![Build Status](https://travis-ci.org/Igosuki/schemafy-gen.svg?branch=master)](https://travis-ci.org/Igosuki/schemafy-gen)
[![Docs](https://docs.rs/schemafy-gen/badge.svg)](https://docs.rs/schemafy-gen)

This is a Rust crate which can take a [JSON schema (draft 4)](http://json-schema.org/) and generate Rust types which are serializable with [serde](https://serde.rs/). No checking such as `min_value` are done but instead only the structure of the schema is followed as closely as possible.

## Requirements

```
serde = { version = "1.0", features = ["derive", "std"] }
serde_json = "1.0"
serde_derive = "1.0"
smart-default = "0.6.0"
```

## Features 

- Handles default values with the SmartDefault crate 
- Outputs structs in nice namespaces

## Example

```
run --package schemafy-gen --bin main -- -s src/schema.json -o src/schema_gen.rs
```

## Development


