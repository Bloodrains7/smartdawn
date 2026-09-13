//! Shared page copy by language. Portfolio and delivery copy live in their
//! respective modules. The URL prefix selects the language (`/`, `/en`, `/cz`, `/de`).

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
    meta_title: "Smart Dawn | Vývoj softvéru pre firmy",
    meta_desc: "Vyvíjame softvér pre firmy a prepájame ich systémy. Jarvis pomáha s operatívou, Cadence s plánovaním a Tally s výkazmi. Dodávame aj riešenia na mieru.",

    nav: ["Riešenia", "Služby", "Automatizácie", "Spolupráca", "Technológie", "Kontakt"],
    cta_start: "Napíšte nám",
    skip: "Preskočiť na obsah",
    brand_sub: "VÝVOJ SOFTVÉRU",
    lang_label: "Jazyk",

    hero_eyebrow: "Softvér na mieru a vlastné produkty",
    hero_pre: "Softvér pre ",
    hero_accent: "vašu firmu.",
    hero_lead: "Pomáhame firmám zjednodušiť prácu pomocou softvéru. Vyvinieme aplikáciu, prepojíme existujúce systémy alebo pripravíme nástroj na opakované úlohy. Postaráme sa aj o nasadenie a ďalší rozvoj.",
    hero_cta1: "Vybrať typ riešenia",
    hero_cta2: "Dohodnúť rozhovor",

    domains_kicker: "Oblasti, ktorým sa venujeme",
    domains_lead: "Pri návrhu vychádzame z toho, ako vaša firma pracuje. Zohľadníme jej procesy, používané systémy aj požiadavky odvetvia.",
    domains: &["Bankovníctvo", "Telekomunikácie", "Internetový predaj", "Finančné technológie", "Poisťovníctvo", "Zdravotníctvo", "Logistika", "Výroba", "Herný priemysel", "Verejný sektor", "Energetika"],

    iv_kicker: "O zakladateľovi",
    iv_role: "Zakladateľ a softvérový architekt",
    iv_p1: "Ondrej Luknár má viac než desať rokov skúseností s návrhom a vývojom softvéru pre banky, telekomunikačné spoločnosti a veľké firmy. Venuje sa technickému návrhu aj samotnej realizácii.",
    iv_p2: "V Smart Dawn vyvíja vlastné produkty aj zákazkové riešenia. Jarvis, Cadence a Tally vznikajú z praktických potrieb pri organizácii práce. Tieto skúsenosti využíva aj pri spolupráci so zákazníkmi.",
    iv_creds: ["RAG & Agentic AI · IBM", "Retrieval-Augmented Generation · DeepLearning.AI", "Bratislava, SK"],

    sv_kicker: "Služby",
    sv_title: "S čím vám pomôžeme",
    sv_intro: "Môžeme prevziať celý vývoj alebo sa pridať k vášmu tímu pri konkrétnej časti projektu. Rozsah práce a zodpovednosti si dohodneme na začiatku.",
    sv_cards: [
        Pair { a: "Návrh riešenia", b: "Prejdeme si vaše potreby, existujúce systémy a obmedzenia. Pripravíme technický návrh, postup prác a odhad nákladov." },
        Pair { a: "Vývoj a integrácie", b: "Vytvoríme aplikáciu, prepojíme systémy alebo doplníme AI funkcie. Súčasťou práce je testovanie aj overenie s vaším tímom." },
        Pair { a: "Nasadenie a podpora", b: "Zabezpečíme nasadenie, dohľad nad prevádzkou a riešenie chýb. Podporu a ďalší rozvoj prispôsobíme tomu, čo vaša firma potrebuje." },
    ],

    mk_kicker: "Prepojenie systémov",
    mk_title: "Vaše nástroje môžu spolupracovať",
    mk_intro: "Prepojíme aplikácie, databázy a služby, ktoré používate. Prenosy dát, testy a ďalšie úlohy môžu bežať na pozadí s prehľadom o výsledkoch a chybách. Riešenie nasadíme do cloudu alebo na vaše servery.",
    mk_chips: &["Prepojenie cez API", "Databázy", "Spracovanie na pozadí", "Automatizované testy", "Prehľad chýb", "Monitoring", "Cloud", "Vlastné servery"],

    au_kicker: "Automatizácia práce",
    au_title: "Kde sa dá ubrať z ručnej práce",
    au_intro: "Pozrieme sa na úlohy, ktoré sa u vás pravidelne opakujú. Navrhneme, ktoré kroky môže spracovať softvér a ktoré majú zostať na človeku. Tu je niekoľko príkladov.",
    use_cases: [
        UseCaseTxt { title: "Pošta a zákaznícke otázky", desc: "Asistent roztriedi prijaté správy, vyberie potrebné údaje a pripraví odpoveď. Pred odoslaním ju skontroluje pracovník.", flow: &["Prijatá správa", "Triedenie", "Potrebné údaje", "Návrh odpovede", "Kontrola"], result: "Odpoveď pripravená na kontrolu" },
        UseCaseTxt { title: "Faktúry a dokumenty", desc: "Z dokumentu sa načítajú údaje, napríklad dodávateľ, suma a dátum. Po kontrole sa prenesú do systému, v ktorom s nimi pracujete.", flow: &["Dokument", "Načítanie údajov", "Kontrola", "Zápis do systému"], result: "Údaje dostupné bez opätovného prepisovania" },
        UseCaseTxt { title: "Fotografie a video", desc: "Z obrázkov možno rozpoznať produkty, prečítať text alebo označiť obsah na kontrolu. Výsledky sa uložia k príslušným záznamom.", flow: &["Obrázok alebo video", "Spracovanie", "Zaradenie", "Výsledok kontroly"], result: "Obsah roztriedený na ďalšie spracovanie" },
        UseCaseTxt { title: "Firemné informácie", desc: "Zamestnanec položí otázku a asistent vyhľadá odpoveď vo firemných podkladoch. Pripojí aj odkaz na dokument, z ktorého vychádzal.", flow: &["Otázka", "Vyhľadanie v podkladoch", "Odpoveď so zdrojom"], result: "Jednoduchšie hľadanie vo firemných dokumentoch" },
        UseCaseTxt { title: "Dáta medzi systémami", desc: "Nastavíme, ako sa majú polia zo zdroja priradiť k údajom v cieľovom systéme. Pred importom sa skontroluje formát a povinné hodnoty.", flow: &["Zdrojové dáta", "Mapovanie polí", "Kontrola", "Import"], result: "Pravidelný prenos podľa uloženého mapovania" },
    ],
    au_cases: [
        Trio { a: "Zodpovednosti", b: "Dohodnuté pravidlá kontroly", c: "Určíme, kto schvaľuje výsledky a ktoré kroky môžu prebehnúť automaticky. Dôležité operácie budú dohľadateľné." },
        Trio { a: "Existujúce systémy", b: "Nadviažeme na vaše nástroje", c: "Pri návrhu počítame s vašou poštou, účtovníctvom, CRM a ďalšími aplikáciami. Možnosti prepojenia preveríme v úvodnej analýze." },
        Trio { a: "Výber technológie", b: "Postup podľa konkrétnej úlohy", c: "Na presné výpočty a prenos dát použijeme pravidlá a integrácie. Pri práci s textom, obrazom alebo hlasom môže pomôcť AI." },
    ],
    au_cta_strong: "Máte na mysli konkrétny proces?",
    au_cta_p: "Napíšte nám, ako dnes prebieha a čo vám pri ňom zaberá čas. Prejdeme si s vami možnosti úpravy.",
    au_cta_btn: "Opísať proces",

    im_kicker: "Prínos pre firmu",
    im_title: "Čo sa zmení v každodennej práci",
    im_intro: "Pri návrhu si dohodneme, čo má riešenie zlepšiť. Po nasadení vieme porovnať čas spracovania, počet opráv aj prehľad o práci.",
    impact: [
        Trio { a: "Čas", b: "na bežné úlohy", c: "Údaje sa prenášajú medzi systémami a opakované kroky spracuje softvér." },
        Trio { a: "Prehľad", b: "o stave práce", c: "Úlohy, ich výsledky a prípadné chyby sú dostupné na jednom mieste." },
        Trio { a: "Dáta", b: "v potrebnom formáte", c: "Vstupy sa zjednotia a skontrolujú pred ďalším spracovaním." },
        Trio { a: "Kontrola", b: "nad výsledkami", c: "Pri dôležitých krokoch zostáva priestor na overenie a schválenie človekom." },
    ],

    pr_kicker: "Spolupráca",
    pr_title: "Ako bude prebiehať projekt",
    pr_intro: "Prácu rozdelíme na menšie časti. Priebežne vám ukazujeme výsledky a spoločne upravujeme ďalší postup.",
    steps: [
        Pair { a: "Úvodný rozhovor", b: "Prejdeme si cieľ, používané systémy, rozpočet a termín. Dohodneme rozsah prvej časti." },
        Pair { a: "Návrh a vývoj", b: "Pripravíme návrh a postupne ho realizujeme. Funkčné časti spolu priebežne overujeme." },
        Pair { a: "Nasadenie", b: "Riešenie uvedieme do prevádzky, overíme jeho fungovanie a zaškolíme používateľov." },
        Pair { a: "Ďalší rozvoj", b: "Sledujeme prevádzku, riešime pripomienky a dohodneme ďalšie úpravy podľa vašich potrieb." },
    ],

    st_kicker: "Technológie",
    st_title: "S čím pracujeme",
    st_intro: "Používame technológie, s ktorými máme skúsenosti. Pri výbere zohľadňujeme aj vaše súčasné prostredie a možnosti jeho údržby.",
    st_groups: ["Serverové aplikácie", "Používateľské rozhrania a API", "Infraštruktúra", "Databázy a prenos správ", "AI a integrácie"],

    faq_kicker: "Časté otázky",
    faq_title: "Čo vás môže zaujímať",
    faq: [
        Pair { a: "S akými projektmi máte skúsenosti?", b: "Máme skúsenosti s firemnými systémami v bankovníctve, telekomunikáciách a ďalších odvetviach. Riešime návrh, vývoj, integrácie aj prevádzku." },
        Pair { a: "Kto bude zodpovedať za technický návrh?", b: "Návrh a realizáciu zabezpečujeme spoločne. Rozhodnutia vám vysvetlíme a dohodneme, ako bude riešenie spravovať váš tím." },
        Pair { a: "Vyvíjate aj vlastné produkty?", b: "Áno. Jarvis, Cadence, Tally a Forge patria medzi naše vlastné produkty. Vznikajú pre úlohy, s ktorými sa stretávame pri každodennej práci." },
        Pair { a: "Viete pomôcť len s časťou projektu?", b: "Áno. Môžeme vytvoriť konkrétnu aplikáciu, integráciu alebo funkciu a spolupracovať s vašimi vývojármi." },
    ],

    dawn_pre: "Porozprávajme sa o tom, ",
    dawn_em: "čo potrebujete",
    dawn_post: " zjednodušiť.",

    ct_kicker: "Kontakt",
    ct_title: "Povedzte nám, čo potrebujete",
    ct_p: "Stručne opíšte, ako dnes pracujete a čo chcete zmeniť. Ak máte predstavu o termíne alebo rozsahu, pripojte ju k správe.",
    f_name: "Meno a priezvisko",
    f_name_ph: "Ján Novák",
    f_email: "E-mail",
    f_email_ph: "jan@firma.sk",
    f_company: "Firma",
    f_company_ph: "Voliteľné",
    f_message: "Správa",
    f_message_ph: "Čo chcete vyriešiť a aké systémy dnes používate?",
    form_note: "Údaje z formulára použijeme na vybavenie vašej požiadavky.",
    submit: "Odoslať správu",

    footer_desc: "Vyvíjame firemný softvér, prepájame systémy a automatizujeme opakované úlohy. Jarvis a ďalšie naše produkty pomáhajú s každodennou prácou.",
    footer_tag_pre: "Softvér pre ",
    footer_tag_em: "každodennú",
    footer_tag_post: " prácu.",
    footer_links: ["Riešenia", "Služby", "Spolupráca", "Kontakt", "E-mail"],
    footer_copy: "© 2026 Smart Dawn.",

    thanks_title: "Ďakujeme za záujem",
    thanks_msg: "Kontaktovať nás môžete aj priamo na hello@smartdawn.eu.",
    err_title: "Formulár obsahuje chyby",
    err_msg: "Skontrolujte označené údaje a skúste formulár odoslať znova.",
    notfound_title: "Stránka sa nenašla",
    notfound_msg: "Na tejto adrese sme stránku nenašli. Pokračovať môžete z hlavnej stránky.",
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
    meta_title: "Smart Dawn | Software development for businesses",
    meta_desc: "We develop business software and connect existing systems. Jarvis helps with daily operations, Cadence with planning and Tally with timesheets. Custom development is also available.",

    nav: ["Solutions", "Services", "Automation", "Working together", "Technology", "Contact"],
    cta_start: "Get in touch",
    skip: "Skip to content",
    brand_sub: "SOFTWARE DEVELOPMENT",
    lang_label: "Language",

    hero_eyebrow: "Custom software and our own products",
    hero_pre: "Software for ",
    hero_accent: "your business.",
    hero_lead: "We help businesses simplify their work with software. We can develop an application, connect existing systems or build a tool for recurring tasks. We also handle deployment and ongoing development.",
    hero_cta1: "Choose a solution",
    hero_cta2: "Arrange a conversation",

    domains_kicker: "Industries we work with",
    domains_lead: "We start by understanding how your business works. Your processes, existing systems and industry requirements guide the design.",
    domains: &["Banking", "Telecommunications", "Online retail", "Financial technology", "Insurance", "Healthcare", "Logistics", "Manufacturing", "Game development", "Public sector", "Energy"],

    iv_kicker: "About the founder",
    iv_role: "Founder and software architect",
    iv_p1: "Ondrej Luknár has more than ten years of experience designing and developing software for banks, telecommunications companies and large businesses. His work covers both technical design and implementation.",
    iv_p2: "At Smart Dawn he develops products and custom software. Jarvis, Cadence and Tally grow out of practical needs in organizing work. He brings this experience to customer projects as well.",
    iv_creds: ["RAG & Agentic AI · IBM", "Retrieval-Augmented Generation · DeepLearning.AI", "Bratislava, SK"],

    sv_kicker: "Services",
    sv_title: "How we can help",
    sv_intro: "We can take on the full development or join your team for a specific part of a project. We agree on scope and responsibilities at the start.",
    sv_cards: [
        Pair { a: "Solution design", b: "We review your needs, existing systems and constraints, then prepare a technical design, a work plan and a cost estimate." },
        Pair { a: "Development and integrations", b: "We build applications, connect systems and add AI features where useful. Testing and reviews with your team are part of the work." },
        Pair { a: "Deployment and support", b: "We handle deployment, monitor operations and resolve issues. Support and further development are agreed around your needs." },
    ],

    mk_kicker: "System integration",
    mk_title: "Connect the tools you already use",
    mk_intro: "We connect your applications, databases and services. Data transfers, tests and other tasks can run in the background with a record of results and errors. Deployment can use the cloud or your own servers.",
    mk_chips: &["API connections", "Databases", "Background processing", "Automated tests", "Error reporting", "Monitoring", "Cloud", "Your own servers"],

    au_kicker: "Work automation",
    au_title: "Where software can reduce manual work",
    au_intro: "We look at the tasks your team repeats regularly and identify which steps software can handle and which need a person. Here are a few examples.",
    use_cases: [
        UseCaseTxt { title: "Mail and customer enquiries", desc: "An assistant sorts incoming messages, extracts the necessary details and drafts a reply. A member of your team reviews it before sending.", flow: &["Incoming message", "Sorting", "Relevant details", "Draft reply", "Review"], result: "A reply ready for review" },
        UseCaseTxt { title: "Invoices and documents", desc: "Details such as the supplier, amount and date are read from a document. After review, they are transferred into the system where you need them.", flow: &["Document", "Read details", "Review", "Save to system"], result: "Details available without retyping" },
        UseCaseTxt { title: "Photos and video", desc: "Images can be used to identify products, read text or flag content for review. Results are stored alongside the relevant records.", flow: &["Image or video", "Processing", "Classification", "Review result"], result: "Content organized for further processing" },
        UseCaseTxt { title: "Company information", desc: "An employee asks a question and the assistant searches company documents for an answer. It includes a link to the source it used.", flow: &["Question", "Search documents", "Answer with source"], result: "Company documents are easier to search" },
        UseCaseTxt { title: "Data between systems", desc: "We define how fields from the source map to the target system. Formats and required values are checked before import.", flow: &["Source data", "Field mapping", "Validation", "Import"], result: "Regular transfers using saved mappings" },
    ],
    au_cases: [
        Trio { a: "Responsibilities", b: "Agreed review rules", c: "We define who approves results and which steps can run automatically. Important operations remain traceable." },
        Trio { a: "Existing systems", b: "Work with your current tools", c: "The design takes account of your email, accounting, CRM and other applications. We check the available integration options during the initial analysis." },
        Trio { a: "Technology choices", b: "A method that suits the task", c: "We use rules and integrations for exact calculations and data transfers. AI can help with text, images and speech." },
    ],
    au_cta_strong: "Do you have a process in mind?",
    au_cta_p: "Tell us how it works today and which parts take up your time. We will review the options with you.",
    au_cta_btn: "Describe your process",

    im_kicker: "Business benefits",
    im_title: "What changes in daily work",
    im_intro: "We agree on what the solution should improve. After deployment, we can compare processing time, corrections and visibility of the work.",
    impact: [
        Trio { a: "Time", b: "for everyday tasks", c: "Data moves between systems and software handles recurring steps." },
        Trio { a: "Clarity", b: "about work in progress", c: "Tasks, results and errors are available in one place." },
        Trio { a: "Data", b: "in the required format", c: "Inputs are standardized and checked before further processing." },
        Trio { a: "Control", b: "over the results", c: "Important steps leave room for a person to review and approve." },
    ],

    pr_kicker: "Working together",
    pr_title: "How the project will run",
    pr_intro: "We divide the work into smaller parts, show you results regularly and agree on the next steps together.",
    steps: [
        Pair { a: "Initial conversation", b: "We discuss your goal, systems, budget and timing, then agree on the scope of the first stage." },
        Pair { a: "Design and development", b: "We prepare a design and build it in stages, reviewing working parts with your team." },
        Pair { a: "Deployment", b: "We put the solution into use, check its operation and train users." },
        Pair { a: "Further development", b: "We monitor operation, address feedback and agree on improvements as your needs change." },
    ],

    st_kicker: "Technologies",
    st_title: "Technologies we work with",
    st_intro: "We use technologies we have experience with. We also consider your existing environment and the people who will maintain it.",
    st_groups: ["Server applications", "User interfaces and APIs", "Infrastructure", "Databases and messaging", "AI and integrations"],

    faq_kicker: "Common questions",
    faq_title: "What you may want to know",
    faq: [
        Pair { a: "What kinds of projects have you worked on?", b: "We have experience with business systems in banking, telecommunications and other industries. Our work includes design, development, integration and operation." },
        Pair { a: "Who is responsible for the technical design?", b: "We handle design and implementation together. We explain the decisions and agree on how your team will maintain the solution." },
        Pair { a: "Do you develop your own products?", b: "Yes. Jarvis, Cadence, Tally and Forge are among our own products. They address tasks we encounter in everyday work." },
        Pair { a: "Can you help with just part of a project?", b: "Yes. We can build a specific application, integration or feature and work alongside your developers." },
    ],

    dawn_pre: "Let us talk about ",
    dawn_em: "the work",
    dawn_post: " you want to simplify.",

    ct_kicker: "Contact",
    ct_title: "Tell us what you need",
    ct_p: "Briefly describe how you work today and what you would like to change. Include any timing or scope you already have in mind.",
    f_name: "Full name",
    f_name_ph: "John Smith",
    f_email: "Email",
    f_email_ph: "john@company.com",
    f_company: "Company",
    f_company_ph: "Optional",
    f_message: "Message",
    f_message_ph: "What would you like to solve, and which systems do you use today?",
    form_note: "We will use the details in this form to respond to your enquiry.",
    submit: "Send message",

    footer_desc: "We develop business software, connect systems and automate recurring tasks. Jarvis and our other products help with everyday work.",
    footer_tag_pre: "Software for ",
    footer_tag_em: "everyday",
    footer_tag_post: " work.",
    footer_links: ["Solutions", "Services", "Working together", "Contact", "Email"],
    footer_copy: "© 2026 Smart Dawn.",

    thanks_title: "Thank you for your interest",
    thanks_msg: "You can also contact us directly at hello@smartdawn.eu.",
    err_title: "The form contains errors",
    err_msg: "Check the highlighted details and try submitting the form again.",
    notfound_title: "Page not found",
    notfound_msg: "We could not find a page at this address. You can continue from the home page.",
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
    meta_title: "Smart Dawn | Vývoj softwaru pro firmy",
    meta_desc: "Vyvíjíme software pro firmy a propojujeme jejich systémy. Jarvis pomáhá s operativou, Cadence s plánováním a Tally s výkazy. Dodáváme i řešení na míru.",

    nav: ["Řešení", "Služby", "Automatizace", "Spolupráce", "Technologie", "Kontakt"],
    cta_start: "Napište nám",
    skip: "Přeskočit na obsah",
    brand_sub: "VÝVOJ SOFTWARU",
    lang_label: "Jazyk",

    hero_eyebrow: "Software na míru a vlastní produkty",
    hero_pre: "Software pro ",
    hero_accent: "vaši firmu.",
    hero_lead: "Pomáháme firmám zjednodušit práci pomocí softwaru. Vyvineme aplikaci, propojíme stávající systémy nebo připravíme nástroj na opakované úkoly. Postaráme se také o nasazení a další rozvoj.",
    hero_cta1: "Vybrat typ řešení",
    hero_cta2: "Domluvit rozhovor",

    domains_kicker: "Oblasti, kterým se věnujeme",
    domains_lead: "Při návrhu vycházíme z toho, jak vaše firma pracuje. Zohledníme její procesy, používané systémy i požadavky odvětví.",
    domains: &["Bankovnictví", "Telekomunikace", "Internetový prodej", "Finanční technologie", "Pojišťovnictví", "Zdravotnictví", "Logistika", "Výroba", "Herní průmysl", "Veřejný sektor", "Energetika"],

    iv_kicker: "O zakladateli",
    iv_role: "Zakladatel a softwarový architekt",
    iv_p1: "Ondrej Luknár má více než deset let zkušeností s návrhem a vývojem softwaru pro banky, telekomunikační společnosti a velké firmy. Věnuje se technickému návrhu i samotné realizaci.",
    iv_p2: "Ve Smart Dawn vyvíjí vlastní produkty i zakázková řešení. Jarvis, Cadence a Tally vznikají z praktických potřeb při organizaci práce. Tyto zkušenosti využívá i při spolupráci se zákazníky.",
    iv_creds: ["RAG & Agentic AI · IBM", "Retrieval-Augmented Generation · DeepLearning.AI", "Bratislava, SK"],

    sv_kicker: "Služby",
    sv_title: "S čím vám pomůžeme",
    sv_intro: "Můžeme převzít celý vývoj nebo se přidat k vašemu týmu při konkrétní části projektu. Rozsah práce a odpovědnosti si dohodneme na začátku.",
    sv_cards: [
        Pair { a: "Návrh řešení", b: "Projdeme vaše potřeby, stávající systémy a omezení. Připravíme technický návrh, postup prací a odhad nákladů." },
        Pair { a: "Vývoj a integrace", b: "Vytvoříme aplikaci, propojíme systémy nebo doplníme AI funkce. Součástí práce je testování i ověření s vaším týmem." },
        Pair { a: "Nasazení a podpora", b: "Zajistíme nasazení, dohled nad provozem a řešení chyb. Podporu a další rozvoj přizpůsobíme tomu, co vaše firma potřebuje." },
    ],

    mk_kicker: "Propojení systémů",
    mk_title: "Vaše nástroje mohou spolupracovat",
    mk_intro: "Propojíme aplikace, databáze a služby, které používáte. Přenosy dat, testy a další úkoly mohou běžet na pozadí s přehledem o výsledcích a chybách. Řešení nasadíme do cloudu nebo na vaše servery.",
    mk_chips: &["Propojení přes API", "Databáze", "Zpracování na pozadí", "Automatizované testy", "Přehled chyb", "Monitoring", "Cloud", "Vlastní servery"],

    au_kicker: "Automatizace práce",
    au_title: "Kde se dá ubrat z ruční práce",
    au_intro: "Podíváme se na úkoly, které se u vás pravidelně opakují. Navrhneme, které kroky může zpracovat software a které mají zůstat na člověku. Tady je několik příkladů.",
    use_cases: [
        UseCaseTxt { title: "Pošta a zákaznické dotazy", desc: "Asistent roztřídí přijaté zprávy, vybere potřebné údaje a připraví odpověď. Před odesláním ji zkontroluje pracovník.", flow: &["Přijatá zpráva", "Třídění", "Potřebné údaje", "Návrh odpovědi", "Kontrola"], result: "Odpověď připravená ke kontrole" },
        UseCaseTxt { title: "Faktury a dokumenty", desc: "Z dokumentu se načtou údaje, například dodavatel, částka a datum. Po kontrole se přenesou do systému, ve kterém s nimi pracujete.", flow: &["Dokument", "Načtení údajů", "Kontrola", "Zápis do systému"], result: "Údaje dostupné bez opětovného přepisování" },
        UseCaseTxt { title: "Fotografie a video", desc: "Z obrázků lze rozpoznat produkty, přečíst text nebo označit obsah ke kontrole. Výsledky se uloží k příslušným záznamům.", flow: &["Obrázek nebo video", "Zpracování", "Zařazení", "Výsledek kontroly"], result: "Obsah roztříděný pro další zpracování" },
        UseCaseTxt { title: "Firemní informace", desc: "Zaměstnanec položí otázku a asistent vyhledá odpověď ve firemních podkladech. Připojí také odkaz na dokument, ze kterého vycházel.", flow: &["Otázka", "Vyhledání v podkladech", "Odpověď se zdrojem"], result: "Jednodušší hledání ve firemních dokumentech" },
        UseCaseTxt { title: "Data mezi systémy", desc: "Nastavíme, jak se mají pole ze zdroje přiřadit k údajům v cílovém systému. Před importem se zkontroluje formát a povinné hodnoty.", flow: &["Zdrojová data", "Mapování polí", "Kontrola", "Import"], result: "Pravidelný přenos podle uloženého mapování" },
    ],
    au_cases: [
        Trio { a: "Odpovědnosti", b: "Dohodnutá pravidla kontroly", c: "Určíme, kdo schvaluje výsledky a které kroky mohou proběhnout automaticky. Důležité operace budou dohledatelné." },
        Trio { a: "Stávající systémy", b: "Navážeme na vaše nástroje", c: "Při návrhu počítáme s vaší poštou, účetnictvím, CRM a dalšími aplikacemi. Možnosti propojení prověříme v úvodní analýze." },
        Trio { a: "Výběr technologie", b: "Postup podle konkrétního úkolu", c: "Na přesné výpočty a přenos dat použijeme pravidla a integrace. Při práci s textem, obrazem nebo hlasem může pomoci AI." },
    ],
    au_cta_strong: "Máte na mysli konkrétní proces?",
    au_cta_p: "Napište nám, jak dnes probíhá a co vám při něm zabírá čas. Projdeme s vámi možnosti úpravy.",
    au_cta_btn: "Popsat proces",

    im_kicker: "Přínos pro firmu",
    im_title: "Co se změní v každodenní práci",
    im_intro: "Při návrhu si dohodneme, co má řešení zlepšit. Po nasazení můžeme porovnat čas zpracování, počet oprav i přehled o práci.",
    impact: [
        Trio { a: "Čas", b: "na běžné úkoly", c: "Údaje se přenášejí mezi systémy a opakované kroky zpracuje software." },
        Trio { a: "Přehled", b: "o stavu práce", c: "Úkoly, jejich výsledky a případné chyby jsou dostupné na jednom místě." },
        Trio { a: "Data", b: "v potřebném formátu", c: "Vstupy se sjednotí a zkontrolují před dalším zpracováním." },
        Trio { a: "Kontrola", b: "nad výsledky", c: "U důležitých kroků zůstává prostor pro ověření a schválení člověkem." },
    ],

    pr_kicker: "Spolupráce",
    pr_title: "Jak bude probíhat projekt",
    pr_intro: "Práci rozdělíme na menší části. Průběžně vám ukazujeme výsledky a společně upravujeme další postup.",
    steps: [
        Pair { a: "Úvodní rozhovor", b: "Projdeme cíl, používané systémy, rozpočet a termín. Dohodneme rozsah první části." },
        Pair { a: "Návrh a vývoj", b: "Připravíme návrh a postupně ho realizujeme. Funkční části společně průběžně ověřujeme." },
        Pair { a: "Nasazení", b: "Řešení uvedeme do provozu, ověříme jeho fungování a zaškolíme uživatele." },
        Pair { a: "Další rozvoj", b: "Sledujeme provoz, řešíme připomínky a dohodneme další úpravy podle vašich potřeb." },
    ],

    st_kicker: "Technologie",
    st_title: "S čím pracujeme",
    st_intro: "Používáme technologie, se kterými máme zkušenosti. Při výběru zohledňujeme také vaše současné prostředí a možnosti jeho údržby.",
    st_groups: ["Serverové aplikace", "Uživatelská rozhraní a API", "Infrastruktura", "Databáze a přenos zpráv", "AI a integrace"],

    faq_kicker: "Časté otázky",
    faq_title: "Co vás může zajímat",
    faq: [
        Pair { a: "S jakými projekty máte zkušenosti?", b: "Máme zkušenosti s firemními systémy v bankovnictví, telekomunikacích a dalších odvětvích. Řešíme návrh, vývoj, integrace i provoz." },
        Pair { a: "Kdo bude odpovídat za technický návrh?", b: "Návrh a realizaci zajišťujeme společně. Rozhodnutí vám vysvětlíme a dohodneme, jak bude řešení spravovat váš tým." },
        Pair { a: "Vyvíjíte také vlastní produkty?", b: "Ano. Jarvis, Cadence, Tally a Forge patří mezi naše vlastní produkty. Vznikají pro úkoly, se kterými se setkáváme při každodenní práci." },
        Pair { a: "Umíte pomoci jen s částí projektu?", b: "Ano. Můžeme vytvořit konkrétní aplikaci, integraci nebo funkci a spolupracovat s vašimi vývojáři." },
    ],

    dawn_pre: "Promluvme si o tom, ",
    dawn_em: "co potřebujete",
    dawn_post: " zjednodušit.",

    ct_kicker: "Kontakt",
    ct_title: "Řekněte nám, co potřebujete",
    ct_p: "Stručně popište, jak dnes pracujete a co chcete změnit. Pokud máte představu o termínu nebo rozsahu, připojte ji ke zprávě.",
    f_name: "Jméno a příjmení",
    f_name_ph: "Jan Novák",
    f_email: "E-mail",
    f_email_ph: "jan@firma.cz",
    f_company: "Firma",
    f_company_ph: "Volitelné",
    f_message: "Zpráva",
    f_message_ph: "Co chcete vyřešit a jaké systémy dnes používáte?",
    form_note: "Údaje z formuláře použijeme k vyřízení vašeho požadavku.",
    submit: "Odeslat zprávu",

    footer_desc: "Vyvíjíme firemní software, propojujeme systémy a automatizujeme opakované úkoly. Jarvis a další naše produkty pomáhají s každodenní prací.",
    footer_tag_pre: "Software pro ",
    footer_tag_em: "každodenní",
    footer_tag_post: " práci.",
    footer_links: ["Řešení", "Služby", "Spolupráce", "Kontakt", "E-mail"],
    footer_copy: "© 2026 Smart Dawn.",

    thanks_title: "Děkujeme za zájem",
    thanks_msg: "Kontaktovat nás můžete také přímo na hello@smartdawn.eu.",
    err_title: "Formulář obsahuje chyby",
    err_msg: "Zkontrolujte označené údaje a zkuste formulář odeslat znovu.",
    notfound_title: "Stránka nebyla nalezena",
    notfound_msg: "Na této adrese jsme stránku nenašli. Pokračovat můžete z hlavní stránky.",
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
    meta_title: "Smart Dawn | Softwareentwicklung für Unternehmen",
    meta_desc: "Wir entwickeln Unternehmenssoftware und verbinden bestehende Systeme. Jarvis hilft im Alltag, Cadence bei der Planung und Tally bei Stundennachweisen. Individuelle Entwicklung gehört ebenfalls dazu.",

    nav: ["Lösungen", "Leistungen", "Automatisierung", "Zusammenarbeit", "Technologien", "Kontakt"],
    cta_start: "Kontakt aufnehmen",
    skip: "Zum Inhalt springen",
    brand_sub: "SOFTWAREENTWICKLUNG",
    lang_label: "Sprache",

    hero_eyebrow: "Individuelle Software und eigene Produkte",
    hero_pre: "Software für ",
    hero_accent: "Ihr Unternehmen.",
    hero_lead: "Wir helfen Unternehmen, ihre Arbeit mit Software zu vereinfachen. Wir entwickeln Anwendungen, verbinden bestehende Systeme oder erstellen Werkzeuge für wiederkehrende Aufgaben. Auch Einführung und Weiterentwicklung gehören dazu.",
    hero_cta1: "Lösung auswählen",
    hero_cta2: "Gespräch vereinbaren",

    domains_kicker: "Branchen, mit denen wir arbeiten",
    domains_lead: "Wir machen uns zunächst mit der Arbeitsweise Ihres Unternehmens vertraut. Ihre Abläufe, vorhandenen Systeme und Branchenanforderungen bestimmen den Entwurf.",
    domains: &["Banken", "Telekommunikation", "Onlinehandel", "Finanztechnologie", "Versicherungen", "Gesundheitswesen", "Logistik", "Produktion", "Spieleentwicklung", "Öffentlicher Sektor", "Energie"],

    iv_kicker: "Über den Gründer",
    iv_role: "Gründer und Softwarearchitekt",
    iv_p1: "Ondrej Luknár hat mehr als zehn Jahre Erfahrung mit dem Entwurf und der Entwicklung von Software für Banken, Telekommunikationsunternehmen und große Firmen. Er arbeitet sowohl am technischen Entwurf als auch an der Umsetzung.",
    iv_p2: "Bei Smart Dawn entwickelt er eigene Produkte und individuelle Software. Jarvis, Cadence und Tally entstehen aus praktischen Anforderungen bei der Arbeitsorganisation. Diese Erfahrungen fließen auch in Kundenprojekte ein.",
    iv_creds: ["RAG & Agentic AI · IBM", "Retrieval-Augmented Generation · DeepLearning.AI", "Bratislava, SK"],

    sv_kicker: "Leistungen",
    sv_title: "Wobei wir helfen können",
    sv_intro: "Wir übernehmen die gesamte Entwicklung oder unterstützen Ihr Team bei einem bestimmten Teil des Projekts. Umfang und Zuständigkeiten vereinbaren wir zu Beginn.",
    sv_cards: [
        Pair { a: "Lösungsentwurf", b: "Wir besprechen Ihre Anforderungen, vorhandenen Systeme und Einschränkungen. Daraus entstehen ein technischer Entwurf, ein Arbeitsplan und eine Kostenschätzung." },
        Pair { a: "Entwicklung und Integration", b: "Wir erstellen Anwendungen, verbinden Systeme und ergänzen bei Bedarf KI-Funktionen. Tests und gemeinsame Prüfungen mit Ihrem Team gehören dazu." },
        Pair { a: "Einführung und Support", b: "Wir übernehmen die Einführung, überwachen den Betrieb und beheben Fehler. Support und Weiterentwicklung richten sich nach Ihrem Bedarf." },
    ],

    mk_kicker: "Systemintegration",
    mk_title: "Verbinden Sie Ihre vorhandenen Werkzeuge",
    mk_intro: "Wir verbinden Ihre Anwendungen, Datenbanken und Dienste. Datenübertragungen, Tests und weitere Aufgaben können im Hintergrund laufen, mit einer Übersicht über Ergebnisse und Fehler. Die Lösung läuft in der Cloud oder auf Ihren eigenen Servern.",
    mk_chips: &["API-Anbindung", "Datenbanken", "Hintergrundverarbeitung", "Automatisierte Tests", "Fehlerübersicht", "Monitoring", "Cloud", "Eigene Server"],

    au_kicker: "Arbeitsautomatisierung",
    au_title: "Wo sich manuelle Arbeit reduzieren lässt",
    au_intro: "Wir betrachten Aufgaben, die Ihr Team regelmäßig wiederholt. Gemeinsam legen wir fest, welche Schritte Software übernehmen kann und welche einen Menschen brauchen. Hier sind einige Beispiele.",
    use_cases: [
        UseCaseTxt { title: "Post und Kundenanfragen", desc: "Ein Assistent sortiert eingehende Nachrichten, liest die nötigen Angaben aus und entwirft eine Antwort. Ein Mitarbeiter prüft sie vor dem Versand.", flow: &["Nachricht", "Sortierung", "Benötigte Angaben", "Antwortentwurf", "Prüfung"], result: "Eine Antwort zur Prüfung vorbereitet" },
        UseCaseTxt { title: "Rechnungen und Dokumente", desc: "Angaben wie Lieferant, Betrag und Datum werden aus einem Dokument gelesen. Nach der Prüfung gelangen sie in das System, in dem Sie damit arbeiten.", flow: &["Dokument", "Angaben auslesen", "Prüfung", "Im System speichern"], result: "Angaben ohne erneutes Abtippen verfügbar" },
        UseCaseTxt { title: "Fotos und Videos", desc: "Bilder lassen sich nutzen, um Produkte zu erkennen, Text zu lesen oder Inhalte zur Prüfung zu markieren. Die Ergebnisse werden den jeweiligen Datensätzen zugeordnet.", flow: &["Bild oder Video", "Verarbeitung", "Zuordnung", "Prüfergebnis"], result: "Inhalte für die Weiterverarbeitung geordnet" },
        UseCaseTxt { title: "Unternehmenswissen", desc: "Ein Mitarbeiter stellt eine Frage, und der Assistent sucht in den Unternehmensunterlagen nach einer Antwort. Ein Link verweist auf die verwendete Quelle.", flow: &["Frage", "Unterlagen durchsuchen", "Antwort mit Quelle"], result: "Einfachere Suche in Unternehmensunterlagen" },
        UseCaseTxt { title: "Daten zwischen Systemen", desc: "Wir legen fest, wie Quellfelder den Angaben im Zielsystem zugeordnet werden. Formate und Pflichtwerte werden vor dem Import geprüft.", flow: &["Quelldaten", "Feldzuordnung", "Prüfung", "Import"], result: "Regelmäßige Übertragung nach gespeicherter Zuordnung" },
    ],
    au_cases: [
        Trio { a: "Zuständigkeiten", b: "Vereinbarte Prüfregeln", c: "Wir legen fest, wer Ergebnisse freigibt und welche Schritte automatisch laufen dürfen. Wichtige Vorgänge bleiben nachvollziehbar." },
        Trio { a: "Vorhandene Systeme", b: "Ihre Werkzeuge weiterverwenden", c: "Der Entwurf berücksichtigt Ihre Post, Buchhaltung, Ihr CRM und weitere Anwendungen. Die Anbindungsmöglichkeiten prüfen wir zu Beginn." },
        Trio { a: "Technologiewahl", b: "Ein passendes Verfahren für die Aufgabe", c: "Für genaue Berechnungen und Datentransfers nutzen wir Regeln und Integrationen. Bei Texten, Bildern und Sprache kann KI helfen." },
    ],
    au_cta_strong: "Haben Sie einen bestimmten Ablauf im Blick?",
    au_cta_p: "Beschreiben Sie uns, wie er heute funktioniert und welche Schritte Zeit kosten. Wir besprechen die Möglichkeiten mit Ihnen.",
    au_cta_btn: "Ablauf beschreiben",

    im_kicker: "Nutzen für Ihr Unternehmen",
    im_title: "Was sich im Arbeitsalltag verändert",
    im_intro: "Wir vereinbaren, was die Lösung verbessern soll. Nach der Einführung können wir Bearbeitungszeit, Korrekturen und Übersicht über die Arbeit vergleichen.",
    impact: [
        Trio { a: "Zeit", b: "für tägliche Aufgaben", c: "Daten werden zwischen Systemen übertragen und wiederkehrende Schritte von Software erledigt." },
        Trio { a: "Überblick", b: "über laufende Arbeit", c: "Aufgaben, Ergebnisse und Fehler sind an einem Ort verfügbar." },
        Trio { a: "Daten", b: "im benötigten Format", c: "Eingaben werden vor der Weiterverarbeitung vereinheitlicht und geprüft." },
        Trio { a: "Kontrolle", b: "über die Ergebnisse", c: "Wichtige Schritte lassen Raum für eine Prüfung und Freigabe durch einen Menschen." },
    ],

    pr_kicker: "Zusammenarbeit",
    pr_title: "So läuft das Projekt ab",
    pr_intro: "Wir teilen die Arbeit in überschaubare Abschnitte auf, zeigen regelmäßig Ergebnisse und stimmen die nächsten Schritte mit Ihnen ab.",
    steps: [
        Pair { a: "Erstes Gespräch", b: "Wir besprechen Ziel, Systeme, Budget und Termin und vereinbaren den Umfang des ersten Abschnitts." },
        Pair { a: "Entwurf und Entwicklung", b: "Wir erstellen einen Entwurf und setzen ihn schrittweise um. Funktionsfähige Teile prüfen wir gemeinsam mit Ihrem Team." },
        Pair { a: "Einführung", b: "Wir nehmen die Lösung in Betrieb, prüfen ihre Funktion und schulen die Anwender." },
        Pair { a: "Weiterentwicklung", b: "Wir beobachten den Betrieb, bearbeiten Rückmeldungen und vereinbaren weitere Anpassungen nach Ihrem Bedarf." },
    ],

    st_kicker: "Technologien",
    st_title: "Womit wir arbeiten",
    st_intro: "Wir nutzen Technologien, mit denen wir Erfahrung haben. Bei der Auswahl berücksichtigen wir auch Ihre vorhandene Umgebung und deren Wartung.",
    st_groups: ["Serveranwendungen", "Benutzeroberflächen und APIs", "Infrastruktur", "Datenbanken und Nachrichten", "KI und Integrationen"],

    faq_kicker: "Häufige Fragen",
    faq_title: "Was Sie wissen möchten",
    faq: [
        Pair { a: "Mit welchen Projekten haben Sie Erfahrung?", b: "Wir haben Erfahrung mit Unternehmenssystemen im Bankwesen, in der Telekommunikation und weiteren Branchen. Unsere Arbeit umfasst Entwurf, Entwicklung, Integration und Betrieb." },
        Pair { a: "Wer ist für den technischen Entwurf verantwortlich?", b: "Wir betreuen Entwurf und Umsetzung gemeinsam. Wir erläutern die Entscheidungen und vereinbaren, wie Ihr Team die Lösung später pflegt." },
        Pair { a: "Entwickeln Sie auch eigene Produkte?", b: "Ja. Jarvis, Cadence, Tally und Forge gehören zu unseren eigenen Produkten. Sie bearbeiten Aufgaben, denen wir im Arbeitsalltag begegnen." },
        Pair { a: "Können Sie auch bei einem Teil des Projekts helfen?", b: "Ja. Wir können eine bestimmte Anwendung, Integration oder Funktion erstellen und mit Ihren Entwicklern zusammenarbeiten." },
    ],

    dawn_pre: "Sprechen wir darüber, ",
    dawn_em: "welche Arbeit",
    dawn_post: " Sie vereinfachen möchten.",

    ct_kicker: "Kontakt",
    ct_title: "Sagen Sie uns, was Sie brauchen",
    ct_p: "Beschreiben Sie kurz, wie Sie heute arbeiten und was Sie ändern möchten. Wenn Sie bereits einen Termin oder Umfang im Blick haben, ergänzen Sie diese Angaben.",
    f_name: "Vor- und Nachname",
    f_name_ph: "Max Mustermann",
    f_email: "E-Mail",
    f_email_ph: "max@firma.de",
    f_company: "Unternehmen",
    f_company_ph: "Optional",
    f_message: "Nachricht",
    f_message_ph: "Was möchten Sie lösen und welche Systeme verwenden Sie heute?",
    form_note: "Wir verwenden Ihre Angaben, um Ihre Anfrage zu bearbeiten.",
    submit: "Nachricht senden",

    footer_desc: "Wir entwickeln Unternehmenssoftware, verbinden Systeme und automatisieren wiederkehrende Aufgaben. Jarvis und unsere anderen Produkte helfen bei der täglichen Arbeit.",
    footer_tag_pre: "Software für die ",
    footer_tag_em: "tägliche",
    footer_tag_post: " Arbeit.",
    footer_links: ["Lösungen", "Leistungen", "Zusammenarbeit", "Kontakt", "E-Mail"],
    footer_copy: "© 2026 Smart Dawn.",

    thanks_title: "Vielen Dank für Ihr Interesse",
    thanks_msg: "Sie erreichen uns auch direkt unter hello@smartdawn.eu.",
    err_title: "Das Formular enthält Fehler",
    err_msg: "Prüfen Sie die markierten Angaben und senden Sie das Formular erneut.",
    notfound_title: "Seite nicht gefunden",
    notfound_msg: "Unter dieser Adresse konnten wir keine Seite finden. Sie können auf der Startseite fortfahren.",
    back_home: "Zurück zur Startseite",
    val_name_short: "Der Name ist zu kurz.",
    val_name_long: "Der Name ist zu lang.",
    val_email: "Die E-Mail-Adresse hat ein ungültiges Format.",
    val_msg_short: "Die Nachricht ist zu kurz (mindestens 20 Zeichen).",
    val_msg_long: "Die Nachricht ist zu lang.",
    val_company_long: "Der Firmenname ist zu lang.",
};
