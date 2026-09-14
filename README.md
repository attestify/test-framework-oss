# test-framework-oss
The open source testing framework for Attestify OSS

## Error provenance assertions

`kernel_error_eq!` remains the exact assertion for an Error's outer `Kind`,
`Audience`, and stable message. Assert provenance separately on the same
retained Result using the public Kernel Error accessors:

```rust,ignore
kernel_error_eq!(&result, Kind::GatewayError, Audience::System, "Unavailable.");
kernel_error_provenance_eq!(
    &result,
    definitions.operation_failed(),
    Some(definitions.dependency_failed()),
);
```

Use `kernel_error_has_no_provenance!(&result)` after `kernel_error_eq!` when
the Error is expected to have no provenance. The provenance macro compares the
complete public descriptors (namespace, schema version, and code); neither
macro parses an Error message or Debug output.

## Release identity

The released Test Framework OSS `0.2.5` distribution is identified solely by
the separately authorized, signed immutable `0.2.5` tag. An untagged checkout
is source and is not a released distribution. Creating or pushing that tag is
outside this finalization authority.
