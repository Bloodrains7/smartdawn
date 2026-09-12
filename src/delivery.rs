use leptos::prelude::*;

use crate::i18n::Lang;

pub struct DeliveryCopy {
    pub kicker: &'static str,
    pub question: &'static str,
    pub options: [&'static str; 3],
    pub descriptions: [&'static str; 3],
    pub modules: [[&'static str; 3]; 3],
    pub extras: [&'static str; 2],
    pub included: &'static str,
    pub stages: [&'static str; 4],
    pub brief: &'static str,
    pub note: &'static str,
    pub pause: &'static str,
}

const SLUGS: [&str; 3] = ["product", "business", "automation"];

const SK: DeliveryCopy = DeliveryCopy {
    kicker: "Od vízie k riešeniu",
    question: "Čo spolu postavíme?",
    options: ["Webový produkt", "Firemný systém", "Automatizácia"],
    descriptions: [
        "Od prvého prototypu po produkt pripravený pre vašich zákazníkov.",
        "Jeden systém pre vaše procesy, dáta a ľudí. Navrhnutý podľa vašej firmy.",
        "Prepojíme nástroje a odstránime opakovanú prácu z každodennej prevádzky.",
    ],
    modules: [
        ["Zákaznícky portál", "Aplikácia a API", "Dáta a prevádzka"],
        ["Pracovné postupy", "Roly a oprávnenia", "Reporty a dáta"],
        ["Vstupné dáta", "Spracovanie úloh", "Kontrola a audit"],
    ],
    extras: ["Prepojenie na existujúce systémy", "AI funkcie"],
    included: "Súčasť každej dodávky",
    stages: ["Analýza", "Návrh a vývoj", "Testovanie", "Nasadenie a podpora"],
    brief: "Preniesť výber do zadania",
    note: "Výber je východisko pre rozhovor. Rozsah, cenu a termín dohodneme po analýze.",
    pause: "Pozastaviť animáciu",
};

const EN: DeliveryCopy = DeliveryCopy {
    kicker: "From vision to solution",
    question: "What shall we build?",
    options: ["Web product", "Business system", "Automation"],
    descriptions: [
        "From the first prototype to a product ready for your customers.",
        "One system for your processes, data and people. Designed around your business.",
        "Connect your tools and remove repetitive work from daily operations.",
    ],
    modules: [
        ["Customer portal", "Application and API", "Data and operations"],
        ["Workflows", "Roles and permissions", "Reports and data"],
        ["Incoming data", "Task processing", "Review and audit"],
    ],
    extras: ["Connect existing systems", "AI features"],
    included: "Included in every delivery",
    stages: ["Discovery", "Design and development", "Testing", "Launch and support"],
    brief: "Add selection to project brief",
    note: "A starting point for a conversation. Scope, price and timeline follow discovery.",
    pause: "Pause animation",
};

const CS: DeliveryCopy = DeliveryCopy {
    kicker: "Od vize k řešení",
    question: "Co spolu postavíme?",
    options: ["Webový produkt", "Firemní systém", "Automatizace"],
    descriptions: [
        "Od prvního prototypu po produkt připravený pro vaše zákazníky.",
        "Jeden systém pro vaše procesy, data a lidi. Navržený podle vaší firmy.",
        "Propojíme nástroje a odstraníme opakovanou práci z každodenního provozu.",
    ],
    modules: [
        ["Zákaznický portál", "Aplikace a API", "Data a provoz"],
        ["Pracovní postupy", "Role a oprávnění", "Reporty a data"],
        ["Vstupní data", "Zpracování úloh", "Kontrola a audit"],
    ],
    extras: ["Propojení se stávajícími systémy", "AI funkce"],
    included: "Součást každé dodávky",
    stages: ["Analýza", "Návrh a vývoj", "Testování", "Nasazení a podpora"],
    brief: "Přenést výběr do zadání",
    note: "Výběr je východiskem pro rozhovor. Rozsah, cenu a termín dohodneme po analýze.",
    pause: "Pozastavit animaci",
};

const DE: DeliveryCopy = DeliveryCopy {
    kicker: "Von der Vision zur Lösung",
    question: "Was entwickeln wir gemeinsam?",
    options: ["Webprodukt", "Unternehmenssystem", "Automatisierung"],
    descriptions: [
        "Vom ersten Prototyp bis zum einsatzbereiten Produkt für Ihre Kunden.",
        "Ein System für Prozesse, Daten und Menschen. Auf Ihr Unternehmen zugeschnitten.",
        "Wir verbinden Ihre Werkzeuge und automatisieren wiederkehrende Aufgaben im Alltag.",
    ],
    modules: [
        ["Kundenportal", "Anwendung und API", "Daten und Betrieb"],
        ["Arbeitsabläufe", "Rollen und Rechte", "Berichte und Daten"],
        ["Eingangsdaten", "Aufgabenverarbeitung", "Kontrolle und Audit"],
    ],
    extras: ["Bestehende Systeme anbinden", "KI-Funktionen"],
    included: "In jeder Lieferung enthalten",
    stages: ["Analyse", "Entwurf und Entwicklung", "Tests", "Einführung und Support"],
    brief: "Auswahl zur Projektanfrage hinzufügen",
    note: "Ein Ausgangspunkt für unser Gespräch. Umfang, Preis und Zeitplan klären wir nach der Analyse.",
    pause: "Animation pausieren",
};

pub fn copy(lang: Lang) -> &'static DeliveryCopy {
    match lang {
        Lang::Sk => &SK,
        Lang::En => &EN,
        Lang::Cs => &CS,
        Lang::De => &DE,
    }
}

#[component]
pub fn DeliveryPreview(lang: Lang) -> impl IntoView {
    let d = copy(lang);
    view! {
        <aside id="navrh" class="delivery-preview" aria-labelledby="delivery-title" data-testid="delivery-preview">
            <div class="delivery-caption"><span class="delivery-light" aria-hidden="true"></span>{d.kicker}<span aria-hidden="true">"01 — 03"</span></div>
            <h2 id="delivery-title">{d.question}</h2>
            <div class="delivery-controls" hidden>
                <label class="sr-only" for="delivery-kind">{d.question}</label>
                <select id="delivery-kind" data-testid="delivery-kind">
                    {SLUGS.iter().zip(d.options).map(|(slug, label)| view! {
                        <option value={*slug}>{label}</option>
                    }).collect::<Vec<_>>()}
                </select>
            </div>
            <div class="delivery-result" aria-live="polite" aria-atomic="true">
                {SLUGS.iter().enumerate().map(|(i, slug)| view! {
                    <article data-scope={*slug} hidden={i != 0}>
                        <h3>{d.options[i]}</h3>
                        <p class="delivery-description">{d.descriptions[i]}</p>
                        <div class="system-map">
                            <div class="system-core" aria-hidden="true"><span class="brand-mark"></span></div>
                            <ol class="system-modules">
                                {d.modules[i].iter().enumerate().map(|(j, label)| view! {
                                    <li><span class="module-index" aria-hidden="true">{format!("0{}", j + 1)}</span><span>{*label}</span><span class="module-dot" aria-hidden="true"></span></li>
                                }).collect::<Vec<_>>()}
                            </ol>
                        </div>
                    </article>
                }).collect::<Vec<_>>()}
                <div class="delivery-extra-summary">
                    {d.extras.iter().enumerate().map(|(i, label)| view! {
                        <span class="delivery-chip" data-extra-chip={i.to_string()} hidden>{*label}</span>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
            <div class="delivery-controls" hidden>
                <div class="delivery-extras">
                    {d.extras.iter().enumerate().map(|(i, label)| view! {
                        <label><input type="checkbox" name="delivery-extra" value={i.to_string()} data-testid={if i == 0 { "delivery-integrations" } else { "delivery-ai" }}/><span>{*label}</span></label>
                    }).collect::<Vec<_>>()}
                </div>
                <button type="button" class="btn primary delivery-brief" data-testid="delivery-brief">{d.brief}<span aria-hidden="true">"↗"</span></button>
            </div>
            <p class="delivery-note">{d.note}</p>
        </aside>
    }
}

#[component]
pub fn DeliveryStages(lang: Lang) -> impl IntoView {
    let d = copy(lang);
    view! {
        <div class="delivery-strip shell">
            <span>{d.included}</span>
            <ol>{d.stages.iter().enumerate().map(|(i, label)| view! {
                <li><span class="stage-index">{format!("0{}", i + 1)}</span>{*label}</li>
            }).collect::<Vec<_>>()}</ol>
        </div>
    }
}
