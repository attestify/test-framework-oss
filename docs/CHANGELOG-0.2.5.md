# Changelog: 0.2.5

## Unreleased local release candidate

- Adds `kernel_error_has_no_provenance!` for explicit absence assertions on a
  retained Kernel Error Result.
- Adds `kernel_error_provenance_eq!` for exact typed current/origin provenance
  assertions on a retained Kernel Error Result.
- Updates Error assertion macros to use the Kernel Error public accessors.

This local release candidate is not published. Consumers must not depend on it
until a separate release decision and publication occur.
