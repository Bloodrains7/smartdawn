use leptos::prelude::*;

use crate::i18n::{Lang, Pair};

struct Project {
    slug: &'static str,
    tags: &'static [&'static str],
}

struct ProjectText {
    name: &'static str,
    category: &'static str,
    title: &'static str,
    description: &'static str,
}

struct PortfolioText {
    eyebrow: &'static str,
    title: &'static str,
    intro: &'static str,
    hero_link: &'static str,
    workflow_label: &'static str,
    workflow: [Pair; 3],
    daily_title: &'static str,
    more_title: &'static str,
    contact: &'static str,
    interest: &'static str,
    projects: [ProjectText; 9],
}

const PROJECTS: [Project; 9] = [
    Project {
        slug: "jarvis",
        tags: &["E-mail", "Telegram", "Slack", "Notion", "GitLab"],
    },
    Project {
        slug: "cadence",
        tags: &["Notion", "Desktop", "MCP"],
    },
    Project {
        slug: "tally",
        tags: &["Excel", "CSV", "Offline"],
    },
    Project {
        slug: "forge",
        tags: &["REST", "GraphQL", "gRPC", "CI/CD"],
    },
    Project {
        slug: "mr-reviewer",
        tags: &["GitLab", "AI", "MCP"],
    },
    Project {
        slug: "archgen",
        tags: &["UML", "BPMN", "ArchiMate"],
    },
    Project {
        slug: "repre",
        tags: &["CSV", "XML", "API"],
    },
    Project {
        slug: "parley",
        tags: &["Whisper", "Audio", "Offline"],
    },
    Project {
        slug: "own-ide",
        tags: &["LSP", "Git", "MCP"],
    },
];

#[component]
pub fn PortfolioTeaser(lang: Lang) -> impl IntoView {
    view! {
        <a class="portfolio-teaser" href="#jarvis" data-testid="jarvis-teaser">
            <span class="portfolio-teaser-mark" aria-hidden="true">"J"</span>
            <span>{copy(lang).hero_link}</span>
            <span aria-hidden="true">"↗"</span>
        </a>
    }
}

