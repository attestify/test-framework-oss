//! Verifies the public Test Framework OSS Error-provenance assertion macros.
//!
//! These tests use only public Kernel OSS re-exports and public Test Framework
//! macros. The generic definition set models no product-owned descriptor.

use kernel_oss::error::{
    Audience, Error, ErrorProvenanceCode, ErrorProvenanceDescriptor, ErrorProvenanceNamespace,
    ErrorProvenanceSchemaVersion, Kind,
};
use test_framework_oss::{
    kernel_error_eq, kernel_error_has_no_provenance, kernel_error_provenance_eq,
};

const OUTER_MESSAGE: &str = "Dependency is unavailable.";

struct GenericProvenanceDefinitions {
    current: ErrorProvenanceDescriptor,
    derived: ErrorProvenanceDescriptor,
    unexpected: ErrorProvenanceDescriptor,
}

impl GenericProvenanceDefinitions {
    #[allow(clippy::question_mark)]
    fn try_new() -> Result<Self, Error> {
        let namespace_result = ErrorProvenanceNamespace::try_new("example.framework");
        let namespace = match namespace_result {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let version_result = ErrorProvenanceSchemaVersion::try_new(1);
        let version = match version_result {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let current_code_result = ErrorProvenanceCode::try_new("current_fault");
        let current_code = match current_code_result {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let derived_code_result = ErrorProvenanceCode::try_new("derived_fault");
        let derived_code = match derived_code_result {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let unexpected_code_result = ErrorProvenanceCode::try_new("unexpected_fault");
        let unexpected_code = match unexpected_code_result {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        Ok(Self {
            current: ErrorProvenanceDescriptor::new(namespace.clone(), version, current_code),
            derived: ErrorProvenanceDescriptor::new(namespace.clone(), version, derived_code),
            unexpected: ErrorProvenanceDescriptor::new(namespace, version, unexpected_code),
        })
    }

    fn current(&self) -> &ErrorProvenanceDescriptor {
        &self.current
    }

    fn derived(&self) -> &ErrorProvenanceDescriptor {
        &self.derived
    }

    fn unexpected(&self) -> &ErrorProvenanceDescriptor {
        &self.unexpected
    }
}

fn definitions() -> GenericProvenanceDefinitions {
    let definitions_result = GenericProvenanceDefinitions::try_new();
    match definitions_result {
        Ok(value) => value,
        Err(_) => panic!("generic provenance definition construction failed"),
    }
}

fn outer_error() -> Error {
    Error::for_system(Kind::GatewayError, OUTER_MESSAGE)
}

fn attached_error(definitions: &GenericProvenanceDefinitions) -> Result<Error, Error> {
    outer_error().try_with_attached_provenance(definitions.current().clone())
}

#[allow(clippy::question_mark)]
fn derived_error(definitions: &GenericProvenanceDefinitions) -> Result<Error, Error> {
    let attached_result = attached_error(definitions);
    let attached = match attached_result {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    attached.try_with_derived_provenance(definitions.derived().clone())
}

fn error_result(error_result: Result<Error, Error>, expected: &str) -> Result<(), Error> {
    match error_result {
        Ok(error) => Err(error),
        Err(_) => panic!("{expected}"),
    }
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Public seam: `kernel_error_eq!` and `kernel_error_has_no_provenance!`.
/// Logical path: S2A-T01 outer Error has no provenance.
/// Observable result: both exact outer and absence assertions succeed.
/// Excluded behavior: no message or Debug parsing occurs.
#[test]
fn outer_error_without_provenance_assertions_success() {
    let result: Result<(), Error> = Err(outer_error());
    kernel_error_eq!(&result, Kind::GatewayError, Audience::System, OUTER_MESSAGE);
    kernel_error_has_no_provenance!(&result);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Public seam: `kernel_error_eq!` and `kernel_error_provenance_eq!`.
/// Logical path: S2A-T02 attached current descriptor has no origin.
/// Observable result: separate outer and typed provenance assertions succeed.
/// Excluded behavior: no raw descriptor literal is attached at the call site.
#[test]
fn attached_current_descriptor_without_origin_assertions_success() {
    let definitions = definitions();
    let result = error_result(
        attached_error(&definitions),
        "an attached Error result was expected",
    );
    kernel_error_eq!(&result, Kind::GatewayError, Audience::System, OUTER_MESSAGE);
    kernel_error_provenance_eq!(&result, definitions.current(), None);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Public seam: `kernel_error_eq!` and `kernel_error_provenance_eq!`.
/// Logical path: S2A-T03 derived descriptor retains an origin descriptor.
/// Observable result: complete current and origin descriptors both match.
/// Excluded behavior: descriptor identity is not inferred from Error text.
#[test]
fn derived_current_and_origin_descriptor_assertions_success() {
    let definitions = definitions();
    let result = error_result(
        derived_error(&definitions),
        "a derived Error result was expected",
    );
    kernel_error_eq!(&result, Kind::GatewayError, Audience::System, OUTER_MESSAGE);
    kernel_error_provenance_eq!(&result, definitions.derived(), Some(definitions.current()),);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Public seam: `kernel_error_has_no_provenance!`.
/// Logical path: S2A-T04 absence assertion receives present provenance.
/// Observable result: the macro reports its fixed absence diagnostic.
/// Excluded behavior: no Kernel Error message is inspected.
#[test]
#[should_panic(
    expected = "kernel error provenance assertion expected no provenance, but provenance was present."
)]
fn absence_assertion_against_present_provenance_error() {
    let definitions = definitions();
    let result = error_result(
        attached_error(&definitions),
        "an attached Error result was expected",
    );
    kernel_error_has_no_provenance!(&result);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Public seam: `kernel_error_provenance_eq!`.
/// Logical path: S2A-T05 expected current descriptor differs from actual.
/// Observable result: the macro reports its fixed wrong-current diagnostic.
/// Excluded behavior: no raw code or namespace comparison is performed.
#[test]
#[should_panic(expected = "kernel error provenance assertion current descriptor does not match.")]
fn provenance_assertion_with_wrong_current_descriptor_error() {
    let definitions = definitions();
    let result = error_result(
        attached_error(&definitions),
        "an attached Error result was expected",
    );
    kernel_error_provenance_eq!(&result, definitions.unexpected(), None);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Public seam: `kernel_error_provenance_eq!`.
/// Logical path: S2A-T06 expected origin descriptor differs from actual.
/// Observable result: the macro reports its fixed wrong-origin diagnostic.
/// Excluded behavior: no Error message or Debug output is parsed.
#[test]
#[should_panic(expected = "kernel error provenance assertion origin descriptor does not match.")]
fn provenance_assertion_with_wrong_origin_descriptor_error() {
    let definitions = definitions();
    let result = error_result(
        derived_error(&definitions),
        "a derived Error result was expected",
    );
    kernel_error_provenance_eq!(
        &result,
        definitions.derived(),
        Some(definitions.unexpected()),
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Public seam: `kernel_error_eq!`.
/// Logical path: S2A-T07 exact outer Error kind differs despite valid provenance.
/// Observable result: the outer assertion reports its fixed kind diagnostic.
/// Excluded behavior: provenance does not weaken the outer Error assertion.
#[test]
#[should_panic(expected = "Kind does not match.")]
fn retained_outer_error_mismatch_error() {
    let definitions = definitions();
    let result = error_result(
        attached_error(&definitions),
        "an attached Error result was expected",
    );
    kernel_error_provenance_eq!(&result, definitions.current(), None);
    kernel_error_eq!(&result, Kind::InvalidInput, Audience::System, OUTER_MESSAGE);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Public seam: all three public Test Framework OSS Error assertion macros.
/// Logical path: S2A-T08 public ABI accessors support absence, current, and origin.
/// Observable result: all three macros compile and execute without Error field access.
/// Excluded behavior: no private Kernel API is imported.
#[test]
fn public_abi_accessor_compatibility_success() {
    let definitions = definitions();
    let none: Result<(), Error> = Err(outer_error());
    let attached = error_result(
        attached_error(&definitions),
        "an attached Error result was expected",
    );
    let derived = error_result(
        derived_error(&definitions),
        "a derived Error result was expected",
    );

    kernel_error_eq!(&none, Kind::GatewayError, Audience::System, OUTER_MESSAGE);
    kernel_error_has_no_provenance!(&none);
    kernel_error_provenance_eq!(&attached, definitions.current(), None);
    kernel_error_provenance_eq!(&derived, definitions.derived(), Some(definitions.current()),);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Public seam: `Error::try_with_attached_provenance`, `kernel_error_eq!`, and
/// `kernel_error_has_no_provenance!` / `kernel_error_provenance_eq!`.
/// Logical path: S2A-T11 a successful attach reconstructs rather than mutates.
/// Observable result: the returned Error has current provenance while the
/// retained source has the same exact outer Error contract and no provenance.
/// Excluded behavior: no raw descriptor literal or private Kernel API is used.
#[test]
fn successful_attach_preserves_source_error_success() {
    let definitions = definitions();
    let source = outer_error();
    let attach_result = source.try_with_attached_provenance(definitions.current().clone());
    let attached = match &attach_result {
        Ok(error) => error.clone(),
        Err(_) => panic!("attached Error was expected"),
    };
    let source_result: Result<(), Error> = Err(source.clone());
    let attached_result: Result<(), Error> = Err(attached);

    kernel_error_eq!(
        &source_result,
        Kind::GatewayError,
        Audience::System,
        OUTER_MESSAGE,
    );
    kernel_error_has_no_provenance!(&source_result);
    kernel_error_eq!(
        &attached_result,
        Kind::GatewayError,
        Audience::System,
        OUTER_MESSAGE,
    );
    kernel_error_provenance_eq!(&attached_result, definitions.current(), None);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Public seam: `Error::try_with_derived_provenance` and the public Error
/// assertion macros.
/// Logical path: S2A-T12 a successful derive reconstructs rather than mutates.
/// Observable result: the returned Error has current plus origin provenance,
/// while the retained attached source retains current provenance and no origin.
/// Excluded behavior: no Error text or Debug value determines provenance.
#[test]
fn successful_derive_preserves_source_error_success() {
    let definitions = definitions();
    let attach_result = outer_error().try_with_attached_provenance(definitions.current().clone());
    let source = match &attach_result {
        Ok(error) => error.clone(),
        Err(_) => panic!("attached Error was expected"),
    };
    let derive_result = source.try_with_derived_provenance(definitions.derived().clone());
    let derived = match &derive_result {
        Ok(error) => error.clone(),
        Err(_) => panic!("derived Error was expected"),
    };
    let source_result: Result<(), Error> = Err(source.clone());
    let derived_result: Result<(), Error> = Err(derived);

    kernel_error_eq!(
        &source_result,
        Kind::GatewayError,
        Audience::System,
        OUTER_MESSAGE,
    );
    kernel_error_provenance_eq!(&source_result, definitions.current(), None);
    kernel_error_eq!(
        &derived_result,
        Kind::GatewayError,
        Audience::System,
        OUTER_MESSAGE,
    );
    kernel_error_provenance_eq!(
        &derived_result,
        definitions.derived(),
        Some(definitions.current()),
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Public seam: `Error::try_with_attached_provenance` and the public Error
/// assertion macros.
/// Logical path: S2A-T13 a second attach is rejected without mutating source.
/// Observable result: the exact Kernel transition Error is returned and the
/// retained source remains current-only.
/// Excluded behavior: no Error message is parsed to identify provenance.
#[test]
fn rejected_second_attach_preserves_source_error_error() {
    let definitions = definitions();
    let attach_result = outer_error().try_with_attached_provenance(definitions.current().clone());
    let source = match &attach_result {
        Ok(error) => error.clone(),
        Err(_) => panic!("attached Error was expected"),
    };
    let rejected_result = source.try_with_attached_provenance(definitions.unexpected().clone());
    let transition_result: Result<(), Error> = match rejected_result {
        Ok(_) => panic!("second attach rejection was expected"),
        Err(error) => Err(error),
    };
    let source_result: Result<(), Error> = Err(source.clone());

    kernel_error_eq!(
        &transition_result,
        Kind::ProcessingFailure,
        Audience::System,
        "Error provenance cannot be attached because it is already present.",
    );
    kernel_error_has_no_provenance!(&transition_result);
    kernel_error_eq!(
        &source_result,
        Kind::GatewayError,
        Audience::System,
        OUTER_MESSAGE,
    );
    kernel_error_provenance_eq!(&source_result, definitions.current(), None);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Public seam: `Error::try_with_derived_provenance` and the public Error
/// assertion macros.
/// Logical path: S2A-T14 derivation without provenance is rejected unchanged.
/// Observable result: the exact Kernel transition Error is returned and the
/// retained source keeps the same exact outer contract with no provenance.
/// Excluded behavior: no private Error state is inspected.
#[test]
fn rejected_absent_derive_preserves_source_error_error() {
    let definitions = definitions();
    let source = outer_error();
    let rejected_result = source.try_with_derived_provenance(definitions.derived().clone());
    let transition_result: Result<(), Error> = match rejected_result {
        Ok(_) => panic!("absent provenance derivation rejection was expected"),
        Err(error) => Err(error),
    };
    let source_result: Result<(), Error> = Err(source.clone());

    kernel_error_eq!(
        &transition_result,
        Kind::ProcessingFailure,
        Audience::System,
        "Error provenance cannot be derived because it is absent.",
    );
    kernel_error_has_no_provenance!(&transition_result);
    kernel_error_eq!(
        &source_result,
        Kind::GatewayError,
        Audience::System,
        OUTER_MESSAGE,
    );
    kernel_error_has_no_provenance!(&source_result);
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Public seam: `Error::try_with_derived_provenance` and the public Error
/// assertion macros.
/// Logical path: S2A-T15 a second derive is rejected without mutating source.
/// Observable result: the exact Kernel transition Error is returned and the
/// retained source preserves its complete current-plus-origin provenance.
/// Excluded behavior: no raw descriptor comparison or message parsing occurs.
#[test]
fn rejected_second_derive_preserves_source_error_error() {
    let definitions = definitions();
    let attach_result = outer_error().try_with_attached_provenance(definitions.current().clone());
    let attached = match &attach_result {
        Ok(error) => error.clone(),
        Err(_) => panic!("attached Error was expected"),
    };
    let derive_result = attached.try_with_derived_provenance(definitions.derived().clone());
    let source = match &derive_result {
        Ok(error) => error.clone(),
        Err(_) => panic!("derived Error was expected"),
    };
    let rejected_result = source.try_with_derived_provenance(definitions.unexpected().clone());
    let transition_result: Result<(), Error> = match rejected_result {
        Ok(_) => panic!("second derivation rejection was expected"),
        Err(error) => Err(error),
    };
    let source_result: Result<(), Error> = Err(source.clone());

    kernel_error_eq!(
        &transition_result,
        Kind::ProcessingFailure,
        Audience::System,
        "Error provenance cannot be derived because an origin is already present.",
    );
    kernel_error_has_no_provenance!(&transition_result);
    kernel_error_eq!(
        &source_result,
        Kind::GatewayError,
        Audience::System,
        OUTER_MESSAGE,
    );
    kernel_error_provenance_eq!(
        &source_result,
        definitions.derived(),
        Some(definitions.current()),
    );
}
