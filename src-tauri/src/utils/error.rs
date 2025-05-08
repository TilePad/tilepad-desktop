/// Attempt to cast an error to a specific error type, will follow the
/// error source attempting to cast source errors
pub fn try_cast_error<'a, O: std::error::Error + 'static>(
    error: &'a (dyn std::error::Error + 'static),
) -> Option<&'a O> {
    error.downcast_ref::<O>().or_else(|| {
        let source = error.source()?;
        try_cast_error(source)
    })
}
