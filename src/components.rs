use leptos::prelude::*;

use crate::i18n::{dict, Dict, Lang, ThemeTxt, UseCaseTxt};
use crate::delivery::{copy as delivery_copy, DeliveryPreview, DeliveryStages};

const NAV_HREF: [&str; 6] = ["#praca", "#sluzby", "#automatizacie", "#proces", "#stack", "#kontakt"];
const NUM2: [&str; 7] = ["01", "02", "03", "04", "05", "06", "07"];

/// Fixed (language-independent) metadata per capability theme: proof project
/// name, tech tags, accent colour, layout side, and which visual to render.
const THEME_META: [(&str, &[&str], &str, bool, u8); 7] = [
    ("Roman Empire", &["Rust", "Tauri", "DSPy", "Agentic"], "dawn", false, 0),
    ("Voice", &["Whisper", "XTTS-v2", "Claude", "Local-first"], "sky", true, 1),
    ("Forge", &["Rust", "MCP", "Playwright", "OWASP"], "electric", false, 2),
    ("ArchGen", &["LangGraph", "DSPy", "pyo3", "Tauri"], "mint", true, 3),
    ("REPRE", &["Django REST", "Gemini Vision", "Celery", "POHODA"], "sky", false, 4),
    ("AI Avatar", &["Gemini Live", "FastAPI", "Lip-sync", "WebSocket"], "electric", true, 5),
    ("Open Claw", &["Python", "Whisper", "Telegram", "GitLab"], "dawn", false, 6),
];

const INTEGRATIONS: [&str; 10] = [
    "Selenium", "Playwright", "Robot FW", "AI Agent", "LLM · RAG",
    "HTTP", "gRPC", "Postgres", "ClickHouse", "MinIO",
];

fn lang_home(lang: Lang) -> &'static str {
    if lang.prefix().is_empty() { "/" } else { lang.prefix() }
}

