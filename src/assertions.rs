/// Asserts that an [`nape_kernel::error::Error`] matches the expected kind, audience, and message.
///
/// # Arguments
///
/// * `$expression` - A `Result` expression that is expected to be an [`nape_kernel::error::Error`].
/// * `$expected_kind` - The expected error kind. Should be of type [`nape_kernel::error::Kind`].
/// * `$expected_audience` - The expected error audience. Should be of type [`nape_kernel::error::Audience`].
/// * `$expected_message` - The expected error message. Should be a [`String`].
///
#[macro_export]
macro_rules! kernel_error_eq {
    ($result:expr, $expected_kind:expr, $expected_audience:expr, $expected_message:expr) => {
        match $result {
            Ok(val) => panic!(
                "An Error was expected, although one was not returned:\n\t{:?}",
                val
            ),
            Err(e) => {
                assert_eq!(
                    e.kind(),
                    $expected_kind,
                    "{}",
                    format!(
                        "Kind does not match.\n\tExpected: {:?},\n\tActual: {:?}\n",
                        $expected_kind,
                        e.kind()
                    )
                );
                assert_eq!(
                    e.audience(),
                    $expected_audience,
                    "{}",
                    format!(
                        "Audience does not match.\n\tExpected: {:?}\n\tActual: {:?}\n",
                        $expected_audience,
                        e.audience()
                    )
                );
                if e.message() != $expected_message {
                    panic!(
                        "The Error Message does not match.\n\tExpected:\t{:?},\n Actual:\t{:?}\n",
                        $expected_message,
                        e.message()
                    );
                }
            }
        }
    };
}

/// Asserts that a retained Kernel Error result has no provenance.
///
/// Use this after [`kernel_error_eq!`] when the outer Error contract is also
/// part of the logical path. This macro uses only the public
/// `Error::provenance()` accessor and does not inspect Error messages or Debug
/// output.
///
/// # Arguments
///
/// * `$result` - A retained `Result` expression expected to contain an Error.
///
/// # Examples
///
/// ```rust
/// use kernel_oss::error::{Audience, Error, Kind};
/// use test_framework_oss::{kernel_error_eq, kernel_error_has_no_provenance};
///
/// let result: Result<(), Error> = Err(Error::for_system(Kind::GatewayError, "Unavailable."));
/// kernel_error_eq!(&result, Kind::GatewayError, Audience::System, "Unavailable.");
/// kernel_error_has_no_provenance!(&result);
/// ```
#[macro_export]
macro_rules! kernel_error_has_no_provenance {
    ($result:expr) => {
        match $result {
            Ok(_) => panic!(
                "kernel error provenance assertion expected Err(Error), received Ok."
            ),
            Err(error) => match error.provenance() {
                None => {},
                Some(_) => panic!(
                    "kernel error provenance assertion expected no provenance, but provenance was present."
                ),
            },
        }
    };
}

/// Asserts that a retained Kernel Error result has the exact current and origin
/// provenance descriptors.
///
/// `$expected_current` must be an `&ErrorProvenanceDescriptor` and
/// `$expected_origin` must be an `Option<&ErrorProvenanceDescriptor>`. The
/// macro compares complete typed descriptors through public accessors; it does
/// not accept raw namespace or code strings and does not inspect Error messages
/// or Debug output.
///
/// # Arguments
///
/// * `$result` - A retained `Result` expression expected to contain an Error.
/// * `$expected_current` - The expected complete current descriptor.
/// * `$expected_origin` - The expected complete optional origin descriptor.
///
/// # Examples
///
/// ```rust
/// use kernel_oss::error::{
///     Audience, Error, ErrorProvenanceCode, ErrorProvenanceDescriptor,
///     ErrorProvenanceNamespace, ErrorProvenanceSchemaVersion, Kind,
/// };
/// use test_framework_oss::{kernel_error_eq, kernel_error_provenance_eq};
///
/// # fn main() -> Result<(), Error> {
/// let namespace = ErrorProvenanceNamespace::try_new("example.framework")?;
/// let version = ErrorProvenanceSchemaVersion::try_new(1)?;
/// let current = ErrorProvenanceDescriptor::new(
///     namespace.clone(),
///     version,
///     ErrorProvenanceCode::try_new("current_fault")?,
/// );
/// let origin = ErrorProvenanceDescriptor::new(
///     namespace,
///     version,
///     ErrorProvenanceCode::try_new("origin_fault")?,
/// );
/// let error = Error::for_system(Kind::GatewayError, "Unavailable.")
///     .try_with_attached_provenance(origin.clone())?
///     .try_with_derived_provenance(current.clone())?;
/// let result: Result<(), Error> = Err(error);
/// kernel_error_eq!(&result, Kind::GatewayError, Audience::System, "Unavailable.");
/// kernel_error_provenance_eq!(&result, &current, Some(&origin));
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! kernel_error_provenance_eq {
    ($result:expr, $expected_current:expr, $expected_origin:expr $(,)?) => {
        match $result {
            Ok(_) => panic!(
                "kernel error provenance assertion expected Err(Error), received Ok."
            ),
            Err(error) => match error.provenance() {
                None => panic!(
                    "kernel error provenance assertion expected provenance, but provenance was absent."
                ),
                Some(provenance) => {
                    let expected_current = $expected_current;
                    let expected_origin = $expected_origin;
                    if provenance.current() != expected_current {
                        panic!(
                            "kernel error provenance assertion current descriptor does not match."
                        );
                    }
                    if provenance.origin() != expected_origin {
                        panic!(
                            "kernel error provenance assertion origin descriptor does not match."
                        );
                    }
                },
            },
        }
    };
}

