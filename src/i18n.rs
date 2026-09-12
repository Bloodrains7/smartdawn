//! Internationalisation. All user-facing copy lives here as a `Dict` per
//! language; `components.rs` renders purely from a `&'static Dict`. Adding a
//! language = one more `Dict` instance. Language is chosen by URL prefix
//! (`/`, `/en`, `/cz`, `/de`).

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Sk,
    En,
    Cs,
    De,
}

impl Lang {
    pub const ALL: [Lang; 4] = [Lang::Sk, Lang::En, Lang::Cs, Lang::De];

    /// HTML `lang` attribute / og:locale base.
    pub fn code(self) -> &'static str {
        match self {
            Lang::Sk => "sk",
            Lang::En => "en",
            Lang::Cs => "cs",
            Lang::De => "de",
        }
    }

    /// URL path prefix (empty for the default Slovak).
    pub fn prefix(self) -> &'static str {
        match self {
            Lang::Sk => "",
            Lang::En => "/en",
            Lang::Cs => "/cz",
            Lang::De => "/de",
        }
    }

    /// Short label for the language switcher.
    pub fn label(self) -> &'static str {
        match self {
            Lang::Sk => "SK",
            Lang::En => "EN",
            Lang::Cs => "CZ",
            Lang::De => "DE",
        }
    }

    /// Endonym for the language menu.
    pub fn name(self) -> &'static str {
        match self {
            Lang::Sk => "Slovenčina",
            Lang::En => "English",
            Lang::Cs => "Čeština",
            Lang::De => "Deutsch",
        }
    }

    /// Home URL for this language (always a valid path).
    pub fn home(self) -> &'static str {
        match self {
            Lang::Sk => "/",
            _ => self.prefix(),
        }
    }

    pub fn from_code(code: Option<&str>) -> Lang {
        match code {
            Some("en") => Lang::En,
            Some("cs") | Some("cz") => Lang::Cs,
            Some("de") => Lang::De,
            _ => Lang::Sk,
        }
    }
}

pub struct ThemeTxt {
    pub kicker: &'static str,
    pub title: &'static str,
    pub desc: &'static str,
    pub proof_tagline: &'static str,
}

pub struct UseCaseTxt {
    pub title: &'static str,
    pub desc: &'static str,
    pub flow: &'static [&'static str],
    pub result: &'static str,
}

pub struct Pair {
    pub a: &'static str,
    pub b: &'static str,
}

pub struct Trio {
    pub a: &'static str,
    pub b: &'static str,
    pub c: &'static str,
}

pub struct Dict {
    pub html_lang: &'static str,
    pub meta_title: &'static str,
    pub meta_desc: &'static str,

    // nav / chrome
    pub nav: [&'static str; 6],
    pub cta_start: &'static str,
    pub skip: &'static str,
    pub brand_sub: &'static str,
    pub lang_label: &'static str,

    // hero
    pub hero_eyebrow: &'static str,
    pub hero_pre: &'static str,
    pub hero_accent: &'static str,
    pub hero_lead: &'static str,
    pub hero_cta1: &'static str,
    pub hero_cta2: &'static str,

    // domains
    pub domains_kicker: &'static str,
    pub domains_lead: &'static str,
    pub domains: &'static [&'static str],

    // themes
    pub themes_kicker: &'static str,
    pub themes_title: &'static str,
    pub themes_intro: &'static str,
    pub proof_label: &'static str,
    pub themes: [ThemeTxt; 7],

    // interview
    pub iv_kicker: &'static str,
    pub iv_role: &'static str,
    pub iv_p1: &'static str,
    pub iv_p2: &'static str,
    pub iv_creds: [&'static str; 3],

    // services
    pub sv_kicker: &'static str,
    pub sv_title: &'static str,
    pub sv_intro: &'static str,
    pub sv_cards: [Pair; 3], // (title, body)

    // marketplace / runtime
    pub mk_kicker: &'static str,
    pub mk_title: &'static str,
    pub mk_intro: &'static str,
    pub mk_chips: &'static [&'static str],

    // automations
    pub au_kicker: &'static str,
    pub au_title: &'static str,
    pub au_intro: &'static str,
    pub use_cases: [UseCaseTxt; 5],
    pub au_cases: [Trio; 3], // (label, strong, body)
    pub au_cta_strong: &'static str,
    pub au_cta_p: &'static str,
    pub au_cta_btn: &'static str,

    // impact
    pub im_kicker: &'static str,
    pub im_title: &'static str,
    pub im_intro: &'static str,
    pub impact: [Trio; 4], // (value, unit, desc)

    // process
    pub pr_kicker: &'static str,
    pub pr_title: &'static str,
    pub pr_intro: &'static str,
    pub steps: [Pair; 4], // (title, body)