#[component]
pub fn App(lang: Lang) -> impl IntoView {
    let d: &'static Dict = dict(lang);
    let home = lang_home(lang);

    view! {
        <div>
            <a class="skip-link" href="#main">{d.skip}</a>
            <div class="scroll-progress" aria-hidden="true"></div>

            <header class="topbar" role="banner">
                <div class="shell topbar-inner">
                    <a class="brand" href={home}>
                        <span class="brand-mark" aria-hidden="true"></span>
                        <span class="brand-text">
                            "SMART DAWN"
                            <small>{d.brand_sub}</small>
                        </span>
                    </a>

                    <nav class="desktop-menu" aria-label="Navigation">
                        {d.nav.iter().zip(NAV_HREF).map(|(label, href)| view! {
                            <a href={href}>{*label}</a>
                        }).collect::<Vec<_>>()}
                    </nav>

                    <div class="topbar-actions">
                        <details class="lang-dd">
                            <summary class="lang-current" aria-label={d.lang_label}>
                                <span class="lang-globe" aria-hidden="true"></span>
                                {lang.label()}
                                <span class="lang-caret" aria-hidden="true"></span>
                            </summary>
                            <div class="lang-menu">
                                {Lang::ALL.iter().map(|&l| {
                                    let href = lang_home(l);
                                    let cls = if l == lang { "lang-opt is-active" } else { "lang-opt" };
                                    view! {
                                        <a class={cls} href={href}>
                                            <span class="lang-code">{l.label()}</span>
                                            <span class="lang-name">{l.name()}</span>
                                        </a>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </details>
                        <a class="btn btn--sm primary" href="#kontakt">{d.cta_start}</a>
                        <details class="mobile-nav">
                            <summary class="hamburger" aria-label="Menu">
                                <span></span><span></span><span></span>
                            </summary>
                            <nav class="mobile-menu" aria-label="Navigation">
                                {d.nav.iter().zip(NAV_HREF).map(|(label, href)| view! {
                                    <a href={href}>{*label}</a>
                                }).collect::<Vec<_>>()}
                                <a class="btn primary" href="#kontakt">{d.cta_start}</a>
                            </nav>
                        </details>
                    </div>
                </div>
            </header>

            <main id="main">

                <section class="hero">
                    <canvas id="neural" aria-hidden="true"></canvas>
                    <div class="hero-veil" aria-hidden="true"></div>
                    <div class="shell hero-content hero-grid">
                      <div class="hero-copy">
                        <span class="eyebrow">
                            <span class="eyebrow-dot" aria-hidden="true"></span>
                            {d.hero_eyebrow}
                        </span>
                        <h1>
                            {d.hero_pre.split_inclusive(". ").map(|line| view! { <span class="headline-line">{line}</span> }).collect::<Vec<_>>()}
                            <span class="grad">
                                {d.hero_accent}
                            </span>
                        </h1>
                        <p class="hero-lead">{d.hero_lead}</p>
                        <div class="hero-actions">
                            <a class="btn primary lg" href="#navrh">{d.hero_cta1}<span aria-hidden="true">"↗"</span></a>
                            <a class="btn ghost lg" href="#kontakt">{d.hero_cta2}</a>
                        </div>
                        <div class="hero-signature"><span class="signature-line" aria-hidden="true"></span>"SMART DAWN / SOFTWARE DELIVERY"</div>
                      </div>
                      <DeliveryPreview lang=lang/>
                    </div>
                    <div class="hero-bottom shell">
                        <span class="hero-runtime" aria-hidden="true">"RUST + WEBASSEMBLY"</span>
                        <div>
                            <button class="motion-toggle" type="button" aria-pressed="false" hidden data-testid="motion-toggle">{delivery_copy(lang).pause}</button>
                        </div>
                    </div>
                </section>
                <DeliveryStages lang=lang/>

                <section class="domains" aria-label="Domains">
                    <div class="shell">
                        <span class="section-kicker">{d.domains_kicker}</span>
                        <p class="domains-lead">{d.domains_lead}</p>
                    </div>
                    <div class="marquee" aria-hidden="true">
                        <div class="marquee-fade marquee-fade--l"></div>
                        <div class="marquee-fade marquee-fade--r"></div>
                        <div class="marquee-track">
                            {d.domains.iter().chain(d.domains.iter()).map(|x| view! {
                                <span class="marquee-item">{*x}<span class="marquee-dot">"◆"</span></span>
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </section>

                <section id="praca" class="themes">
                    <div class="shell">
                        <div class="section-head" data-reveal>
                            <span class="section-kicker">{d.themes_kicker}</span>
                            <h2 class="section-title">{d.themes_title}</h2>
                            <p class="section-intro">{d.themes_intro}</p>
                        </div>
                    </div>
                    {(0..7).map(|i| {
                        let t: &'static ThemeTxt = &d.themes[i];
                        let (name, tags, accent, reverse, vk) = THEME_META[i];
                        let visual = match vk {
                            0 => visual_agents().into_any(),
                            1 => visual_voice().into_any(),
                            2 => visual_testing().into_any(),
                            3 => visual_arch().into_any(),
                            4 => visual_data().into_any(),
                            5 => visual_avatar().into_any(),
                            _ => visual_ops().into_any(),
                        };
                        theme_section(NUM2[i], t, name, tags, accent, reverse, d.proof_label, visual)
                    }).collect::<Vec<_>>()}
                </section>

                <section class="interview" aria-label="Founder">
                    <div class="shell interview-grid">
                        <div class="interview-media" data-reveal>
                            <video class="interview-video" controls playsinline preload="metadata" src="/assets/interview.mp4"></video>
                            <span class="interview-badge" aria-hidden="true">"● REC"</span>
                        </div>
                        <div class="interview-body" data-reveal>
                            <span class="section-kicker">{d.iv_kicker}</span>
                            <h2 class="section-title">"Ondrej Luknár"</h2>
                            <p class="interview-role">{d.iv_role}</p>
                            <p>{d.iv_p1}</p>
                            <p>{d.iv_p2}</p>
                            <div class="cred-row">
                                {d.iv_creds.iter().map(|c| view! { <span class="cred">{*c}</span> }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    </div>
                </section>

                <section id="sluzby" class="bg-alt">
                    <div class="shell">
                        <div class="section-head" data-reveal>
                            <span class="section-kicker">{d.sv_kicker}</span>
                            <h2 class="section-title">{d.sv_title}</h2>
                            <p class="section-intro">{d.sv_intro}</p>
                        </div>
                        <div class="cards">
                            {d.sv_cards.iter().enumerate().map(|(i, c)| view! {
                                <article class="card" data-reveal>
                                    <div class="card-number">{NUM2[i]}</div>
                                    <h3>{c.a}</h3>
                                    <p>{c.b}</p>
                                </article>
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </section>

                <section class="market" aria-label="Runtime platform">
                    <div class="shell">
                        <div class="section-head market-head" data-reveal>
                            <span class="section-kicker">{d.mk_kicker}</span>
                            <h2 class="section-title">{d.mk_title}</h2>
                            <p class="section-intro">{d.mk_intro}</p>
                        </div>
                        <div class="market-stage" data-reveal>
                            <div class="orbit-scene">
                                <div class="orbit-core" aria-hidden="true">
                                    <span class="orbit-core-ring"></span>
                                    <span class="orbit-core-ring orbit-core-ring--2"></span>
                                    <span class="orbit-core-label">"POD"<br/>"RUNTIME"</span>
                                </div>
                                <div class="orbit-ring" aria-hidden="true">
                                    {
                                        let n = INTEGRATIONS.len() as f32;
                                        INTEGRATIONS.iter().enumerate().map(|(i, name)| {
                                            let deg = i as f32 * 360.0 / n;
                                            let style = format!("transform: rotateY({:.0}deg) translateZ(var(--orbit-r));", deg);
                                            view! {
                                                <div class="pod-card" style={style}>
                                                    <span class="pod-card-dot"></span>
                                                    <span class="pod-card-name">{*name}</span>
                                                    <span class="pod-card-tag">"· pod"</span>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()
                                    }
                                </div>
                            </div>
                            <div class="orbit-fog" aria-hidden="true"></div>
                        </div>
                        <div class="market-chips" data-reveal>
                            {d.mk_chips.iter().map(|c| view! { <span class="chip">{*c}</span> }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </section>

                <section id="automatizacie">
                    <div class="shell">
                        <div class="section-head" data-reveal>
                            <span class="section-kicker">{d.au_kicker}</span>
                            <h2 class="section-title">{d.au_title}</h2>
                            <p class="section-intro">{d.au_intro}</p>
                        </div>
                        <div class="use-cases">
                            {d.use_cases.iter().enumerate().map(|(i, uc)| use_case_card(NUM2[i], uc)).collect::<Vec<_>>()}
                        </div>
                        <div class="ai-bottom-row">
                            {d.au_cases.iter().map(|c| view! {
                                <div class="case" data-reveal>
                                    <div class="case-label">{c.a}</div>
                                    <strong>{c.b}</strong>
                                    <p>{c.c}</p>
                                </div>
                            }).collect::<Vec<_>>()}
                        </div>
                        <div class="automate-cta" data-reveal>
                            <div>
                                <strong>{d.au_cta_strong}</strong>
                                <p>{d.au_cta_p}</p>
                            </div>
                            <a class="btn primary lg" href="#kontakt">{d.au_cta_btn}</a>
                        </div>
                    </div>
                </section>

                <section class="impact">
                    <div class="shell">
                        <div class="section-head" data-reveal>
                            <span class="section-kicker">{d.im_kicker}</span>
                            <h2 class="section-title">{d.im_title}</h2>
                            <p class="section-intro">{d.im_intro}</p>
                        </div>
                        <div class="impact-grid">
                            {d.impact.iter().map(|m| view! {
                                <div class="impact-card" data-reveal>
                                    <div class="impact-value" data-count={m.a}>{m.a}</div>
                                    <div class="impact-unit">{m.b}</div>
                                    <div class="impact-desc">{m.c}</div>
                                </div>
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </section>

                <section id="proces" class="bg-alt">
                    <div class="shell">
                        <div class="section-head" data-reveal>
                            <span class="section-kicker">{d.pr_kicker}</span>
                            <h2 class="section-title">{d.pr_title}</h2>
                            <p class="section-intro">{d.pr_intro}</p>
                        </div>
                        <div class="timeline" role="list">
                            {d.steps.iter().enumerate().map(|(i, s)| view! {
                                <div class="step" role="listitem" data-reveal>
                                    <div class="step-number">{(i + 1).to_string()}</div>
                                    <div class="step-content">
                                        <h4>{s.a}</h4>
                                        <p>{s.b}</p>
                                    </div>
                                </div>
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </section>

                <section id="stack">
                    <div class="shell">
                        <div class="section-head" data-reveal>
                            <span class="section-kicker">{d.st_kicker}</span>
                            <h2 class="section-title">{d.st_title}</h2>
                            <p class="section-intro">{d.st_intro}</p>
                        </div>
                        <div class="stack-section" data-reveal>
                            <div>
                                <div class="stack-group-label">{d.st_groups[0]}</div>
                                <div class="stack">
                                    <span class="pill pill--backend"><i class="devicon-rust-original"></i>"Rust"</span>
                                    <span class="pill pill--backend"><i class="devicon-java-plain colored"></i>"Java"</span>
                                    <span class="pill pill--backend"><i class="devicon-kotlin-plain colored"></i>"Kotlin"</span>
                                    <span class="pill pill--backend"><i class="devicon-spring-original colored"></i>"Spring WebFlux"</span>
                                    <span class="pill pill--backend"><i class="devicon-go-plain"></i>"Go"</span>
                                    <span class="pill pill--backend"><i class="devicon-python-plain colored"></i>"Python"</span>
                                    <span class="pill pill--backend"><i class="devicon-zig-original"></i>"Zig"</span>
                                    <span class="pill pill--backend"><i class="devicon-django-plain"></i>"Django + DRF"</span>
                                </div>
                            </div>
                            <div>
                                <div class="stack-group-label">{d.st_groups[1]}</div>
                                <div class="stack">
                                    <span class="pill pill--frontend">"Leptos"</span>
                                    <span class="pill pill--frontend">"WebAssembly"</span>
                                    <span class="pill pill--frontend"><i class="devicon-react-original colored"></i>"React"</span>
                                    <span class="pill pill--frontend"><i class="devicon-nextjs-plain"></i>"Next.js"</span>
                                    <span class="pill pill--frontend"><i class="devicon-angular-original colored"></i>"Angular"</span>
                                    <span class="pill pill--frontend"><i class="devicon-typescript-plain colored"></i>"TypeScript"</span>
                                    <span class="pill pill--frontend"><i class="devicon-grpc-plain"></i>"gRPC"</span>
                                    <span class="pill pill--frontend"><i class="devicon-graphql-plain colored"></i>"GraphQL Federation"</span>
                                </div>
                            </div>
                            <div>
                                <div class="stack-group-label">{d.st_groups[2]}</div>
                                <div class="stack">
                                    <span class="pill pill--infra"><i class="devicon-docker-plain colored"></i>"Docker"</span>
                                    <span class="pill pill--infra"><i class="devicon-kubernetes-plain colored"></i>"Kubernetes"</span>
                                    <span class="pill pill--infra"><i class="devicon-redhat-plain colored"></i>"OpenShift"</span>
                                    <span class="pill pill--infra"><i class="devicon-terraform-plain colored"></i>"Terraform"</span>
                                    <span class="pill pill--infra"><i class="devicon-helm-plain"></i>"Helm"</span>
                                    <span class="pill pill--infra"><i class="devicon-argocd-plain colored"></i>"ArgoCD"</span>
                                    <span class="pill pill--infra"><i class="devicon-amazonwebservices-plain-wordmark colored"></i>"AWS"</span>
                                    <span class="pill pill--infra"><i class="devicon-azure-plain colored"></i>"Azure"</span>
                                    <span class="pill pill--infra"><i class="devicon-googlecloud-plain colored"></i>"GCP"</span>
                                </div>
                            </div>
                            <div>
                                <div class="stack-group-label">{d.st_groups[3]}</div>
                                <div class="stack">
                                    <span class="pill pill--data"><i class="devicon-postgresql-plain colored"></i>"PostgreSQL"</span>
                                    <span class="pill pill--data">"ClickHouse"</span>
                                    <span class="pill pill--data"><i class="devicon-mongodb-plain colored"></i>"MongoDB"</span>
                                    <span class="pill pill--data"><i class="devicon-neo4j-plain colored"></i>"Neo4j"</span>
                                    <span class="pill pill--data"><i class="devicon-apachekafka-original"></i>"Kafka"</span>
                                    <span class="pill pill--data"><i class="devicon-rabbitmq-original colored"></i>"RabbitMQ"</span>
                                    <span class="pill pill--data">"MinIO"</span>
                                </div>
                            </div>
                            <div>
                                <div class="stack-group-label">{d.st_groups[4]}</div>
                                <div class="stack">
                                    <span class="pill pill--ai">"LLM integrácie"</span>
                                    <span class="pill pill--ai">"GraphRAG"</span>
                                    <span class="pill pill--ai">"DSPy"</span>
                                    <span class="pill pill--ai">"MCP"</span>
                                    <span class="pill pill--ai">"Agentic AI"</span>
                                    <span class="pill pill--ai">"Vektorové DB"</span>
                                    <span class="pill pill--ai">"Apache Camel"</span>
                                    <span class="pill pill--ai">"Keycloak"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                <section class="faq bg-alt" aria-label="FAQ">
                    <div class="shell">
                        <div class="section-head" data-reveal>
                            <span class="section-kicker">{d.faq_kicker}</span>
                            <h2 class="section-title">{d.faq_title}</h2>
                        </div>
                        <div data-reveal>
                            {d.faq.iter().enumerate().map(|(i, f)| {
                                if i == 0 {
                                    view! { <details open><summary>{f.a}</summary><p>{f.b}</p></details> }.into_any()
                                } else {
                                    view! { <details><summary>{f.a}</summary><p>{f.b}</p></details> }.into_any()
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </section>

                <section class="dawn-band" aria-label="Smart Dawn">
                    <div class="dawn-horizon" aria-hidden="true"></div>
                    <div class="shell">
                        <p class="dawn-statement" data-reveal>
                            {d.dawn_pre}<span>{d.dawn_em}</span>{d.dawn_post}
                        </p>
                    </div>
                </section>

                <section id="kontakt">
                    <div class="shell">
                        <div class="cta" data-reveal>
                            <span class="section-kicker">{d.ct_kicker}</span>
                            <h2>{d.ct_title}</h2>
                            <p>{d.ct_p}</p>
                            <form class="contact-form" action="/contact" method="post">
                                <input type="hidden" name="lang" value={lang.code()} />
                                <div class="form-row">
                                    <label>
                                        {d.f_name}
                                        <input type="text" name="name" required minlength="2" maxlength="120" placeholder={d.f_name_ph} />
                                    </label>
                                    <label>
                                        {d.f_email}
                                        <input type="email" name="email" required maxlength="180" placeholder={d.f_email_ph} />
                                    </label>
                                </div>
                                <label>
                                    {d.f_company}
                                    <input type="text" name="company" maxlength="160" placeholder={d.f_company_ph} />
                                </label>
                                <label>
                                    {d.f_message}
                                    <textarea name="message" required minlength="20" maxlength="2500" placeholder={d.f_message_ph}></textarea>
                                </label>
                                <input class="honeypot" type="text" name="website" tabindex="-1" autocomplete="off" />
                                <div class="hero-actions">
                                    <button class="btn primary lg" type="submit">{d.submit}</button>
                                    <a class="btn ghost lg" href="mailto:hello@smartdawn.eu">"hello@smartdawn.eu"</a>
                                </div>
                                <p class="form-note">{d.form_note}</p>
                            </form>
                        </div>
                    </div>
                </section>
            </main>

            <footer class="footer" role="contentinfo">
                <div class="shell">
                    <div class="footer-inner">
                        <div>
                            <div class="footer-brand">
                                <span class="brand-mark" aria-hidden="true"></span>
                                "Smart Dawn"
                            </div>
                            <p class="footer-desc">{d.footer_desc}</p>
                            <p class="footer-tagline">{d.footer_tag_pre}<span>{d.footer_tag_em}</span>{d.footer_tag_post}</p>
                        </div>
                        <nav class="footer-links" aria-label="Footer">
                            {
                                let hrefs = ["#praca", "#sluzby", "#proces", "#kontakt", "mailto:hello@smartdawn.eu"];
                                d.footer_links.iter().zip(hrefs).map(|(label, href)| view! {
                                    <a href={href}>{*label}</a>
                                }).collect::<Vec<_>>()
                            }
                        </nav>
                    </div>
                    <p class="footer-copy">{d.footer_copy}</p>
                </div>
            </footer>
        </div>
    }
}

fn theme_section(
    num: &'static str,
    t: &'static ThemeTxt,
    proof_name: &'static str,
    tags: &'static [&'static str],
    accent: &'static str,
    reverse: bool,
    proof_label: &'static str,
    visual: AnyView,
) -> impl IntoView {
    let cls = if reverse { "theme theme--reverse" } else { "theme" };
    let proof_cls = format!("proof-card proof--{}", accent);
    view! {
        <div class={cls} data-reveal>
            <div class="shell theme-grid">
                <div class="theme-copy">
                    <div class="theme-num">{num}</div>
                    <span class="section-kicker">{t.kicker}</span>
                    <h3 class="theme-title">{t.title}</h3>
                    <p class="theme-desc">{t.desc}</p>
                    <div class={proof_cls}>
                        <span class="proof-label">{proof_label}</span>
                        <div class="proof-head">
                            <strong>{proof_name}</strong>
                            <span class="proof-tagline">{t.proof_tagline}</span>
                        </div>
                        <div class="proof-tags">
                            {tags.iter().map(|x| view! { <span class="chip">{*x}</span> }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </div>
                <div class="theme-visual">{visual}</div>
            </div>
        </div>
    }
}

fn use_case_card(badge: &'static str, uc: &'static UseCaseTxt) -> impl IntoView {
    let last = uc.flow.len().saturating_sub(1);
    view! {
        <article class="use-case" data-reveal>
            <div class="use-case-header">
                <div class="use-case-badge">{badge}</div>
                <h3>{uc.title}</h3>
            </div>
            <p>{uc.desc}</p>
            <div class="flow">
                {uc.flow.iter().enumerate().map(|(i, step)| {
                    let arrow = if i < last {
                        Some(view! { <span class="flow-arrow" aria-hidden="true">"→"</span> })
                    } else {
                        None
                    };
                    view! { <span class="flow-step">{*step}</span>{arrow} }
                }).collect::<Vec<_>>()}
            </div>
            <div class="use-case-result">{uc.result}</div>
        </article>
    }
}

fn visual_agents() -> impl IntoView {
    let nodes: [(f32, f32, &str); 6] = [
        (300.0, 150.0, "Senát"),
        (240.0, 52.0, "Légia"),
        (120.0, 52.0, "Gladiátor"),
        (60.0, 150.0, "Censor"),
        (120.0, 248.0, "Vigil"),
        (240.0, 248.0, "Courier"),
    ];
    view! {
        <div class="viz viz-agents">
            <svg viewBox="0 0 360 300" class="ag-svg" preserveAspectRatio="xMidYMid meet">
                {nodes.iter().enumerate().map(|(i, &(x, y, _))| {
                    let st = format!("--d:{}ms", i * 260);
                    view! { <line class="ag-link" x1="180" y1="150" x2={x.to_string()} y2={y.to_string()} style={st}/> }
                }).collect::<Vec<_>>()}
                <circle class="ag-core" cx="180" cy="150" r="30"/>
                <text class="ag-core-t" x="180" y="154">"TASK"</text>
                {nodes.iter().enumerate().map(|(i, &(x, y, label))| {
                    let st = format!("--d:{}ms", i * 260);
                    let ly = (y + 27.0).to_string();
                    view! {
                        <g>
                            <circle class="ag-node" cx={x.to_string()} cy={y.to_string()} r="13" style={st}/>
                            <text class="ag-node-t" x={x.to_string()} y={ly}>{label}</text>
                        </g>
                    }
                }).collect::<Vec<_>>()}
            </svg>
        </div>
    }
}

fn visual_voice() -> impl IntoView {
    view! {
        <div class="viz viz-voice">
            <div class="voice-orb">
                <div class="voice-eq">
                    {(0..11).map(|i| {
                        let st = format!("--d:{}ms", (i * 110) % 760);
                        view! { <span class="voice-bar" style={st}></span> }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
            <div class="voice-pipe">
                <span class="vp">"wake"</span>
                <i class="vp-arr">"→"</i>
                <span class="vp">"STT"</span>
                <i class="vp-arr">"→"</i>
                <span class="vp">"LLM"</span>
                <i class="vp-arr">"→"</i>
                <span class="vp">"TTS"</span>
            </div>
        </div>
    }
}

fn visual_testing() -> impl IntoView {
    let rows = ["GET /orders", "POST /checkout", "GraphQL · cart", "gRPC · Pay", "WS · stream"];
    view! {
        <div class="viz viz-test">
            <div class="test-ring">
                <span class="test-ring-num">"98%"</span>
                <span class="test-ring-lbl">"pass"</span>
            </div>
            <ul class="test-rows">
                {rows.iter().enumerate().map(|(i, r)| {
                    let st = format!("--d:{}ms", i * 360);
                    view! {
                        <li class="test-row" style={st}>
                            <span class="test-name">{*r}</span>
                            <span class="test-pass">"PASS"</span>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

fn visual_arch() -> impl IntoView {
    view! {
        <div class="viz viz-arch">
            <svg viewBox="0 0 320 230" class="arch-svg" preserveAspectRatio="xMidYMid meet">
                <line class="arch-edge" x1="160" y1="58" x2="80" y2="130" style="--d:200ms"/>
                <line class="arch-edge" x1="160" y1="58" x2="240" y2="130" style="--d:500ms"/>
                <line class="arch-edge" x1="80" y1="130" x2="160" y2="188" style="--d:800ms"/>
                <line class="arch-edge" x1="240" y1="130" x2="160" y2="188" style="--d:1100ms"/>
                <g class="arch-box" style="--d:0ms"><rect x="122" y="30" width="76" height="40" rx="8"/><text x="160" y="54">"UI"</text></g>
                <g class="arch-box" style="--d:300ms"><rect x="42" y="110" width="76" height="40" rx="8"/><text x="80" y="134">"API"</text></g>
                <g class="arch-box" style="--d:600ms"><rect x="202" y="110" width="76" height="40" rx="8"/><text x="240" y="134">"Auth"</text></g>
                <g class="arch-box" style="--d:900ms"><rect x="122" y="168" width="76" height="40" rx="8"/><text x="160" y="192">"DB"</text></g>
            </svg>
        </div>
    }
}

fn visual_data() -> impl IntoView {
    let stages = ["Feed", "Preklad", "Vision", "POHODA"];
    view! {
        <div class="viz viz-data">
            <div class="data-rail">
                <div class="data-node-row">
                    {(0..4).map(|_| view! { <span class="data-node"></span> }).collect::<Vec<_>>()}
                </div>
                {(0..4).map(|i| {
                    let st = format!("--d:{}ms", i * 850);
                    view! { <span class="data-packet" style={st}></span> }
                }).collect::<Vec<_>>()}
            </div>
            <div class="data-labels">
                {stages.iter().map(|s| view! { <span class="data-lbl">{*s}</span> }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

fn visual_avatar() -> impl IntoView {
    view! {
        <div class="viz viz-avatar">
            <div class="avatar-head">
                <span class="avatar-live" aria-hidden="true">"● LIVE"</span>
                <span class="avatar-eye"></span>
                <span class="avatar-eye avatar-eye--r"></span>
                <div class="avatar-mouth">
                    {(0..7).map(|i| {
                        let st = format!("--d:{}ms", (i * 90) % 500);
                        view! { <span class="avatar-bar" style={st}></span> }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
            <div class="avatar-langs">
                <span class="vp">"SK"</span>
                <span class="vp">"EN"</span>
                <span class="vp">"lip-sync · 480 ms"</span>
            </div>
        </div>
    }
}

fn visual_ops() -> impl IntoView {
    let nodes: [(f32, f32, &str); 6] = [
        (300.0, 150.0, "E-mail"),
        (240.0, 52.0, "Slack"),
        (120.0, 52.0, "Notion"),
        (60.0, 150.0, "GitLab"),
        (120.0, 248.0, "Telegram"),
        (240.0, 248.0, "Meet"),
    ];
    view! {
        <div class="viz viz-agents">
            <svg viewBox="0 0 360 300" class="ag-svg" preserveAspectRatio="xMidYMid meet">
                {nodes.iter().enumerate().map(|(i, &(x, y, _))| {
                    let st = format!("--d:{}ms", i * 260);
                    view! { <line class="ag-link" x1="180" y1="150" x2={x.to_string()} y2={y.to_string()} style={st}/> }
                }).collect::<Vec<_>>()}
                <circle class="ag-core" cx="180" cy="150" r="30"/>
                <text class="ag-core-t" x="180" y="154">"AI"</text>
                {nodes.iter().enumerate().map(|(i, &(x, y, label))| {
                    let st = format!("--d:{}ms", i * 260);
                    let ly = (y + 27.0).to_string();
                    view! {
                        <g>
                            <circle class="ag-node" cx={x.to_string()} cy={y.to_string()} r="13" style={st}/>
                            <text class="ag-node-t" x={x.to_string()} y={ly}>{label}</text>
                        </g>
                    }
                }).collect::<Vec<_>>()}
            </svg>
        </div>
    }
}

#[component]
pub fn StatusPage(
    icon: &'static str,
    title: &'static str,
    message: &'static str,
    back_label: &'static str,
    home: &'static str,
    #[prop(optional)] errors: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <main class="shell">
            <article class="status-card">
                <div class="status-icon">{icon}</div>
                <h1>{title}</h1>
                <p>{message}</p>
                {if !errors.is_empty() {
                    Some(view! {
                        <ul>
                            {errors.into_iter().map(|e| view! { <li>{e}</li> }).collect::<Vec<_>>()}
                        </ul>
                    })
                } else {
                    None
                }}
                <div class="hero-actions">
                    <a class="btn primary" href={home}>{back_label}</a>
                </div>
            </article>
        </main>
    }
}

#[component]
pub fn NotFoundPage(title: &'static str, message: &'static str, back_label: &'static str, home: &'static str) -> impl IntoView {
    view! {
        <main class="shell">
            <article class="status-card">
                <div class="status-icon">"404"</div>
                <h1>{title}</h1>
                <p>{message}</p>
                <div class="hero-actions">
                    <a class="btn primary" href={home}>{back_label}</a>
                </div>
            </article>
        </main>
    }
}