#[component]
pub fn Portfolio(lang: Lang) -> impl IntoView {
    let text = copy(lang);
    let jarvis = &text.projects[0];

    view! {
        <section id="praca" class="portfolio" aria-labelledby="portfolio-title" data-testid="portfolio">
            <div class="shell">
                <div class="section-head" data-reveal>
                    <span class="section-kicker">{text.eyebrow}</span>
                    <h2 id="portfolio-title" class="section-title">{text.title}</h2>
                    <p class="section-intro">{text.intro}</p>
                </div>
                <article id="jarvis" class="jarvis-feature" aria-labelledby="jarvis-name" data-project="jarvis" data-reveal>
                    <div class="jarvis-copy">
                        <span class="solution-category">{jarvis.category}</span>
                        <h3 id="jarvis-name">{jarvis.name}<span class="jarvis-period" aria-hidden="true">"."</span></h3>
                        <p class="jarvis-promise">{jarvis.title}</p>
                        <p class="solution-description">{jarvis.description}</p>
                        {project_tags(&PROJECTS[0])}
                        {contact_link(&PROJECTS[0], jarvis, text, true)}
                    </div>
                    <div class="jarvis-workflow">
                        <div class="jarvis-workflow-head">
                            <span class="jarvis-monogram" aria-hidden="true">"J"</span>
                            <span>{text.workflow_label}</span>
                        </div>
                        <ol>
                            {text.workflow.iter().enumerate().map(|(i, step)| view! {
                                <li>
                                    <span class="workflow-index" aria-hidden="true">{format!("{:02}", i + 1)}</span>
                                    <div><strong>{step.a}</strong><p>{step.b}</p></div>
                                </li>
                            }).collect::<Vec<_>>()}
                        </ol>
                        <div class="jarvis-workflow-foot" aria-hidden="true"><span></span>"SMART DAWN / JARVIS"</div>
                    </div>
                </article>
                <h3 class="portfolio-group-title" data-reveal>{text.daily_title}</h3>
                <div class="daily-solutions">
                    {(1..3).map(|i| project_card(&PROJECTS[i], &text.projects[i], text, true)).collect::<Vec<_>>()}
                </div>
                <h3 class="portfolio-group-title" data-reveal>{text.more_title}</h3>
                <div class="portfolio-grid">
                    {(3..PROJECTS.len()).map(|i| project_card(&PROJECTS[i], &text.projects[i], text, false)).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

fn project_card(
    project: &'static Project,
    body: &'static ProjectText,
    text: &'static PortfolioText,
    daily: bool,
) -> impl IntoView {
    let class = if daily {
        "solution-card solution-card--daily"
    } else {
        "solution-card"
    };
    let heading_id = format!("{}-name", project.slug);

    view! {
        <article id={project.slug} class={class} data-project={project.slug} aria-labelledby={heading_id.clone()} data-reveal>
            <span class="solution-category">{body.category}</span>
            <h4 id={heading_id.clone()}>{body.name}</h4>
            <p class="solution-promise">{body.title}</p>
            <p class="solution-description">{body.description}</p>
            {project_tags(project)}
            {contact_link(project, body, text, false)}
        </article>
    }
}

fn project_tags(project: &'static Project) -> impl IntoView {
    view! {
        <ul class="solution-tags">
            {project.tags.iter().map(|tag| view! { <li>{*tag}</li> }).collect::<Vec<_>>()}
        </ul>
    }
}

fn contact_link(
    project: &'static Project,
    body: &'static ProjectText,
    text: &'static PortfolioText,
    primary: bool,
) -> impl IntoView {
    let class = if primary {
        "btn primary solution-contact"
    } else {
        "solution-contact"
    };

    view! {
        <a class={class} href="#kontakt" data-testid={format!("solution-contact-{}", project.slug)}
            aria-label={format!("{}: {}", text.contact, body.name)}
            data-solution-brief={format!("{} {}: {}", text.interest, body.name, body.title)}>
            {text.contact}<span aria-hidden="true">"↗"</span>
        </a>
    }
}

fn copy(lang: Lang) -> &'static PortfolioText {
    match lang {
        Lang::Sk => &SK,
        Lang::En => &EN,
        Lang::Cs => &CS,
        Lang::De => &DE,
    }
}

const SK: PortfolioText = PortfolioText {
    eyebrow: "Naše riešenia",
    title: "Nástroje pre každodennú prácu",
    intro: "Vyvíjame vlastné produkty na správu úloh, spracovanie dát a vývoj softvéru. Môžeme ich prispôsobiť vašej firme alebo na ich základe pripraviť riešenie na mieru.",
    hero_link: "Jarvis pomôže s poštou, úlohami a poradami.",
    workflow_label: "Príklady využitia",
    workflow: [
        Pair { a: "Spracovanie pošty", b: "Jarvis vyberie podstatné informácie z prijatej správy a pripraví návrh odpovede." },
        Pair { a: "Práca s úlohami", b: "Cez Telegram alebo Slack si vyhľadáte úlohu v Notione a získate potrebné podrobnosti." },
        Pair { a: "Zápis z porady", b: "Zo zvukového záznamu vytvorí prepis a zhrnutie, ku ktorému sa môžete vrátiť." },
    ],
    daily_title: "Plánovanie práce a výkazy",
    more_title: "Nástroje na ďalšie úlohy",
    contact: "Mám záujem o riešenie",
    interest: "Zaujíma nás",
    projects: [
        ProjectText { name: "Jarvis", category: "Firemný AI asistent", title: "Pomoc s poštou, úlohami a poradami.", description: "Jarvis triedi poštu, pripravuje odpovede, vyhľadáva úlohy a spracúva záznamy z porád. Používa sa cez Telegram alebo Slack a prepája sa s Notionom a GitLabom, kde pomáha aj pri kontrole kódu." },
        ProjectText { name: "Cadence", category: "Organizácia práce tímu", title: "Plánovanie vydaní podľa kapacít tímu.", description: "Cadence dáva do súvislosti úlohy, termíny a dostupné kapacity. Pomáha s odhadom prácnosti, upozorňuje na blokácie a pripravuje podklady na rozhodnutie, čo zaradiť do najbližšieho vydania." },
        ProjectText { name: "Tally", category: "Spracovanie pracovných výkazov", title: "Mesačné výkazy v zákazníkovej šablóne.", description: "Tally prenáša odpracovaný čas z exportu do Excel šablóny zákazníka. Zapamätá si priradenie údajov, skontroluje súčty a eviduje úpravy. Dáta spracúva lokálne pomocou presných pravidiel." },
        ProjectText { name: "Forge", category: "Testovanie softvéru", title: "Overenie API aj nadväzujúcich systémov.", description: "Forge pripravuje a spúšťa testy podľa API špecifikácie alebo textového zadania. Overuje aj výsledky v databázach a frontoch a umožňuje zaradiť kontroly do procesu vydávania softvéru." },
        ProjectText { name: "MR Reviewer", category: "Kontrola zmien v kóde", title: "Posúdenie zmien pred ich zaradením.", description: "Pri kontrole zmien v GitLabe používa pravidlá tímu aj AI modely. Pripomienky priradí ku konkrétnym riadkom a nechá na vývojárovi, ktoré z nich zverejní." },
        ProjectText { name: "ArchGen", category: "Návrh systémov a procesov", title: "Diagramy a dokumentácia k návrhu.", description: "ArchGen pripravuje UML, BPMN a ArchiMate diagramy podľa zadania. K návrhu doplní dokumentáciu a umožní export pre Visio alebo Enterprise Architect." },
        ProjectText { name: "Mapovanie dát", category: "Dodávateľské dáta pre e-shop", title: "Dáta od dodávateľov priamo do e-shopu.", description: "Prepojí dodávateľské feedy s katalógom vášho e-shopu. Priradí zdrojové polia k produktovým údajom, zjednotí formáty a kategórie a pripraví dáta na import. Uložené mapovania a kontrola zmien zjednodušujú pravidelné aktualizácie." },
        ProjectText { name: "Parley / vox", category: "Spracovanie hovorenej reči", title: "Prepis a preklad počas porady.", description: "Parley spracúva zvuk porady lokálne a zobrazuje prepis aj preklad v titulkoch. Knižnica vox zabezpečuje príjem a prepis zvuku z mikrofónu aj počítača." },
        ProjectText { name: "Own IDE", category: "Prostredie pre vývojárov", title: "Práca s kódom a AI asistentom.", description: "Own IDE spája editor, navigáciu v kóde, testy, terminál a Git. AI asistent pracuje s kontextom projektu a nástroje sú dostupné aj pre agentov cez MCP." },
    ],
};

const EN: PortfolioText = PortfolioText {
    eyebrow: "Our solutions",
    title: "Tools for everyday work",
    intro: "We develop products for managing tasks, processing data and building software. We can adapt them to your business or use the experience to build a custom solution.",
    hero_link: "Jarvis helps with mail, tasks and meetings.",
    workflow_label: "Examples of use",
    workflow: [
        Pair { a: "Handling mail", b: "Jarvis picks out the important details in an incoming message and prepares a draft reply." },
        Pair { a: "Working with tasks", b: "Use Telegram or Slack to find a task in Notion and get the details you need." },
        Pair { a: "Meeting notes", b: "An audio recording becomes a transcript and summary you can refer to later." },
    ],
    daily_title: "Work planning and timesheets",
    more_title: "Tools for other tasks",
    contact: "Discuss this solution",
    interest: "We are interested in",
    projects: [
        ProjectText { name: "Jarvis", category: "Business AI assistant", title: "Help with mail, tasks and meetings.", description: "Jarvis sorts mail, drafts replies, finds tasks and processes meeting recordings. It works through Telegram or Slack and connects to Notion and GitLab, where it also helps review code." },
        ProjectText { name: "Cadence", category: "Organizing team work", title: "Release planning based on team capacity.", description: "Cadence brings tasks, deadlines and available capacity together. It helps estimate effort, flags blockers and provides the information needed to decide what belongs in the next release." },
        ProjectText { name: "Tally", category: "Work log processing", title: "Monthly timesheets in your customer's template.", description: "Tally transfers exported work logs into your customer's Excel template. It remembers field mappings, checks totals and records adjustments. Data is processed locally using exact rules." },
        ProjectText { name: "Forge", category: "Software testing", title: "Check APIs and connected systems.", description: "Forge prepares and runs tests from API specifications or written instructions. It also verifies results in databases and message queues and lets you include checks in your release process." },
        ProjectText { name: "MR Reviewer", category: "Reviewing code changes", title: "Assess changes before they are merged.", description: "It uses team rules and AI models to review changes in GitLab. Comments are tied to specific code lines, and the developer decides which to publish." },
        ProjectText { name: "ArchGen", category: "System and process design", title: "Diagrams and documentation for your design.", description: "ArchGen prepares UML, BPMN and ArchiMate diagrams from your requirements. It adds documentation to the design and supports export to Visio or Enterprise Architect." },
        ProjectText { name: "Data mapping", category: "Supplier data for your online store", title: "Supplier data directly into your online store.", description: "Connect supplier feeds to your online store's catalogue. Map source fields to product data, normalize formats and categories, and prepare data for import. Saved mappings and change checks simplify regular updates." },
        ProjectText { name: "Parley / vox", category: "Speech processing", title: "Transcription and translation during meetings.", description: "Parley processes meeting audio locally and displays transcripts and translations as captions. The vox library handles capture and transcription from the microphone and computer audio." },
        ProjectText { name: "Own IDE", category: "Development environment", title: "Work with code and an AI assistant.", description: "Own IDE combines an editor, code navigation, tests, terminal and Git. The AI assistant uses project context, and tools are also available to agents through MCP." },
    ],
};

const CS: PortfolioText = PortfolioText {
    eyebrow: "Naše řešení",
    title: "Nástroje pro každodenní práci",
    intro: "Vyvíjíme vlastní produkty pro správu úkolů, zpracování dat a vývoj softwaru. Můžeme je přizpůsobit vaší firmě nebo na jejich základě připravit řešení na míru.",
    hero_link: "Jarvis pomůže s poštou, úkoly a poradami.",
    workflow_label: "Příklady využití",
    workflow: [
        Pair { a: "Zpracování pošty", b: "Jarvis vybere podstatné informace z přijaté zprávy a připraví návrh odpovědi." },
        Pair { a: "Práce s úkoly", b: "Přes Telegram nebo Slack si vyhledáte úkol v Notionu a získáte potřebné podrobnosti." },
        Pair { a: "Zápis z porady", b: "Ze zvukového záznamu vytvoří přepis a shrnutí, ke kterému se můžete vrátit." },
    ],
    daily_title: "Plánování práce a výkazy",
    more_title: "Nástroje na další úkoly",
    contact: "Mám zájem o řešení",
    interest: "Zajímá nás",
    projects: [
        ProjectText { name: "Jarvis", category: "Firemní AI asistent", title: "Pomoc s poštou, úkoly a poradami.", description: "Jarvis třídí poštu, připravuje odpovědi, vyhledává úkoly a zpracovává záznamy z porad. Používá se přes Telegram nebo Slack a propojuje se s Notionem a GitLabem, kde pomáhá také při kontrole kódu." },
        ProjectText { name: "Cadence", category: "Organizace práce týmu", title: "Plánování vydání podle kapacit týmu.", description: "Cadence dává do souvislosti úkoly, termíny a dostupné kapacity. Pomáhá s odhadem pracnosti, upozorňuje na blokace a připravuje podklady pro rozhodnutí, co zařadit do nejbližšího vydání." },
        ProjectText { name: "Tally", category: "Zpracování pracovních výkazů", title: "Měsíční výkazy v zákazníkově šabloně.", description: "Tally přenáší odpracovaný čas z exportu do Excel šablony zákazníka. Zapamatuje si přiřazení údajů, zkontroluje součty a eviduje úpravy. Data zpracovává lokálně pomocí přesných pravidel." },
        ProjectText { name: "Forge", category: "Testování softwaru", title: "Ověření API i navazujících systémů.", description: "Forge připravuje a spouští testy podle API specifikace nebo textového zadání. Ověřuje také výsledky v databázích a frontách a umožňuje zařadit kontroly do procesu vydávání softwaru." },
        ProjectText { name: "MR Reviewer", category: "Kontrola změn v kódu", title: "Posouzení změn před jejich zařazením.", description: "Při kontrole změn v GitLabu používá pravidla týmu i AI modely. Připomínky přiřadí ke konkrétním řádkům a nechá na vývojáři, které z nich zveřejní." },
        ProjectText { name: "ArchGen", category: "Návrh systémů a procesů", title: "Diagramy a dokumentace k návrhu.", description: "ArchGen připravuje UML, BPMN a ArchiMate diagramy podle zadání. K návrhu doplní dokumentaci a umožní export pro Visio nebo Enterprise Architect." },
        ProjectText { name: "Mapování dat", category: "Dodavatelská data pro e-shop", title: "Data od dodavatelů přímo do e-shopu.", description: "Propojí dodavatelské feedy s katalogem vašeho e-shopu. Přiřadí zdrojová pole k produktovým údajům, sjednotí formáty a kategorie a připraví data k importu. Uložená mapování a kontrola změn zjednodušují pravidelné aktualizace." },
        ProjectText { name: "Parley / vox", category: "Zpracování mluvené řeči", title: "Přepis a překlad během porady.", description: "Parley zpracovává zvuk porady lokálně a zobrazuje přepis i překlad v titulcích. Knihovna vox zajišťuje příjem a přepis zvuku z mikrofonu i počítače." },
        ProjectText { name: "Own IDE", category: "Prostředí pro vývojáře", title: "Práce s kódem a AI asistentem.", description: "Own IDE spojuje editor, navigaci v kódu, testy, terminál a Git. AI asistent pracuje s kontextem projektu a nástroje jsou dostupné také pro agenty přes MCP." },
    ],
};

const DE: PortfolioText = PortfolioText {
    eyebrow: "Unsere Lösungen",
    title: "Werkzeuge für die tägliche Arbeit",
    intro: "Wir entwickeln eigene Produkte zur Aufgabenverwaltung, Datenverarbeitung und Softwareentwicklung. Wir können sie an Ihr Unternehmen anpassen oder auf dieser Grundlage eine individuelle Lösung erstellen.",
    hero_link: "Jarvis hilft bei Post, Aufgaben und Meetings.",
    workflow_label: "Anwendungsbeispiele",
    workflow: [
        Pair { a: "Post bearbeiten", b: "Jarvis liest die wichtigen Angaben aus einer Nachricht und bereitet einen Antwortentwurf vor." },
        Pair { a: "Mit Aufgaben arbeiten", b: "Über Telegram oder Slack finden Sie eine Aufgabe in Notion und die benötigten Einzelheiten." },
        Pair { a: "Meeting-Protokoll", b: "Aus einer Audioaufnahme entstehen ein Transkript und eine Zusammenfassung zum Nachlesen." },
    ],
    daily_title: "Arbeitsplanung und Stundennachweise",
    more_title: "Werkzeuge für weitere Aufgaben",
    contact: "Über diese Lösung sprechen",
    interest: "Wir interessieren uns für",
    projects: [
        ProjectText { name: "Jarvis", category: "KI-Assistent für Unternehmen", title: "Hilfe bei Post, Aufgaben und Meetings.", description: "Jarvis sortiert Post, entwirft Antworten, findet Aufgaben und verarbeitet Meeting-Aufnahmen. Er lässt sich über Telegram oder Slack nutzen und verbindet sich mit Notion und GitLab, wo er auch bei Code-Reviews hilft." },
        ProjectText { name: "Cadence", category: "Organisation der Teamarbeit", title: "Release-Planung nach Teamkapazität.", description: "Cadence bringt Aufgaben, Termine und verfügbare Kapazitäten zusammen. Es hilft bei Aufwandsschätzungen, zeigt Blockaden und liefert die Grundlage für die Entscheidung über den nächsten Release-Umfang." },
        ProjectText { name: "Tally", category: "Verarbeitung von Arbeitszeiten", title: "Monatliche Nachweise in der Kundenvorlage.", description: "Tally überträgt exportierte Arbeitszeiten in die Excel-Vorlage des Kunden. Es speichert Feldzuordnungen, prüft Summen und erfasst Anpassungen. Die Verarbeitung erfolgt lokal anhand genauer Regeln." },
        ProjectText { name: "Forge", category: "Softwaretests", title: "APIs und verbundene Systeme prüfen.", description: "Forge erstellt und startet Tests anhand von API-Spezifikationen oder schriftlichen Vorgaben. Es prüft auch Ergebnisse in Datenbanken und Nachrichtenwarteschlangen und bindet Kontrollen in den Release-Prozess ein." },
        ProjectText { name: "MR Reviewer", category: "Prüfung von Codeänderungen", title: "Änderungen vor der Übernahme bewerten.", description: "Bei der Prüfung von GitLab-Änderungen nutzt es Teamregeln und KI-Modelle. Hinweise werden konkreten Codezeilen zugeordnet. Der Entwickler entscheidet, welche Kommentare veröffentlicht werden." },
        ProjectText { name: "ArchGen", category: "System- und Prozessentwurf", title: "Diagramme und Dokumentation zum Entwurf.", description: "ArchGen erstellt UML-, BPMN- und ArchiMate-Diagramme anhand Ihrer Anforderungen. Es ergänzt die Dokumentation und ermöglicht den Export für Visio oder Enterprise Architect." },
        ProjectText { name: "Datenmapping", category: "Lieferantendaten für Ihren Online-Shop", title: "Lieferantendaten direkt in Ihren Online-Shop.", description: "Verbindet Lieferantenfeeds mit dem Katalog Ihres Online-Shops. Ordnet Quellfelder den Produktdaten zu, vereinheitlicht Formate und Kategorien und bereitet Daten für den Import vor. Gespeicherte Zuordnungen und Änderungsprüfungen vereinfachen regelmäßige Aktualisierungen." },
        ProjectText { name: "Parley / vox", category: "Sprachverarbeitung", title: "Transkription und Übersetzung im Meeting.", description: "Parley verarbeitet Meeting-Audio lokal und zeigt Transkript und Übersetzung als Untertitel an. Die Bibliothek vox erfasst und transkribiert Mikrofon- und Computeraudio." },
        ProjectText { name: "Own IDE", category: "Umgebung für Entwickler", title: "Mit Code und einem KI-Assistenten arbeiten.", description: "Own IDE verbindet Editor, Codenavigation, Tests, Terminal und Git. Der KI-Assistent arbeitet mit dem Projektkontext. Werkzeuge stehen auch Agenten über MCP zur Verfügung." },
    ],
};
