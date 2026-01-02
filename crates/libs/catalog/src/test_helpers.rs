use fake::{Fake, StringFaker};
use std::borrow::Cow;
use std::collections::HashMap;
use validator::{ValidationErrors, ValidationErrorsKind};

pub fn random_str(len: usize) -> String {
    const ASCII: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let f = StringFaker::with(Vec::from(ASCII), len);
    f.fake()
}

pub fn unwrap_map<F>(errors: &ValidationErrors, f: F)
where
    F: FnOnce(HashMap<Cow<'_, str>, ValidationErrorsKind>),
{
    let errors = errors.clone();
    f(errors.errors().clone());
}
