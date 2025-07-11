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
                    e.kind,
                    $expected_kind,
                    "{}",
                    format!(
                        "Kind does not match.\n\tExpected: {:?},\n\tActual: {:?}\n",
                        $expected_kind, e.kind
                    )
                );
                assert_eq!(
                    e.audience,
                    $expected_audience,
                    "{}",
                    format!(
                        "Audience does not match.\n\tExpected: {:?}\n\tActual: {:?}\n",
                        $expected_audience, e.audience
                    )
                );
                if e.message != $expected_message {
                    panic!(
                        "The Error Message does not match.\n\tExpected:\t{:?},\n Actual:\t{:?}\n",
                        $expected_message, e.message
                    );
                }
            }
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
                    e.kind,
                    $expected_kind,
                    "{}",
                    format!(
                        "Kind does not match.\n\tExpected: {:?},\n\tActual: {:?}\n",
                        $expected_kind, e.kind
                    )
                );
                assert_eq!(
                    e.audience,
                    $expected_audience,
                    "{}",
                    format!(
                        "Audience does not match.\n\tExpected: {:?}\n\tActual: {:?}\n",
                        $expected_audience, e.audience
                    )
                );
                if e.message.len() == 0 {
                    panic!(
                        "The error message is empty.  A populated error message is expected.\n"
                    );
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
                assert_eq!(e.kind, $expected_kind,  "{}", format!("Kind does not match.\n\tExpected:\t{:?}\n\tActual:\t{:?}\n", $expected_kind, e.kind));
                assert_eq!(e.audience, $expected_audience,  "{}", format!("Audience does not match.\n\tExpected:\t{:?}\n\tActual:\t{:?}\n ", $expected_audience, e.audience));
                if !e.message.starts_with($expected_message) {
                    panic!("The Error Message does not start with the expected phrase.\n\tExpected:\t{:?}\n\tActual:\t{:?}\n", $expected_message, e.message);
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
                assert_eq!(e.kind, $expected_kind,  "{}", format!("Kind does not match.\n\tExpected: {:?},\n\tActual: {:?}\n", $expected_kind, e.kind));
                assert_eq!(e.audience, $expected_audience,  "{}", format!("Audience does not match.\n\tExpected: {:?}\n\tActual: {:?}\n", $expected_audience, e.audience));
                if !e.message.contains($expected_message) {
                    panic!("The Error Message does not contains the expected phrase.\n\tExpected:\t{:?}\n\tActual:\t{:?}\n", $expected_message, e.message);
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
