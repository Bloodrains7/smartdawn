use axum::{
    extract::Form,
    http::{header, StatusCode},
    response::{Html, IntoResponse},
};
use leptos::prelude::*;

use crate::components::{App, NotFoundPage, StatusPage};
use crate::i18n::{dict, Lang};
use crate::seo::{render_document, site_url};
use crate::validation::{validate_contact, ContactFormData};

fn render_index(lang: Lang) -> Html<String> {
    let d = dict(lang);
    let app_html = view! { <App lang=lang/> }.to_html();
    let path = if lang.prefix().is_empty() { "/" } else { lang.prefix() };
    Html(render_document(&app_html, d.meta_title, d.meta_desc, lang, path, false))
}

pub async fn index() -> Html<String> {
    render_index(Lang::Sk)
}
pub async fn index_en() -> Html<String> {
    render_index(Lang::En)
}
pub async fn index_cz() -> Html<String> {
    render_index(Lang::Cs)
}
pub async fn index_de() -> Html<String> {
    render_index(Lang::De)
}

pub async fn contact(Form(form): Form<ContactFormData>) -> impl IntoResponse {
    let lang = Lang::from_code(form.lang.as_deref());
    let d = dict(lang);
    let home = lang.home();

    // Honeypot: silently accept.
    if form.website.as_ref().is_some_and(|v| !v.trim().is_empty()) {
        let html = view! {
            <StatusPage icon="✅" title=d.thanks_title message=d.thanks_msg back_label=d.back_home home=home/>
        }
        .to_html();
        return (
            StatusCode::OK,
            Html(render_document(&html, d.meta_title, d.thanks_msg, lang, "/contact", true)),
        );
    }

    let errors = validate_contact(&form, d);
    if !errors.is_empty() {
        let html = view! {
            <StatusPage icon="⚠️" title=d.err_title message=d.err_msg back_label=d.back_home home=home errors=errors/>
        }
        .to_html();
        return (
            StatusCode::BAD_REQUEST,
            Html(render_document(&html, d.meta_title, d.err_msg, lang, "/contact", true)),
        );
    }

    tracing::info!(
        contact_name = %form.name.trim(),
        contact_email = %form.email.trim(),
        lang = lang.code(),
        has_company = form.company.as_ref().is_some_and(|v| !v.trim().is_empty()),
        message_len = form.message.trim().len(),
        "contact form submitted"
    );

    let html = view! {
        <StatusPage icon="🎉" title=d.thanks_title message=d.thanks_msg back_label=d.back_home home=home/>
    }
    .to_html();
    (
        StatusCode::OK,
        Html(render_document(&html, d.meta_title, d.thanks_msg, lang, "/contact", true)),
    )
}

pub async fn sitemap() -> impl IntoResponse {
    let base = site_url();
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url><loc>{b}/</loc><changefreq>weekly</changefreq><priority>1.0</priority></url>
  <url><loc>{b}/en</loc><changefreq>weekly</changefreq><priority>0.9</priority></url>
  <url><loc>{b}/cz</loc><changefreq>weekly</changefreq><priority>0.9</priority></url>
  <url><loc>{b}/de</loc><changefreq>weekly</changefreq><priority>0.9</priority></url>
</urlset>"#,
        b = base
    );
    (
        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        xml,
    )
}

pub async fn robots_txt() -> impl IntoResponse {
    let base = site_url();
    let body = format!("User-agent: *\nAllow: /\nSitemap: {}/sitemap.xml\n", base);
    ([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], body)
}

pub async fn healthz() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

pub async fn not_found() -> impl IntoResponse {
    let d = dict(Lang::Sk);
    let html = view! {
        <NotFoundPage title=d.notfound_title message=d.notfound_msg back_label=d.back_home home="/"/>
    }
    .to_html();
    (
        StatusCode::NOT_FOUND,
        Html(render_document(&html, d.meta_title, d.notfound_msg, Lang::Sk, "/404", true)),
    )
}