    // stack
    pub st_kicker: &'static str,
    pub st_title: &'static str,
    pub st_intro: &'static str,
    pub st_groups: [&'static str; 5],

    // faq
    pub faq_kicker: &'static str,
    pub faq_title: &'static str,
    pub faq: [Pair; 4], // (question, answer)

    // dawn beat
    pub dawn_pre: &'static str,
    pub dawn_em: &'static str,
    pub dawn_post: &'static str,

    // contact
    pub ct_kicker: &'static str,
    pub ct_title: &'static str,
    pub ct_p: &'static str,
    pub f_name: &'static str,
    pub f_name_ph: &'static str,
    pub f_email: &'static str,
    pub f_email_ph: &'static str,
    pub f_company: &'static str,
    pub f_company_ph: &'static str,
    pub f_message: &'static str,
    pub f_message_ph: &'static str,
    pub form_note: &'static str,
    pub submit: &'static str,

    // footer
    pub footer_desc: &'static str,
    pub footer_tag_pre: &'static str,
    pub footer_tag_em: &'static str,
    pub footer_tag_post: &'static str,
    pub footer_links: [&'static str; 5],
    pub footer_copy: &'static str,

    // status / 404 / validation
    pub thanks_title: &'static str,
    pub thanks_msg: &'static str,
    pub err_title: &'static str,
    pub err_msg: &'static str,
    pub notfound_title: &'static str,
    pub notfound_msg: &'static str,
    pub back_home: &'static str,
    pub val_name_short: &'static str,
    pub val_name_long: &'static str,
    pub val_email: &'static str,
    pub val_msg_short: &'static str,
    pub val_msg_long: &'static str,
    pub val_company_long: &'static str,
}

pub fn dict(lang: Lang) -> &'static Dict {
    match lang {
        Lang::Sk => &SK,
        Lang::En => &EN,
        Lang::Cs => &CS,
        Lang::De => &DE,
    }
}

pub static SK: Dict = Dict {
    html_lang: "sk",
    meta_title: "Smart Dawn | Kompletné dodávky softvéru na mieru",
    meta_desc: "Softvér na mieru od analýzy a návrhu cez vývoj a testovanie až po nasadenie a podporu. Webové produkty, firemné systémy, integrácie a AI.",

    nav: ["Práca", "Služby", "Automatizácie", "Proces", "Tech", "Kontakt"],
    cta_start: "Začať projekt",
    skip: "Preskočiť na obsah",
    brand_sub: "SOFTWARE DELIVERY",
    lang_label: "Jazyk",

    hero_eyebrow: "Kompletné dodávky softvéru na mieru",
    hero_pre: "Váš nápad. Náš kód. ",
    hero_accent: "Nový začiatok.",
    hero_lead: "Od prvého rozhovoru po nasadený produkt. Navrhneme, vyvinieme a otestujeme váš softvér. Prepojíme ho s vaším biznisom a postaráme sa o jeho ďalší rozvoj.",
    hero_cta1: "Navrhnime váš projekt",
    hero_cta2: "Porozprávajme sa",

    domains_kicker: "Žiadny biznis nám nie je cudzí",
    domains_lead: "Doménu pochopíme rýchlejšie, než čakáte — a postavíme v nej softvér. Od regulovaného enterprise po rýchle digitálne produkty.",
    domains: &[
        "Online casino", "Bankovníctvo", "Telco", "E-commerce", "Fintech",
        "Poisťovníctvo", "Healthcare", "Logistika", "Výroba", "Gaming",
        "Verejný sektor", "Energetika",
    ],

    themes_kicker: "Čo staviame",
    themes_title: "Sedem schopností. Každú máme overenú na vlastnom projekte.",
    themes_intro: "Nehovoríme, že to vieme — ukazujeme. Za každou schopnosťou je reálny softvér postavený v Smart Dawn ako dôkaz, nie slajd v prezentácii.",
    proof_label: "Dôkaz z praxe",
    themes: [
        ThemeTxt {
            kicker: "Agentové systémy",
            title: "Légie agentov, ktoré dodávajú",
            desc: "Viacero AI agentov, ktoré spolupracujú, plánujú, navzájom sa kontrolujú a dotiahnu úlohu do konca — s ľudským schválením, pamäťou, nástrojmi a guardrails. Nie jeden chatbot, ale celý tím so zodpovednosťami.",
            proof_tagline: "Légia 12+ AI agentov, ktorá dodá softvér",
        },
        ThemeTxt {
            kicker: "Hlasové & realtime rozhrania",
            title: "AI, ktorá počúva a hovorí",
            desc: "Wake-word, prepis reči, odpoveď a syntéza hlasu v reálnom čase, s prirodzeným prerušovaním. Hlasové agenty pre call centrá, asistentov či živý prepis porád — lokálne aj v cloude, s latenciou pod 300 ms.",
            proof_tagline: "Hlasový agent, čo beží u teba",
        },
        ThemeTxt {
            kicker: "AI testovanie & QA",
            title: "Testy, ktoré sa napíšu samy",
            desc: "Generovanie a beh testov z OpenAPI alebo prirodzeného jazyka, cross-system verifikácia (HTTP + databáza + fronty), OWASP bezpečnostný sken a sledovanie regresií. QA, ktoré drží krok s vývojom, nie ho brzdí.",
            proof_tagline: "Autonómny tester API",
        },
        ThemeTxt {
            kicker: "Architektúra na autopilota",
            title: "Diagramy a dokumentácia z textu",
            desc: "Z popisu vznikne UML, BPMN či ArchiMate diagram aj architektonická dokumentácia podľa arc42, C4 a TOGAF — a udržiava sa spolu s kódom. Druhý názor architekta, kedykoľvek ho potrebujete.",
            proof_tagline: "AI generátor architektúry",
        },
        ThemeTxt {
            kicker: "Dáta & AI pipeliny",
            title: "Dáta, ktoré sa upracú samy",
            desc: "Ingest z hocijakého zdroja, preklad, obohatenie cez vision AI a doručenie do cieľového systému — s auditom a kontrolou nákladov. Tisíce položiek bez ručného prepisovania a copy-paste.",
            proof_tagline: "AI ingest produktových dát",
        },
        ThemeTxt {
            kicker: "Digitálni dvojníci & avatari",
            title: "Avatar, ktorý sa s vami rozpráva",
            desc: "Hovoriaci AI avatar v reálnom čase — odpovedá hlasom, s presnou synchronizáciou pier a latenciou pod 500 ms, dvojjazyčne. Naskenujete logo a v prehliadači ožije 3D avatar ako živá vizitka. Mimochodom — video so zakladateľom vyššie vygeneroval presne tento engine.",
            proof_tagline: "Real-time hlasový avatar (Gemini Live)",
        },
        ThemeTxt {
            kicker: "AI asistenti pre operatívu",
            title: "Asistent, ktorý drží firmu v chode",
            desc: "AI, ktorá triedi a odpovedá na e-maily, sleduje úlohy, robí code review v GitLabe, prepisuje a sumarizuje porady a všetko prepája cez Telegram, Slack a Notion. Operatíva, ktorá beží, aj keď vy nie.",
            proof_tagline: "AI e-mailový a ops asistent",
        },
    ],

    iv_kicker: "Kto za tým stojí",
    iv_role: "Zakladateľ · Solution Architect & Tech Lead",
    iv_p1: "„Nedizajnujem len na whiteboarde — staviam.“ Viac než 10 rokov stavia vysokovýkonné enterprise platformy v bankovníctve, telco a korporátnom prostredí, kde sa stabilita stretáva s rýchlosťou startupu.",
    iv_p2: "Dnes spája túto enterprise disciplínu s agentovou AI — vlastný R&D Smart Dawn stojí na produkčne použiteľných agentových systémoch, nie na demách.",
    iv_creds: ["RAG & Agentic AI · IBM", "Retrieval-Augmented Generation · DeepLearning.AI", "Bratislava, SK"],

    sv_kicker: "Služby",
    sv_title: "Od analýzy po prevádzku",
    sv_intro: "Dodáme celý projekt alebo konkrétny modul — výskum, frontend, mikroservisu, AI pipeline alebo integráciu. Vždy s vlastníctvom dodávky až do produkcie.",
    sv_cards: [
        Pair { a: "Discovery a architektúra", b: "Business analýza, technická architektúra, roadmapa a rozpočet v reálnych míľnikoch. Vieme, kde sú riziká ešte pred prvým riadkom kódu." },
        Pair { a: "AI a agentové systémy", b: "Produkčné RAG, agentic workflow, hlasové a multimodálne rozhrania. S guardrails, ľudskou kontrolou a auditom — nie čierna skrinka." },
        Pair { a: "Prevádzka a rozvoj", b: "SLA podpora, observability, automatizované testy a bezpečnostné revízie. Kontinuálny rozvoj podľa dát z produkcie, nie podľa technického dlhu." },
    ],

    mk_kicker: "Behová platforma",
    mk_title: "Jedna vrstva, na ktorej beží čokoľvek",
    mk_intro: "Izolovaný runtime, ktorý na požiadanie spustí ľubovoľnú úlohu ako samostatný pod — testy, integrácie, dátové joby, AI agentov, aj celý marketplace. Cez frontu ho vytvorí, sleduje jeho zdroje a po dobehnutí ho zruší. Tú istú vrstvu zapojíte do hocijakého biznisu. Beží na Kubernetes, OpenShift aj v čistom Dockeri — v cloude aj on-premise.",
    mk_chips: &[
        "Kubernetes", "OpenShift", "Docker", "RabbitMQ control plane",
        "Ephemeral & izolované", "Monitoring zdrojov", "Auto-cleanup", "SaaS aj on-premise",
    ],

    au_kicker: "AI automatizácie",
    au_title: "Ak to robíte opakovane, vieme to zautomatizovať",
    au_intro: "Nemáme fixný cenník hotových služieb. Má proces vstup, pravidlá a výstup? Postavíme okolo neho AI — s ľudskou kontrolou tam, kde na tom záleží. Nižšie je pár príkladov z praxe, nie strop toho, čo zvládneme.",
    use_cases: [
        UseCaseTxt { title: "E-maily a dopyty", desc: "AI prečíta prichádzajúci e-mail, rozpozná typ, vytiahne kľúčové údaje a pripraví návrh odpovede. Človek len skontroluje a odošle.", flow: &["E-mail príde", "AI klasifikuje", "Extrakcia údajov", "Návrh odpovede", "Schválenie"], result: "Z 15 minút na 2 minúty na dopyt" },
        UseCaseTxt { title: "Faktúry, zmluvy a dokumenty", desc: "Nahráte PDF alebo scan — AI prečíta obsah, rozpozná typ dokumentu, vytiahne sumy, dátumy a dodávateľa a zapíše údaje priamo do vášho systému.", flow: &["Dokument príde", "AI prečíta obsah", "Extrakcia dát", "Zápis do systému"], result: "Až 90% menej manuálneho prepisovania" },
        UseCaseTxt { title: "Obrázky a videá", desc: "AI analyzuje fotky alebo video — rozpozná produkty, prečíta text z obrázkov, identifikuje chyby alebo klasifikuje obsah. Výstup ide do reportu alebo spúšťa ďalší krok.", flow: &["Vstup média", "AI analýza", "Klasifikácia", "Report / alert"], result: "Kontrola v sekundách namiesto hodín" },
        UseCaseTxt { title: "Interný AI asistent", desc: "Zamestnanec sa opýta — AI prehľadá firemnú dokumentáciu, interné postupy a pravidlá a odpovie aj s odkazom na zdroj. Menej otázok na kolegov, rýchlejší onboarding.", flow: &["Otázka", "Prehľadá dokumentáciu", "Odpoveď + zdroj"], result: "Onboarding nových ľudí 3x rýchlejší" },
        UseCaseTxt { title: "Transformácia a prepájanie dát", desc: "Dáta z jedného systému treba dostať do druhého v inom formáte? AI rozumie štruktúre, premapuje polia, doplní chýbajúce údaje a doručí výstup kam treba.", flow: &["Vstupné dáta", "AI transformácia", "Validácia", "Výstup do cieľa"], result: "Koniec ručného copy-paste medzi systémami" },
    ],
    au_cases: [
        Trio { a: "Vždy pod kontrolou", b: "Žiadna čierna skrinka", c: "Každé riešenie má jasné pravidlá — kto čo schvaľuje, čo sa loguje a kde do procesu vstupuje človek. Po celý čas máte prehľad o tom, čo AI urobila a prečo." },
        Trio { a: "Beží na vašom", b: "Napojíme sa na to, čo máte", c: "E-mail, CRM, ERP, SharePoint, interné nástroje — pracujeme s vašimi dátami vo vašom prostredí. Žiadna migrácia, žiadny vendor lock-in." },
        Trio { a: "AI nie je povinnosť", b: "Aj bez AI, keď tak chcete", c: "Nie ste fanúšik AI? Nevadí. Veľa procesov vieme zautomatizovať aj klasicky — deterministické pravidlá, integrácie a skripty bez modelu. AI nasadíme len tam, kde reálne pridáva hodnotu, nie pre efekt." },
    ],
    au_cta_strong: "Váš proces tu nie je? To je často najlepšia správa.",
    au_cta_p: "Práve tie netypické, „len u nás sa to robí takto“ procesy mávajú najväčšiu návratnosť. Opíšte nám jeden a rovno vám povieme, či a ako sa dá zautomatizovať — bez záväzku.",
    au_cta_btn: "Opíšte nám svoj proces",

    im_kicker: "Dopad",
    im_title: "Koľko mravčej práce sme zautomatizovali",
    im_intro: "Reálne čísla z nasadených riešení — metriky z produkčného prostredia, nie odhady.",
    impact: [
        Trio { a: "12 000+", b: "hodín / rok", c: "Manuálnej práce nahradené automatizovanými AI workflow — prepisovanie, triedenie, klasifikácia." },
        Trio { a: "87%", b: "rýchlejšie spracovanie", c: "Priemerné zrýchlenie spracovania dopytov, faktúr a interných požiadaviek oproti manuálnemu procesu." },
        Trio { a: "340 000+", b: "dokumentov spracovaných", c: "Faktúry, zmluvy, e-maily a interné dokumenty cez naše AI pipeliny bez ručného zásahu." },
        Trio { a: "4.2s", b: "priemerný čas odpovede", c: "Od prijatia dopytu po hotový návrh odpovede. Interný AI asistent odpovedá rýchlejšie než kolega na Slacku." },
    ],

    pr_kicker: "Proces",
    pr_title: "Delivery proces",
    pr_intro: "Jasný backlog, rýchle rozhodovanie, pravidelné demo výstupy a predvídateľné releasy. Žiadne prekvapenia.",
    steps: [
        Pair { a: "Discovery sprint", b: "Mapujeme ciele, riziká, metriky úspechu a navrhujeme cieľový operating model projektu." },
        Pair { a: "Build sprinty", b: "Implementácia inkrementálne, priebežné testovanie a prepájanie s internými systémami." },
        Pair { a: "Go-live a hypercare", b: "Nasadenie, stabilizácia, monitoring a transfer know-how na interný tím klienta." },
        Pair { a: "Continuous improvement", b: "Optimalizácia podľa reálnych dát z produkcie, nie podľa predpokladov z úvodnej fázy." },
    ],

    st_kicker: "Technológie",
    st_title: "Tech stack",
    st_intro: "Technológie vyberáme podľa projektu, nie podľa trendov. Toto je stack s produkčnými skúsenosťami.",
    st_groups: ["Backend", "Frontend & API", "Infraštruktúra", "Dáta & Messaging", "AI & Integrácie"],

    faq_kicker: "Prečo my",
    faq_title: "Prečo Smart Dawn",
    faq: [
        Pair { a: "Enterprise disciplína + AI rýchlosť", b: "Roky práce na komplexných projektoch v bankovníctve, telco a korporáte. Vieme, čo enterprise vyžaduje — bezpečnosť, stabilitu a predvídateľné dodávky — a spájame to s rýchlosťou agentovej AI." },
        Pair { a: "Architektúra aj delivery v jednom tíme", b: "Navrhujeme aj staviame. Jasná architektúra, realistický plán dodávky a technické rozhodnutia, ktoré dávajú zmysel dlhodobo — nie len na papieri." },
        Pair { a: "Vlastný výskum, nie len integrácie", b: "Projekty ako Roman Empire, Voice či Forge sú náš vlastný R&D. Na klientske dodávky prinášame veci, ktoré sme si najprv overili na sebe." },
        Pair { a: "Viete dodať len časť riešenia?", b: "Áno. Vieme dodať aj samostatný scope: iba frontend, iba AI pipeline, iba integráciu alebo infra komponenty — podľa toho, čo práve potrebujete." },
    ],

    dawn_pre: "Smart Dawn — ",
    dawn_em: "nový úsvit",
    dawn_post: " pre každý softvér, ktorý od nás dostanete.",

    ct_kicker: "Kontakt",
    ct_title: "Posuňme váš projekt ďalej",
    ct_p: "Popíšte cieľ a aktuálny stav. Či potrebujete celý projekt alebo len konkrétny modul — do 48 hodín dostanete návrh ďalšieho postupu.",
    f_name: "Meno a priezvisko",
    f_name_ph: "Ján Novák",
    f_email: "E-mail",
    f_email_ph: "jan@firma.sk",
    f_company: "Firma",
    f_company_ph: "Voliteľné",
    f_message: "Správa",
    f_message_ph: "Popíšte váš projekt, cieľ alebo konkrétnu potrebu...",
    form_note: "Použitím formulára súhlasíte so spracovaním údajov na účely odpovede na dopyt.",
    submit: "Odoslať dopyt",

    footer_desc: "AI project delivery — agentové systémy, hlasové rozhrania a LLM pipeliny postavené s enterprise disciplínou.",
    footer_tag_pre: "Nový ",
    footer_tag_em: "úsvit",
    footer_tag_post: " pre váš softvér.",
    footer_links: ["Práca", "Služby", "Proces", "Kontakt", "E-mail"],
    footer_copy: "© 2026 Smart Dawn. Postavené v Ruste — Leptos SSR + WebAssembly.",

    thanks_title: "Ďakujeme za dopyt",
    thanks_msg: "Správa bola úspešne odoslaná. Smart Dawn tím sa ozve do 48 hodín s návrhom ďalšieho postupu.",
    err_title: "Formulár obsahuje chyby",
    err_msg: "Opravte nasledujúce položky a skúste to znova.",
    notfound_title: "Stránka sa nenašla",
    notfound_msg: "Táto adresa neexistuje alebo bola presunutá.",
    back_home: "Späť na hlavnú stránku",
    val_name_short: "Meno je príliš krátke.",
    val_name_long: "Meno je príliš dlhé.",
    val_email: "E-mail nemá správny formát.",
    val_msg_short: "Správa je príliš krátka (minimum 20 znakov).",
    val_msg_long: "Správa je príliš dlhá.",
    val_company_long: "Názov firmy je príliš dlhý.",
};

pub static EN: Dict = Dict {
    html_lang: "en",
    meta_title: "Smart Dawn | Complete custom software delivery",
    meta_desc: "Custom software from discovery and design through development and testing to launch and support. Web products, business systems, integrations and AI.",

    nav: ["Work", "Services", "Automations", "Process", "Tech", "Contact"],
    cta_start: "Start a project",
    skip: "Skip to content",
    brand_sub: "SOFTWARE DELIVERY",
    lang_label: "Language",

    hero_eyebrow: "Complete custom software delivery",
    hero_pre: "Your idea. Our code. ",
    hero_accent: "A new dawn.",
    hero_lead: "From the first conversation to a live product. We design, build and test your software, connect it to your business and support its continued growth.",
    hero_cta1: "Shape your project",
    hero_cta2: "Let’s talk",

    domains_kicker: "No business is foreign to us",
    domains_lead: "We grasp your domain faster than you'd expect — and build software in it. From regulated enterprise to fast-moving digital products.",
    domains: &[
        "Online casino", "Banking", "Telco", "E-commerce", "Fintech",
        "Insurance", "Healthcare", "Logistics", "Manufacturing", "Gaming",
        "Public sector", "Energy",
    ],

    themes_kicker: "What we build",
    themes_title: "Seven capabilities. Each one proven on a project of our own.",
    themes_intro: "We don't just claim we can do it — we show it. Behind every capability is real software built at Smart Dawn as proof, not a slide in a deck.",
    proof_label: "Proof from practice",
    themes: [
        ThemeTxt {
            kicker: "Agentic systems",
            title: "Legions of agents that deliver",
            desc: "Multiple AI agents that collaborate, plan, check one another and see the task through to the end — with human approval, memory, tools and guardrails. Not a single chatbot, but a whole team with responsibilities.",
            proof_tagline: "A legion of 12+ AI agents that ships software",
        },
        ThemeTxt {
            kicker: "Voice & realtime interfaces",
            title: "AI that listens and speaks",
            desc: "Wake-word, speech transcription, response and voice synthesis in real time, with natural interruption. Voice agents for call centers, assistants or live meeting transcription — local or in the cloud, with latency under 300 ms.",
            proof_tagline: "A voice agent that runs on your machine",
        },
        ThemeTxt {
            kicker: "AI testing & QA",
            title: "Tests that write themselves",
            desc: "Generating and running tests from OpenAPI or natural language, cross-system verification (HTTP + database + queues), OWASP security scanning and regression tracking. QA that keeps pace with development instead of slowing it down.",
            proof_tagline: "An autonomous API tester",
        },
        ThemeTxt {
            kicker: "Architecture on autopilot",
            title: "Diagrams and documentation from text",
            desc: "A description turns into a UML, BPMN or ArchiMate diagram plus architecture documentation following arc42, C4 and TOGAF — and it stays in sync with the code. An architect's second opinion, whenever you need it.",
            proof_tagline: "An AI architecture generator",
        },
        ThemeTxt {
            kicker: "Data & AI pipelines",
            title: "Data that tidies itself",
            desc: "Ingest from any source, translation, enrichment via vision AI and delivery into the target system — with auditing and cost control. Thousands of items with no manual retyping or copy-paste.",
            proof_tagline: "AI product-data ingest",
        },
        ThemeTxt {
            kicker: "Digital twins & avatars",
            title: "An avatar that talks back",
            desc: "A real-time talking AI avatar — it answers by voice, with frame-accurate lip-sync and latency under 500 ms, bilingual. Scan a logo and a 3D avatar comes to life in the browser as a living business card. By the way — the founder video above was generated by this very engine.",
            proof_tagline: "A real-time voice avatar (Gemini Live)",
        },
        ThemeTxt {
            kicker: "AI ops assistants",
            title: "An assistant that keeps the company running",
            desc: "AI that triages and answers email, tracks tasks, reviews merge requests in GitLab, transcribes and summarizes meetings, and ties it all together across Telegram, Slack and Notion. Back-office that runs even when you don't.",
            proof_tagline: "An AI email & ops assistant",
        },
    ],

    iv_kicker: "Who's behind it",
    iv_role: "Founder · Solution Architect & Tech Lead",
    iv_p1: "\"I don't just design on a whiteboard — I build.\" For more than 10 years he has built high-performance enterprise platforms in banking, telco and corporate environments, where stability meets startup speed.",
    iv_p2: "Today he combines that enterprise discipline with agentic AI — Smart Dawn's own R&D is grounded in production-ready agentic systems, not demos.",
    iv_creds: ["RAG & Agentic AI · IBM", "Retrieval-Augmented Generation · DeepLearning.AI", "Bratislava, SK"],

    sv_kicker: "Services",
    sv_title: "From analysis to operations",
    sv_intro: "We deliver the entire project or a specific module — research, frontend, a microservice, an AI pipeline or an integration. Always owning delivery all the way to production.",
    sv_cards: [
        Pair { a: "Discovery and architecture", b: "Business analysis, technical architecture, a roadmap and a budget in real milestones. We know where the risks are before the first line of code." },
        Pair { a: "AI and agentic systems", b: "Production-grade RAG, agentic workflows, voice and multimodal interfaces. With guardrails, human oversight and auditing — not a black box." },
        Pair { a: "Operations and growth", b: "SLA support, observability, automated tests and security reviews. Continuous improvement driven by production data, not by technical debt." },
    ],

    mk_kicker: "Runtime platform",
    mk_title: "One layer that runs anything",
    mk_intro: "An isolated runtime that spins up any task on demand as a standalone pod — tests, integrations, data jobs, AI agents, even an entire marketplace. It creates the task through a queue, monitors its resources and tears it down once it's done. Plug the same layer into any business. Runs on Kubernetes, OpenShift and plain Docker — in the cloud and on-premise.",
    mk_chips: &[
        "Kubernetes", "OpenShift", "Docker", "RabbitMQ control plane",
        "Ephemeral & isolated", "Resource monitoring", "Auto-cleanup", "SaaS and on-premise",
    ],

    au_kicker: "AI automations",
    au_title: "If you do it over and over, we can automate it",
    au_intro: "We don't have a fixed price list of off-the-shelf services. Does a process have an input, rules and an output? We'll build AI around it — with human oversight where it matters. Below are a few examples from practice, not the ceiling of what we can do.",
    use_cases: [
        UseCaseTxt { title: "Emails and inquiries", desc: "AI reads the incoming email, recognizes its type, pulls out the key details and drafts a reply. A person just reviews and sends.", flow: &["Email arrives", "AI classifies", "Data extraction", "Draft reply", "Approval"], result: "From 15 minutes to 2 minutes per inquiry" },
        UseCaseTxt { title: "Invoices, contracts and documents", desc: "Upload a PDF or scan — AI reads the content, recognizes the document type, pulls out amounts, dates and the supplier, and writes the data straight into your system.", flow: &["Document arrives", "AI reads content", "Data extraction", "Write to system"], result: "Up to 90% less manual retyping" },
        UseCaseTxt { title: "Images and videos", desc: "AI analyzes photos or video — recognizes products, reads text from images, spots defects or classifies content. The output goes into a report or triggers the next step.", flow: &["Media input", "AI analysis", "Classification", "Report / alert"], result: "Checks in seconds instead of hours" },
        UseCaseTxt { title: "Internal AI assistant", desc: "An employee asks — AI searches company documentation, internal procedures and rules, and answers with a link to the source. Fewer questions for colleagues, faster onboarding.", flow: &["Question", "Searches documentation", "Answer + source"], result: "Onboarding new people 3x faster" },
        UseCaseTxt { title: "Data transformation and integration", desc: "Need to get data from one system into another in a different format? AI understands the structure, remaps fields, fills in missing values and delivers the output where it belongs.", flow: &["Input data", "AI transformation", "Validation", "Output to target"], result: "The end of manual copy-paste between systems" },
    ],
    au_cases: [
        Trio { a: "Always in control", b: "No black box", c: "Every solution has clear rules — who approves what, what gets logged and where a human steps into the process. The whole time you have full visibility into what the AI did and why." },
        Trio { a: "Runs on yours", b: "We connect to what you already have", c: "Email, CRM, ERP, SharePoint, internal tools — we work with your data in your environment. No migration, no vendor lock-in." },
        Trio { a: "AI isn't mandatory", b: "Even without AI, if that's what you want", c: "Not an AI fan? No problem. We can automate plenty of processes the classic way too — deterministic rules, integrations and scripts with no model. We deploy AI only where it genuinely adds value, not for show." },
    ],
    au_cta_strong: "Don't see your process here? That's often the best news.",
    au_cta_p: "It's exactly the unusual, \"only we do it this way\" processes that tend to have the biggest payoff. Describe one to us and we'll tell you straight away whether and how it can be automated — no commitment.",
    au_cta_btn: "Tell us about your process",

    im_kicker: "Impact",
    im_title: "How much grunt work we've automated",
    im_intro: "Real numbers from deployed solutions — metrics from production, not estimates.",
    impact: [
        Trio { a: "12 000+", b: "hours / year", c: "Manual work replaced by automated AI workflows — retyping, sorting, classification." },
        Trio { a: "87%", b: "faster processing", c: "Average speed-up in processing inquiries, invoices and internal requests compared to a manual process." },
        Trio { a: "340 000+", b: "documents processed", c: "Invoices, contracts, emails and internal documents through our AI pipelines with no manual intervention." },
        Trio { a: "4.2s", b: "average response time", c: "From receiving an inquiry to a finished draft reply. The internal AI assistant answers faster than a colleague on Slack." },
    ],

    pr_kicker: "Process",
    pr_title: "Delivery process",
    pr_intro: "A clear backlog, fast decisions, regular demo outputs and predictable releases. No surprises.",
    steps: [
        Pair { a: "Discovery sprint", b: "We map goals, risks, success metrics and propose the project's target operating model." },
        Pair { a: "Build sprints", b: "Incremental implementation, continuous testing and integration with internal systems." },
        Pair { a: "Go-live and hypercare", b: "Deployment, stabilization, monitoring and knowledge transfer to the client's internal team." },
        Pair { a: "Continuous improvement", b: "Optimization based on real production data, not on assumptions from the initial phase." },
    ],

    st_kicker: "Technologies",
    st_title: "Tech stack",
    st_intro: "We choose technologies by the project, not by trends. This is a stack with production experience.",
    st_groups: ["Backend", "Frontend & API", "Infrastructure", "Data & Messaging", "AI & Integrations"],

    faq_kicker: "Why us",
    faq_title: "Why Smart Dawn",
    faq: [
        Pair { a: "Enterprise discipline + AI speed", b: "Years of work on complex projects in banking, telco and corporate. We know what enterprise demands — security, stability and predictable delivery — and we pair it with the speed of agentic AI." },
        Pair { a: "Architecture and delivery in one team", b: "We design and we build. Clear architecture, a realistic delivery plan and technical decisions that make sense in the long run — not just on paper." },
        Pair { a: "Our own research, not just integrations", b: "Projects like Roman Empire, Voice and Forge are our own R&D. We bring to client work the things we've first proven on ourselves." },
        Pair { a: "Can you deliver just part of a solution?", b: "Yes. We can also deliver a standalone scope: just the frontend, just the AI pipeline, just the integration or infra components — whatever you need right now." },
    ],

    dawn_pre: "Smart Dawn — ",
    dawn_em: "a new dawn",
    dawn_post: " for every piece of software you get from us.",

    ct_kicker: "Contact",
    ct_title: "Let's move your project forward",
    ct_p: "Describe the goal and where things stand today. Whether you need a whole project or just a specific module — within 48 hours you'll get a proposal for the next steps.",
    f_name: "Full name",
    f_name_ph: "John Smith",
    f_email: "Email",
    f_email_ph: "john@company.com",
    f_company: "Company",
    f_company_ph: "Optional",
    f_message: "Message",
    f_message_ph: "Describe your project, goal or specific need...",
    form_note: "By using this form you agree to the processing of your data for the purpose of responding to your inquiry.",
    submit: "Send inquiry",

    footer_desc: "AI project delivery — agentic systems, voice interfaces and LLM pipelines built with enterprise discipline.",
    footer_tag_pre: "New ",
    footer_tag_em: "dawn",
    footer_tag_post: " for your software.",
    footer_links: ["Work", "Services", "Process", "Contact", "Email"],
    footer_copy: "© 2026 Smart Dawn. Built in Rust — Leptos SSR + WebAssembly.",

    thanks_title: "Thanks for reaching out",
    thanks_msg: "Your message was sent successfully. The Smart Dawn team will get back to you within 48 hours with a proposal for the next steps.",
    err_title: "The form contains errors",
    err_msg: "Please fix the following fields and try again.",
    notfound_title: "Page not found",
    notfound_msg: "This address doesn't exist or has been moved.",
    back_home: "Back to the homepage",
    val_name_short: "The name is too short.",
    val_name_long: "The name is too long.",
    val_email: "The email isn't in a valid format.",
    val_msg_short: "The message is too short (minimum 20 characters).",
    val_msg_long: "The message is too long.",
    val_company_long: "The company name is too long.",
};

pub static CS: Dict = Dict {
    html_lang: "cs",
    meta_title: "Smart Dawn | Kompletní dodávky softwaru na míru",
    meta_desc: "Software na míru od analýzy a návrhu přes vývoj a testování až po nasazení a podporu. Webové produkty, firemní systémy, integrace a AI.",

    nav: ["Práce", "Služby", "Automatizace", "Proces", "Tech", "Kontakt"],
    cta_start: "Začít projekt",
    skip: "Přeskočit na obsah",
    brand_sub: "SOFTWARE DELIVERY",
    lang_label: "Jazyk",

    hero_eyebrow: "Kompletní dodávky softwaru na míru",
    hero_pre: "Váš nápad. Náš kód. ",
    hero_accent: "Nový začátek.",
    hero_lead: "Od prvního rozhovoru po nasazený produkt. Navrhneme, vyvineme a otestujeme váš software. Propojíme ho s vaším byznysem a postaráme se o jeho další rozvoj.",
    hero_cta1: "Navrhněme váš projekt",
    hero_cta2: "Pojďme si promluvit",

    domains_kicker: "Žádný byznys nám není cizí",
    domains_lead: "Doménu pochopíme rychleji, než čekáte — a postavíme v ní software. Od regulovaného enterprise po rychlé digitální produkty.",
    domains: &[
        "Online casino", "Bankovnictví", "Telco", "E-commerce", "Fintech",
        "Pojišťovnictví", "Healthcare", "Logistika", "Výroba", "Gaming",
        "Veřejný sektor", "Energetika",
    ],

    themes_kicker: "Co stavíme",
    themes_title: "Sedm schopností. Každou máme ověřenou na vlastním projektu.",
    themes_intro: "Neříkáme, že to umíme — ukazujeme. Za každou schopností je reálný software postavený v Smart Dawn jako důkaz, ne slajd v prezentaci.",
    proof_label: "Důkaz z praxe",
    themes: [
        ThemeTxt {
            kicker: "Agentové systémy",
            title: "Legie agentů, které dodávají",
            desc: "Více AI agentů, kteří spolupracují, plánují, vzájemně se kontrolují a dotáhnou úkol do konce — s lidským schválením, pamětí, nástroji a guardrails. Ne jeden chatbot, ale celý tým s odpovědnostmi.",
            proof_tagline: "Legie 12+ AI agentů, která dodá software",
        },
        ThemeTxt {
            kicker: "Hlasová & realtime rozhraní",
            title: "AI, která naslouchá a mluví",
            desc: "Wake-word, přepis řeči, odpověď a syntéza hlasu v reálném čase, s přirozeným přerušováním. Hlasoví agenti pro call centra, asistenty či živý přepis porad — lokálně i v cloudu, s latencí pod 300 ms.",
            proof_tagline: "Hlasový agent, co běží u tebe",
        },
        ThemeTxt {
            kicker: "AI testování & QA",
            title: "Testy, které se napíšou samy",
            desc: "Generování a běh testů z OpenAPI nebo přirozeného jazyka, cross-system verifikace (HTTP + databáze + fronty), OWASP bezpečnostní sken a sledování regresí. QA, které drží krok s vývojem, ne ho brzdí.",
            proof_tagline: "Autonomní tester API",
        },
        ThemeTxt {
            kicker: "Architektura na autopilota",
            title: "Diagramy a dokumentace z textu",
            desc: "Z popisu vznikne UML, BPMN či ArchiMate diagram i architektonická dokumentace podle arc42, C4 a TOGAF — a udržuje se spolu s kódem. Druhý názor architekta, kdykoli ho potřebujete.",
            proof_tagline: "AI generátor architektury",
        },
        ThemeTxt {
            kicker: "Data & AI pipeliny",
            title: "Data, která se uklidí sama",
            desc: "Ingest z jakéhokoli zdroje, překlad, obohacení přes vision AI a doručení do cílového systému — s auditem a kontrolou nákladů. Tisíce položek bez ručního přepisování a copy-paste.",
            proof_tagline: "AI ingest produktových dat",
        },
        ThemeTxt {
            kicker: "Digitální dvojníci & avataři",
            title: "Avatar, který s vámi mluví",
            desc: "Mluvící AI avatar v reálném čase — odpovídá hlasem, s přesnou synchronizací rtů a latencí pod 500 ms, dvojjazyčně. Naskenujete logo a v prohlížeči ožije 3D avatar jako živá vizitka. Mimochodem — video se zakladatelem výše vygeneroval přesně tento engine.",
            proof_tagline: "Real-time hlasový avatar (Gemini Live)",
        },
        ThemeTxt {
            kicker: "AI asistenti pro operativu",
            title: "Asistent, který drží firmu v chodu",
            desc: "AI, která třídí a odpovídá na e-maily, sleduje úkoly, dělá code review v GitLabu, přepisuje a sumarizuje porady a vše propojuje přes Telegram, Slack a Notion. Operativa, která běží, i když vy ne.",
            proof_tagline: "AI e-mailový a ops asistent",
        },
    ],

    iv_kicker: "Kdo za tím stojí",
    iv_role: "Zakladatel · Solution Architect & Tech Lead",
    iv_p1: "„Nenavrhuji jen na whiteboardu — stavím.“ Více než 10 let staví vysoce výkonné enterprise platformy v bankovnictví, telco a korporátním prostředí, kde se stabilita setkává s rychlostí startupu.",
    iv_p2: "Dnes spojuje tuto enterprise disciplínu s agentovou AI — vlastní R&D Smart Dawn stojí na produkčně použitelných agentových systémech, ne na demech.",
    iv_creds: ["RAG & Agentic AI · IBM", "Retrieval-Augmented Generation · DeepLearning.AI", "Bratislava, SK"],

    sv_kicker: "Služby",
    sv_title: "Od analýzy po provoz",
    sv_intro: "Dodáme celý projekt nebo konkrétní modul — výzkum, frontend, mikroslužbu, AI pipeline nebo integraci. Vždy s vlastnictvím dodávky až do produkce.",
    sv_cards: [
        Pair { a: "Discovery a architektura", b: "Business analýza, technická architektura, roadmapa a rozpočet v reálných milnících. Víme, kde jsou rizika ještě před prvním řádkem kódu." },
        Pair { a: "AI a agentové systémy", b: "Produkční RAG, agentic workflow, hlasová a multimodální rozhraní. S guardrails, lidskou kontrolou a auditem — ne černá skříňka." },
        Pair { a: "Provoz a rozvoj", b: "SLA podpora, observability, automatizované testy a bezpečnostní revize. Kontinuální rozvoj podle dat z produkce, ne podle technického dluhu." },
    ],

    mk_kicker: "Běhová platforma",
    mk_title: "Jedna vrstva, na které běží cokoli",
    mk_intro: "Izolovaný runtime, který na požádání spustí libovolnou úlohu jako samostatný pod — testy, integrace, datové joby, AI agenty, i celý marketplace. Přes frontu ho vytvoří, sleduje jeho zdroje a po doběhnutí ho zruší. Tutéž vrstvu zapojíte do jakéhokoli byznysu. Běží na Kubernetes, OpenShift i v čistém Dockeru — v cloudu i on-premise.",
    mk_chips: &[
        "Kubernetes", "OpenShift", "Docker", "RabbitMQ control plane",
        "Ephemeral & izolované", "Monitoring zdrojů", "Auto-cleanup", "SaaS i on-premise",
    ],

    au_kicker: "AI automatizace",
    au_title: "Pokud to děláte opakovaně, umíme to zautomatizovat",
    au_intro: "Nemáme fixní ceník hotových služeb. Má proces vstup, pravidla a výstup? Postavíme kolem něj AI — s lidskou kontrolou tam, kde na tom záleží. Níže je pár příkladů z praxe, ne strop toho, co zvládneme.",
    use_cases: [
        UseCaseTxt { title: "E-maily a poptávky", desc: "AI přečte příchozí e-mail, rozpozná typ, vytáhne klíčové údaje a připraví návrh odpovědi. Člověk jen zkontroluje a odešle.", flow: &["E-mail přijde", "AI klasifikuje", "Extrakce údajů", "Návrh odpovědi", "Schválení"], result: "Z 15 minut na 2 minuty na poptávku" },
        UseCaseTxt { title: "Faktury, smlouvy a dokumenty", desc: "Nahrajete PDF nebo sken — AI přečte obsah, rozpozná typ dokumentu, vytáhne částky, data a dodavatele a zapíše údaje přímo do vašeho systému.", flow: &["Dokument přijde", "AI přečte obsah", "Extrakce dat", "Zápis do systému"], result: "Až 90% méně manuálního přepisování" },
        UseCaseTxt { title: "Obrázky a videa", desc: "AI analyzuje fotky nebo video — rozpozná produkty, přečte text z obrázků, identifikuje chyby nebo klasifikuje obsah. Výstup jde do reportu nebo spouští další krok.", flow: &["Vstup média", "AI analýza", "Klasifikace", "Report / alert"], result: "Kontrola v sekundách místo hodin" },
        UseCaseTxt { title: "Interní AI asistent", desc: "Zaměstnanec se zeptá — AI prohledá firemní dokumentaci, interní postupy a pravidla a odpoví i s odkazem na zdroj. Méně otázek na kolegy, rychlejší onboarding.", flow: &["Otázka", "Prohledá dokumentaci", "Odpověď + zdroj"], result: "Onboarding nových lidí 3x rychlejší" },
        UseCaseTxt { title: "Transformace a propojování dat", desc: "Data z jednoho systému je třeba dostat do druhého v jiném formátu? AI rozumí struktuře, přemapuje pole, doplní chybějící údaje a doručí výstup, kam je třeba.", flow: &["Vstupní data", "AI transformace", "Validace", "Výstup do cíle"], result: "Konec ručního copy-paste mezi systémy" },
    ],
    au_cases: [
        Trio { a: "Vždy pod kontrolou", b: "Žádná černá skříňka", c: "Každé řešení má jasná pravidla — kdo co schvaluje, co se loguje a kde do procesu vstupuje člověk. Po celou dobu máte přehled o tom, co AI udělala a proč." },
        Trio { a: "Běží na vašem", b: "Napojíme se na to, co máte", c: "E-mail, CRM, ERP, SharePoint, interní nástroje — pracujeme s vašimi daty ve vašem prostředí. Žádná migrace, žádný vendor lock-in." },
        Trio { a: "AI není povinnost", b: "I bez AI, když tak chcete", c: "Nejste fanoušek AI? Nevadí. Spoustu procesů umíme zautomatizovat i klasicky — deterministická pravidla, integrace a skripty bez modelu. AI nasadíme jen tam, kde reálně přidává hodnotu, ne pro efekt." },
    ],
    au_cta_strong: "Váš proces tu není? To je často nejlepší zpráva.",
    au_cta_p: "Právě ty netypické, „jen u nás se to dělá takhle“ procesy mívají největší návratnost. Popište nám jeden a rovnou vám řekneme, zda a jak se dá zautomatizovat — bez závazku.",
    au_cta_btn: "Popište nám svůj proces",

    im_kicker: "Dopad",
    im_title: "Kolik mravenčí práce jsme zautomatizovali",
    im_intro: "Reálná čísla z nasazených řešení — metriky z produkčního prostředí, ne odhady.",
    impact: [
        Trio { a: "12 000+", b: "hodin / rok", c: "Manuální práce nahrazené automatizovanými AI workflow — přepisování, třídění, klasifikace." },
        Trio { a: "87%", b: "rychlejší zpracování", c: "Průměrné zrychlení zpracování poptávek, faktur a interních požadavků oproti manuálnímu procesu." },
        Trio { a: "340 000+", b: "dokumentů zpracováno", c: "Faktury, smlouvy, e-maily a interní dokumenty přes naše AI pipeliny bez ručního zásahu." },
        Trio { a: "4.2s", b: "průměrný čas odpovědi", c: "Od přijetí poptávky po hotový návrh odpovědi. Interní AI asistent odpovídá rychleji než kolega na Slacku." },
    ],

    pr_kicker: "Proces",
    pr_title: "Delivery proces",
    pr_intro: "Jasný backlog, rychlé rozhodování, pravidelné demo výstupy a předvídatelné release. Žádná překvapení.",
    steps: [
        Pair { a: "Discovery sprint", b: "Mapujeme cíle, rizika, metriky úspěchu a navrhujeme cílový operating model projektu." },
        Pair { a: "Build sprinty", b: "Implementace inkrementálně, průběžné testování a propojování s interními systémy." },
        Pair { a: "Go-live a hypercare", b: "Nasazení, stabilizace, monitoring a transfer know-how na interní tým klienta." },
        Pair { a: "Continuous improvement", b: "Optimalizace podle reálných dat z produkce, ne podle předpokladů z úvodní fáze." },
    ],

    st_kicker: "Technologie",
    st_title: "Tech stack",
    st_intro: "Technologie vybíráme podle projektu, ne podle trendů. Toto je stack s produkčními zkušenostmi.",
    st_groups: ["Backend", "Frontend & API", "Infrastruktura", "Data & Messaging", "AI & Integrace"],

    faq_kicker: "Proč my",
    faq_title: "Proč Smart Dawn",
    faq: [
        Pair { a: "Enterprise disciplína + AI rychlost", b: "Roky práce na komplexních projektech v bankovnictví, telco a korporátu. Víme, co enterprise vyžaduje — bezpečnost, stabilitu a předvídatelné dodávky — a spojujeme to s rychlostí agentové AI." },
        Pair { a: "Architektura i delivery v jednom týmu", b: "Navrhujeme i stavíme. Jasná architektura, realistický plán dodávky a technická rozhodnutí, která dávají smysl dlouhodobě — ne jen na papíře." },
        Pair { a: "Vlastní výzkum, ne jen integrace", b: "Projekty jako Roman Empire, Voice či Forge jsou náš vlastní R&D. Na klientské dodávky přinášíme věci, které jsme si nejprve ověřili na sobě." },
        Pair { a: "Umíte dodat jen část řešení?", b: "Ano. Umíme dodat i samostatný scope: jen frontend, jen AI pipeline, jen integraci nebo infra komponenty — podle toho, co právě potřebujete." },
    ],

    dawn_pre: "Smart Dawn — ",
    dawn_em: "nový úsvit",
    dawn_post: " pro každý software, který od nás dostanete.",

    ct_kicker: "Kontakt",
    ct_title: "Posuňme váš projekt dál",
    ct_p: "Popište cíl a aktuální stav. Ať už potřebujete celý projekt nebo jen konkrétní modul — do 48 hodin dostanete návrh dalšího postupu.",
    f_name: "Jméno a příjmení",
    f_name_ph: "Jan Novák",
    f_email: "E-mail",
    f_email_ph: "jan@firma.cz",
    f_company: "Firma",
    f_company_ph: "Volitelné",
    f_message: "Zpráva",
    f_message_ph: "Popište váš projekt, cíl nebo konkrétní potřebu...",
    form_note: "Použitím formuláře souhlasíte se zpracováním údajů za účelem odpovědi na poptávku.",
    submit: "Odeslat poptávku",

    footer_desc: "AI project delivery — agentové systémy, hlasová rozhraní a LLM pipeliny postavené s enterprise disciplínou.",
    footer_tag_pre: "Nový ",
    footer_tag_em: "úsvit",
    footer_tag_post: " pro váš software.",
    footer_links: ["Práce", "Služby", "Proces", "Kontakt", "E-mail"],
    footer_copy: "© 2026 Smart Dawn. Postaveno v Rustu — Leptos SSR + WebAssembly.",

    thanks_title: "Děkujeme za poptávku",
    thanks_msg: "Zpráva byla úspěšně odeslána. Tým Smart Dawn se ozve do 48 hodin s návrhem dalšího postupu.",
    err_title: "Formulář obsahuje chyby",
    err_msg: "Opravte následující položky a zkuste to znovu.",
    notfound_title: "Stránka nebyla nalezena",
    notfound_msg: "Tato adresa neexistuje nebo byla přesunuta.",
    back_home: "Zpět na hlavní stránku",
    val_name_short: "Jméno je příliš krátké.",
    val_name_long: "Jméno je příliš dlouhé.",
    val_email: "E-mail nemá správný formát.",
    val_msg_short: "Zpráva je příliš krátká (minimum 20 znaků).",
    val_msg_long: "Zpráva je příliš dlouhá.",
    val_company_long: "Název firmy je příliš dlouhý.",
};

pub static DE: Dict = Dict {
    html_lang: "de",
    meta_title: "Smart Dawn | Individuelle Software von der Idee bis zum Betrieb",
    meta_desc: "Maßgeschneiderte Software von Analyse und Entwurf über Entwicklung und Tests bis zu Einführung und Support. Webprodukte, Unternehmenssysteme, Integrationen und KI.",

    nav: ["Arbeit", "Leistungen", "Automatisierungen", "Prozess", "Tech", "Kontakt"],
    cta_start: "Projekt starten",
    skip: "Zum Inhalt springen",
    brand_sub: "SOFTWARE DELIVERY",
    lang_label: "Sprache",

    hero_eyebrow: "Individuelle Software aus einer Hand",
    hero_pre: "Ihre Idee. Unser Code. ",
    hero_accent: "Ein neuer Anfang.",
    hero_lead: "Vom ersten Gespräch bis zum fertigen Produkt. Wir entwerfen, entwickeln und testen Ihre Software, integrieren sie in Ihr Unternehmen und begleiten ihre Weiterentwicklung.",
    hero_cta1: "Ihr Projekt gestalten",
    hero_cta2: "Sprechen wir darüber",

    domains_kicker: "Keine Branche ist uns fremd",
    domains_lead: "Wir verstehen Ihre Domäne schneller, als Sie erwarten — und bauen darin Software. Vom regulierten Enterprise bis zum schnellen digitalen Produkt.",
    domains: &[
        "Online-Casino", "Banking", "Telco", "E-Commerce", "Fintech",
        "Versicherung", "Healthcare", "Logistik", "Fertigung", "Gaming",
        "Öffentlicher Sektor", "Energie",
    ],

    themes_kicker: "Was wir bauen",
    themes_title: "Sieben Fähigkeiten. Jede an einem eigenen Projekt erprobt.",
    themes_intro: "Wir behaupten es nicht — wir zeigen es. Hinter jeder Fähigkeit steht echte Software, gebaut bei Smart Dawn als Beweis, nicht als Folie in einer Präsentation.",
    proof_label: "Beweis aus der Praxis",
    themes: [
        ThemeTxt {
            kicker: "Agentensysteme",
            title: "Legionen von Agenten, die liefern",
            desc: "Mehrere KI-Agenten, die zusammenarbeiten, planen, sich gegenseitig prüfen und die Aufgabe zu Ende bringen — mit menschlicher Freigabe, Gedächtnis, Werkzeugen und Guardrails. Kein einzelner Chatbot, sondern ein ganzes Team mit klaren Verantwortlichkeiten.",
            proof_tagline: "Eine Legion aus 12+ KI-Agenten, die Software liefert",
        },
        ThemeTxt {
            kicker: "Voice- & Realtime-Schnittstellen",
            title: "KI, die zuhört und spricht",
            desc: "Wake-Word, Spracherkennung, Antwort und Sprachsynthese in Echtzeit, mit natürlichem Unterbrechen. Voice-Agenten für Callcenter, Assistenten oder Live-Transkription von Meetings — lokal und in der Cloud, mit einer Latenz unter 300 ms.",
            proof_tagline: "Ein Voice-Agent, der bei Ihnen läuft",
        },
        ThemeTxt {
            kicker: "KI-Testing & QA",
            title: "Tests, die sich selbst schreiben",
            desc: "Generierung und Ausführung von Tests aus OpenAPI oder natürlicher Sprache, systemübergreifende Verifikation (HTTP + Datenbank + Queues), OWASP-Sicherheitsscan und Regressionsverfolgung. QA, die mit der Entwicklung Schritt hält, statt sie zu bremsen.",
            proof_tagline: "Ein autonomer API-Tester",
        },
        ThemeTxt {
            kicker: "Architektur auf Autopilot",
            title: "Diagramme und Dokumentation aus Text",
            desc: "Aus einer Beschreibung entstehen UML-, BPMN- oder ArchiMate-Diagramme sowie Architekturdokumentation nach arc42, C4 und TOGAF — und sie wird gemeinsam mit dem Code aktuell gehalten. Die zweite Meinung eines Architekten, wann immer Sie sie brauchen.",
            proof_tagline: "Ein KI-Generator für Architektur",
        },
        ThemeTxt {
            kicker: "Daten- & KI-Pipelines",
            title: "Daten, die sich selbst aufräumen",
            desc: "Ingest aus beliebigen Quellen, Übersetzung, Anreicherung per Vision-KI und Auslieferung in das Zielsystem — mit Audit und Kostenkontrolle. Tausende Einträge ohne manuelles Abtippen und Copy-paste.",
            proof_tagline: "KI-Ingest von Produktdaten",
        },
        ThemeTxt {
            kicker: "Digitale Zwillinge & Avatare",
            title: "Ein Avatar, der mit Ihnen spricht",
            desc: "Ein sprechender KI-Avatar in Echtzeit — er antwortet per Stimme, mit bildgenauem Lippensync und einer Latenz unter 500 ms, zweisprachig. Scannen Sie ein Logo und ein 3D-Avatar erwacht im Browser als lebende Visitenkarte. Übrigens — das Gründer-Video oben hat genau diese Engine erzeugt.",
            proof_tagline: "Ein Echtzeit-Voice-Avatar (Gemini Live)",
        },
        ThemeTxt {
            kicker: "KI-Assistenten für den Betrieb",
            title: "Ein Assistent, der den Laden am Laufen hält",
            desc: "KI, die E-Mails sortiert und beantwortet, Aufgaben verfolgt, Merge Requests in GitLab reviewt, Meetings transkribiert und zusammenfasst und alles über Telegram, Slack und Notion verbindet. Backoffice, das läuft, auch wenn Sie es nicht tun.",
            proof_tagline: "Ein KI-E-Mail- und Ops-Assistent",
        },
    ],

    iv_kicker: "Wer dahintersteht",
    iv_role: "Gründer · Solution Architect & Tech Lead",
    iv_p1: "„Ich entwerfe nicht nur am Whiteboard — ich baue.“ Seit über 10 Jahren entwickelt er hochperformante Enterprise-Plattformen in Banking, Telco und Konzernumfeldern, wo Stabilität auf Startup-Tempo trifft.",
    iv_p2: "Heute verbindet er diese Enterprise-Disziplin mit agentischer KI — die eigene R&D von Smart Dawn baut auf produktionsreifen Agentensystemen auf, nicht auf Demos.",
    iv_creds: ["RAG & Agentic AI · IBM", "Retrieval-Augmented Generation · DeepLearning.AI", "Bratislava, SK"],

    sv_kicker: "Leistungen",
    sv_title: "Von der Analyse bis zum Betrieb",
    sv_intro: "Wir liefern das gesamte Projekt oder ein konkretes Modul — Forschung, Frontend, Microservice, KI-Pipeline oder Integration. Immer mit voller Verantwortung für die Auslieferung bis in die Produktion.",
    sv_cards: [
        Pair { a: "Discovery und Architektur", b: "Business-Analyse, technische Architektur, Roadmap und Budget in realen Meilensteinen. Wir wissen, wo die Risiken liegen — noch vor der ersten Zeile Code." },
        Pair { a: "KI- und Agentensysteme", b: "Produktionsreifes RAG, agentische Workflows, Voice- und multimodale Schnittstellen. Mit Guardrails, menschlicher Kontrolle und Audit — keine Blackbox." },
        Pair { a: "Betrieb und Weiterentwicklung", b: "SLA-Support, Observability, automatisierte Tests und Sicherheitsreviews. Kontinuierliche Weiterentwicklung anhand von Produktionsdaten, nicht anhand technischer Schulden." },
    ],

    mk_kicker: "Runtime-Plattform",
    mk_title: "Eine Schicht, auf der alles läuft",
    mk_intro: "Eine isolierte Runtime, die auf Anforderung jede beliebige Aufgabe als eigenständigen Pod startet — Tests, Integrationen, Daten-Jobs, KI-Agenten und sogar einen ganzen Marketplace. Über eine Queue wird er erzeugt, seine Ressourcen werden überwacht und nach Abschluss wird er wieder entfernt. Dieselbe Schicht binden Sie in jedes Business ein. Sie läuft auf Kubernetes, OpenShift und in reinem Docker — in der Cloud und on-premise.",
    mk_chips: &[
        "Kubernetes", "OpenShift", "Docker", "RabbitMQ Control Plane",
        "Ephemeral & isoliert", "Ressourcen-Monitoring", "Auto-Cleanup", "SaaS und on-premise",
    ],

    au_kicker: "KI-Automatisierungen",
    au_title: "Wenn Sie es wiederholt tun, können wir es automatisieren",
    au_intro: "Wir haben keine feste Preisliste fertiger Services. Hat ein Prozess Input, Regeln und Output? Dann bauen wir KI darum herum — mit menschlicher Kontrolle dort, wo es darauf ankommt. Unten finden Sie einige Beispiele aus der Praxis, nicht die Grenze dessen, was wir können.",
    use_cases: [
        UseCaseTxt { title: "E-Mails und Anfragen", desc: "Die KI liest eine eingehende E-Mail, erkennt den Typ, extrahiert die wichtigsten Daten und bereitet einen Antwortvorschlag vor. Ein Mensch prüft nur noch und sendet ab.", flow: &["E-Mail trifft ein", "KI klassifiziert", "Datenextraktion", "Antwortvorschlag", "Freigabe"], result: "Von 15 Minuten auf 2 Minuten pro Anfrage" },
        UseCaseTxt { title: "Rechnungen, Verträge und Dokumente", desc: "Sie laden ein PDF oder einen Scan hoch — die KI liest den Inhalt, erkennt den Dokumenttyp, extrahiert Beträge, Daten und Lieferanten und schreibt die Werte direkt in Ihr System.", flow: &["Dokument trifft ein", "KI liest den Inhalt", "Datenextraktion", "Eintrag ins System"], result: "Bis zu 90 % weniger manuelles Abtippen" },
        UseCaseTxt { title: "Bilder und Videos", desc: "Die KI analysiert Fotos oder Videos — erkennt Produkte, liest Text aus Bildern, identifiziert Fehler oder klassifiziert Inhalte. Das Ergebnis fließt in einen Report oder löst den nächsten Schritt aus.", flow: &["Medien-Input", "KI-Analyse", "Klassifikation", "Report / Alert"], result: "Prüfung in Sekunden statt Stunden" },
        UseCaseTxt { title: "Interner KI-Assistent", desc: "Ein Mitarbeiter fragt — die KI durchsucht die Unternehmensdokumentation, interne Abläufe und Regeln und antwortet samt Quellenangabe. Weniger Rückfragen an Kollegen, schnelleres Onboarding.", flow: &["Frage", "Durchsucht die Dokumentation", "Antwort + Quelle"], result: "Onboarding neuer Mitarbeiter 3x schneller" },
        UseCaseTxt { title: "Datentransformation und -verknüpfung", desc: "Daten müssen von einem System in ein anderes Format übertragen werden? Die KI versteht die Struktur, mappt die Felder neu, ergänzt fehlende Werte und liefert das Ergebnis dorthin, wo es gebraucht wird.", flow: &["Eingangsdaten", "KI-Transformation", "Validierung", "Output ins Ziel"], result: "Schluss mit manuellem Copy-paste zwischen Systemen" },
    ],
    au_cases: [
        Trio { a: "Immer unter Kontrolle", b: "Keine Blackbox", c: "Jede Lösung hat klare Regeln — wer was freigibt, was protokolliert wird und an welcher Stelle der Mensch in den Prozess eingreift. Sie behalten jederzeit den Überblick, was die KI getan hat und warum." },
        Trio { a: "Läuft bei Ihnen", b: "Wir docken an das an, was Sie haben", c: "E-Mail, CRM, ERP, SharePoint, interne Tools — wir arbeiten mit Ihren Daten in Ihrer Umgebung. Keine Migration, kein Vendor-Lock-in." },
        Trio { a: "KI ist keine Pflicht", b: "Auch ganz ohne KI, wenn Sie das möchten", c: "Kein Fan von KI? Kein Problem. Viele Prozesse können wir auch klassisch automatisieren — deterministische Regeln, Integrationen und Skripte ohne Modell. KI setzen wir nur dort ein, wo sie echten Mehrwert bringt, nicht des Effekts wegen." },
    ],
    au_cta_strong: "Ihr Prozess ist nicht dabei? Das ist oft die beste Nachricht.",
    au_cta_p: "Gerade die untypischen Prozesse nach dem Motto „das machen nur wir so“ haben häufig den größten Return. Beschreiben Sie uns einen — und wir sagen Ihnen direkt, ob und wie er sich automatisieren lässt, ganz unverbindlich.",
    au_cta_btn: "Beschreiben Sie uns Ihren Prozess",

    im_kicker: "Wirkung",
    im_title: "Wie viel Kleinarbeit wir automatisiert haben",
    im_intro: "Echte Zahlen aus produktiven Lösungen — Metriken aus der Produktionsumgebung, keine Schätzungen.",
    impact: [
        Trio { a: "12 000+", b: "Stunden / Jahr", c: "Manuelle Arbeit, ersetzt durch automatisierte KI-Workflows — Abtippen, Sortieren, Klassifizieren." },
        Trio { a: "87%", b: "schnellere Verarbeitung", c: "Durchschnittliche Beschleunigung bei der Bearbeitung von Anfragen, Rechnungen und internen Vorgängen gegenüber dem manuellen Prozess." },
        Trio { a: "340 000+", b: "verarbeitete Dokumente", c: "Rechnungen, Verträge, E-Mails und interne Dokumente durch unsere KI-Pipelines, ohne manuellen Eingriff." },
        Trio { a: "4.2s", b: "durchschnittliche Antwortzeit", c: "Von der eingehenden Anfrage bis zum fertigen Antwortvorschlag. Der interne KI-Assistent antwortet schneller als ein Kollege auf Slack." },
    ],

    pr_kicker: "Prozess",
    pr_title: "Delivery-Prozess",
    pr_intro: "Klares Backlog, schnelle Entscheidungen, regelmäßige Demo-Ergebnisse und planbare Releases. Keine Überraschungen.",
    steps: [
        Pair { a: "Discovery-Sprint", b: "Wir kartieren Ziele, Risiken und Erfolgskennzahlen und entwerfen das angestrebte Operating Model des Projekts." },
        Pair { a: "Build-Sprints", b: "Inkrementelle Implementierung, laufendes Testen und Anbindung an interne Systeme." },
        Pair { a: "Go-live und Hypercare", b: "Deployment, Stabilisierung, Monitoring und Wissenstransfer an das interne Team des Kunden." },
        Pair { a: "Continuous Improvement", b: "Optimierung anhand echter Produktionsdaten, nicht anhand von Annahmen aus der Anfangsphase." },
    ],

    st_kicker: "Technologien",
    st_title: "Tech-Stack",
    st_intro: "Wir wählen Technologien nach Projekt aus, nicht nach Trends. Das ist ein Stack mit Produktionserfahrung.",
    st_groups: ["Backend", "Frontend & API", "Infrastruktur", "Daten & Messaging", "KI & Integrationen"],

    faq_kicker: "Warum wir",
    faq_title: "Warum Smart Dawn",
    faq: [
        Pair { a: "Enterprise-Disziplin + KI-Tempo", b: "Jahre an Arbeit an komplexen Projekten in Banking, Telco und Konzernumfeld. Wir wissen, was Enterprise verlangt — Sicherheit, Stabilität und planbare Lieferungen — und verbinden das mit dem Tempo agentischer KI." },
        Pair { a: "Architektur und Delivery in einem Team", b: "Wir entwerfen und bauen zugleich. Klare Architektur, ein realistischer Lieferplan und technische Entscheidungen, die langfristig Sinn ergeben — nicht nur auf dem Papier." },
        Pair { a: "Eigene Forschung, nicht nur Integrationen", b: "Projekte wie Roman Empire, Voice oder Forge sind unsere eigene R&D. In Kundenprojekte bringen wir Dinge ein, die wir zuerst an uns selbst erprobt haben." },
        Pair { a: "Können Sie auch nur einen Teil der Lösung liefern?", b: "Ja. Wir liefern auch einen abgegrenzten Scope: nur das Frontend, nur die KI-Pipeline, nur die Integration oder Infra-Komponenten — ganz nach Ihrem aktuellen Bedarf." },
    ],

    dawn_pre: "Smart Dawn — ",
    dawn_em: "ein neuer Anbruch",
    dawn_post: " für jede Software, die Sie von uns bekommen.",

    ct_kicker: "Kontakt",
    ct_title: "Bringen wir Ihr Projekt voran",
    ct_p: "Beschreiben Sie Ihr Ziel und den aktuellen Stand. Ob Sie ein ganzes Projekt oder nur ein konkretes Modul brauchen — innerhalb von 48 Stunden erhalten Sie einen Vorschlag für das weitere Vorgehen.",
    f_name: "Vor- und Nachname",
    f_name_ph: "Max Mustermann",
    f_email: "E-Mail",
    f_email_ph: "max@firma.de",
    f_company: "Unternehmen",
    f_company_ph: "Optional",
    f_message: "Nachricht",
    f_message_ph: "Beschreiben Sie Ihr Projekt, Ihr Ziel oder Ihren konkreten Bedarf...",
    form_note: "Mit der Nutzung des Formulars stimmen Sie der Verarbeitung Ihrer Daten zur Beantwortung Ihrer Anfrage zu.",
    submit: "Anfrage senden",

    footer_desc: "AI project delivery — Agentensysteme, Sprachschnittstellen und LLM-Pipelines, gebaut mit Enterprise-Disziplin.",
    footer_tag_pre: "Ein neuer ",
    footer_tag_em: "Anbruch",
    footer_tag_post: " für Ihre Software.",
    footer_links: ["Arbeit", "Leistungen", "Prozess", "Kontakt", "E-Mail"],
    footer_copy: "© 2026 Smart Dawn. Gebaut in Rust — Leptos SSR + WebAssembly.",

    thanks_title: "Danke für Ihre Anfrage",
    thanks_msg: "Ihre Nachricht wurde erfolgreich gesendet. Das Smart Dawn Team meldet sich innerhalb von 48 Stunden mit einem Vorschlag für das weitere Vorgehen.",
    err_title: "Das Formular enthält Fehler",
    err_msg: "Bitte korrigieren Sie die folgenden Felder und versuchen Sie es erneut.",
    notfound_title: "Seite nicht gefunden",
    notfound_msg: "Diese Adresse existiert nicht oder wurde verschoben.",
    back_home: "Zurück zur Startseite",
    val_name_short: "Der Name ist zu kurz.",
    val_name_long: "Der Name ist zu lang.",
    val_email: "Die E-Mail-Adresse hat ein ungültiges Format.",
    val_msg_short: "Die Nachricht ist zu kurz (mindestens 20 Zeichen).",
    val_msg_long: "Die Nachricht ist zu lang.",
    val_company_long: "Der Firmenname ist zu lang.",
};