/// Asserts that an [`nape_kernel::error::Error`] matches the expected kind and audience; this DOES NOT
/// check the message content, but verifies there is a message.
///
/// This is useful when you only care about the kind and audience of the error, such as when you
/// expect an error to be returned when some deeply-nested error happens, and you are simply re-throwing
/// that message, but want to make sure the message is not empty.
///
/// # Arguments
///
/// * `$expression` - A `Result` expression that is expected to be an [`nape_kernel::error::Error`].
/// * `$expected_kind` - The expected error kind. Should be of type [`nape_kernel::error::Kind`].
/// * `$expected_audience` - The expected error audience. Should be of type [`nape_kernel::error::Audience`].
/// * `$expected_message` - The expected error message. Should be a [`String`].
///
#[macro_export]
macro_rules! kernel_error_has_message {
    ($result:expr, $expected_kind:expr, $expected_audience:expr) => {
        match $result {
            Ok(val) => panic!(
                "An Error was expected, although one was not returned:\n\t{:?}",
                val
            ),
            Err(e) => {
                assert_eq!(
                    e.kind(),
                    $expected_kind,
                    "{}",
                    format!(
                        "Kind does not match.\n\tExpected: {:?},\n\tActual: {:?}\n",
                        $expected_kind,
                        e.kind()
                    )
                );
                assert_eq!(
                    e.audience(),
                    $expected_audience,
                    "{}",
                    format!(
                        "Audience does not match.\n\tExpected: {:?}\n\tActual: {:?}\n",
                        $expected_audience,
                        e.audience()
                    )
                );
                if e.message().is_empty() {
                    panic!("The error message is empty.  A populated error message is expected.\n");
                }
            }
        }
    };
}

/// Asserts that an [`nape_kernel::error::Error`] has the expected kind, audience, and the message starts with a specific phrase.
///
/// # Arguments
///
/// * `result` - A `Result` expression that is expected to be an [`nape_kernel::error::Error`].
/// * `$expected_kind` - The expected error kind. Should be of type [`nape_kernel::error::Kind`].
/// * `$expected_audience` - The expected error audience. Should be of type [`nape_kernel::error::Audience`].
/// * `$expected_message` - The expected message phrase. Should be a [`String`].
///
#[macro_export]
macro_rules! kernel_error_starts_with {
    ($result:expr, $expected_kind:expr, $expected_audience:expr, $expected_message:expr)=> {
        match $result {
            Ok(val) =>   panic!("An Error was expected, although one was not retured:\n\t{:?}", val),
            Err(e) => {
                assert_eq!(e.kind(), $expected_kind,  "{}", format!("Kind does not match.\n\tExpected:\t{:?}\n\tActual:\t{:?}\n", $expected_kind, e.kind()));
                assert_eq!(e.audience(), $expected_audience,  "{}", format!("Audience does not match.\n\tExpected:\t{:?}\n\tActual:\t{:?}\n ", $expected_audience, e.audience()));
                if !e.message().starts_with($expected_message) {
                    panic!("The Error Message does not start with the expected phrase.\n\tExpected:\t{:?}\n\tActual:\t{:?}\n", $expected_message, e.message());
                }
            }
        }
    };
}

/// Asserts that an [`nape_kernel::error::Error`] has the expected kind, audience, and the message contains a specific phrase.
///
/// # Arguments
///
/// * `&result` - A `Result` expression that is expected to be an [`nape_kernel::error::Error`].
/// * `$expected_kind` - The expected error kind. Should be of type [`nape_kernel::error::Kind`].
/// * `$expected_audience` - The expected error audience. Should be of type [`nape_kernel::error::Audience`].
/// * `$expected_message` - The expected message phrase. Should be a [`String`].
///
#[macro_export]
macro_rules! kernel_error_contains {
    ($result:expr, $expected_kind:expr, $expected_audience:expr, $expected_message:expr)=> {
        match $result {
            Ok(val) =>   panic!("An Error was expected, although one was not retured:\n\t{:?}", val),
            Err(e) => {
                assert_eq!(e.kind(), $expected_kind,  "{}", format!("Kind does not match.\n\tExpected: {:?},\n\tActual: {:?}\n", $expected_kind, e.kind()));
                assert_eq!(e.audience(), $expected_audience,  "{}", format!("Audience does not match.\n\tExpected: {:?}\n\tActual: {:?}\n", $expected_audience, e.audience()));
                if !e.message().contains($expected_message) {
                    panic!("The Error Message does not contains the expected phrase.\n\tExpected:\t{:?}\n\tActual:\t{:?}\n", $expected_message, e.message());
                }
            }
        }
    };
}

/// Asserts that a [`Result`] is an [`Ok`] and returns the value.
/// If the result is an [`Err`], the test will panic with the error message.
///
/// # Arguments
///
/// * `$result` - A `Result` expression that is expected to be an [`Ok`].
///
#[macro_export]
macro_rules! is_ok {
    ($result:expr) => {
        match $result {
            Ok(val) => val,
            Err(e) => panic!(
                "An Ok was expected, although an Error was returned:\n\t{:?}",
                e
            ),
        }
    };
}

/// Asserts that a [`Result`] is an [`Error`] and returns the error value
/// If the result is an [`Ok`], the test will panic with the error message.
///
/// # Arguments
///
/// * `$result` - A `Result` expression that is expected to be an [`Error`].
///
#[macro_export]
macro_rules! is_error {
    ($result:expr) => {
        match $result {
            Ok(val) => panic!("An error was expected, although one was not returned."),
            Err(e) => e,
        }
    };
}
