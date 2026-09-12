use serde::Deserialize;

use crate::i18n::Dict;

#[derive(Debug, Deserialize)]
pub struct ContactFormData {
    pub name: String,
    pub email: String,
    pub company: Option<String>,
    pub message: String,
    pub website: Option<String>,
    pub lang: Option<String>,
}

pub fn validate_contact(form: &ContactFormData, d: &Dict) -> Vec<&'static str> {
    let mut errors = Vec::new();

    if form.name.trim().len() < 2 {
        errors.push(d.val_name_short);
    }
    if form.name.len() > 120 {
        errors.push(d.val_name_long);
    }
    if !is_valid_email(form.email.trim()) {
        errors.push(d.val_email);
    }
    let message_len = form.message.trim().len();
    if message_len < 20 {
        errors.push(d.val_msg_short);
    }
    if message_len > 2500 {
        errors.push(d.val_msg_long);
    }

    if let Some(company) = &form.company {
        if company.len() > 160 {
            errors.push(d.val_company_long);
        }
    }

    errors
}

fn is_valid_email(email: &str) -> bool {
    if email.is_empty() || email.contains(' ') {
        return false;
    }
    let mut parts = email.split('@');
    let local = parts.next().unwrap_or_default();
    let domain = parts.next().unwrap_or_default();
    parts.next().is_none() && !local.is_empty() && domain.contains('.') && !domain.ends_with('.')
}
