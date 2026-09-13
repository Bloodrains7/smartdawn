# Smart Dawn

Firemný web pre kompletné dodávky softvéru na mieru. Rust, Leptos SSR a Axum poskytujú HTML; samostatný Rust modul vo WebAssembly počíta pohyb častíc a ich vzájomné prepojenia. Canvas vykresľuje výsledný graf. Obsah a navigácia fungujú aj bez JavaScriptu alebo WASM.

Úvod obsahuje výber webového produktu, firemného systému alebo automatizácie. Voliteľné integrácie a AI funkcie sa prenášajú do zadania spolu s vybranými modulmi. Prenos zachováva vlastný text návštevníka a nič automaticky neodosiela. Jazykové verzie: `/`, `/en`, `/cz`, `/de`.

Portfólio predstavuje Jarvis ako hlavné riešenie pre firemnú operatívu, s odkazom z úvodu a tromi príkladmi využitia. Nasledujú Cadence pre plánovanie práce a Tally pre výkazy, potom Forge, MR Reviewer, ArchGen, Mapovanie dát pre internetové obchody, Parley / vox a Own IDE. Obsah a poradie produktov sú v `src/portfolio.rs`, vo všetkých štyroch jazykoch.

Odkazy pri riešeniach doplnia názov a zameranie produktu do kontaktného zadania. Výber iného riešenia alebo dodávky nahradí iba predchádzajúci vložený text a zachová dopísanú správu. Bez JavaScriptu odkazy vedú priamo na kontaktný formulár. `e2e/tests/portfolio.spec.mjs` overuje poradie, preklady, každé riešenie, klávesnicu aj zobrazenie bez JavaScriptu. Playwright ho automaticky zahŕňa cez `testMatch` v `playwright.config.mjs`. Náhľady portfólia sú po teste v `artifacts/portfolio-{360,768,1440}.png`; mutačné kontroly sú súčasťou `npm run test:mutations`.

## Lokálne spustenie

```powershell
cargo run --locked
```

Web je dostupný na `http://localhost:3000`. `PORT` je voliteľný, predvolene 3000. `SITE_URL` nastavuje verejnú HTTPS adresu pre canonical, sitemap a Open Graph. Predvolená hodnota je `https://www.smartdawn.eu`. `RUST_LOG` nastavuje logovanie.

## Railway

Railway podporuje tento web cez priložený `Dockerfile`. Server počúva na `0.0.0.0:$PORT`; `railway.json` nastavuje kontrolu nasadenia na `/healthz`. WASM sa vykonáva v prehliadači návštevníka, takže nepotrebuje samostatnú službu.

1. V Railway vytvorte **New Project → Deploy from GitHub repo** a vyberte tento repozitár.
2. Railway zostaví Docker image vrátane aktuálneho WASM modulu zo zdrojov.
3. V **Settings → Networking → Generate Domain** vytvorte verejnú adresu.
4. Vo **Variables** nastavte `SITE_URL` na túto HTTPS adresu bez koncového lomítka.
5. Pre vlastnú doménu použite **Custom Domain**, nastavte DNS záznam podľa Railway a zmeňte `SITE_URL` na finálnu adresu. HTTPS zabezpečuje Railway.

Nepridávajte vlastný Start Command; image má pripravený `CMD`. Nie je potrebná databáza ani volume pre samotný web. Railway je platený hosting podľa aktuálneho programu a spotreby; WASM nepridáva osobitnú serverovú službu.

Dokumentácia: [Dockerfiles](https://docs.railway.com/builds/dockerfiles), [domény](https://docs.railway.com/guides/public-networking), [konfigurácia](https://docs.railway.com/config-as-code/reference).

Lokálne overenie rovnakého image:

```powershell
docker build --tag smart-dawn-web:preview .
docker run --rm --publish 3108:8080 --env PORT=8080 --env SITE_URL=http://localhost:3108 smart-dawn-web:preview
```

Otvorte `http://localhost:3108/`. `/healthz` musí odpovedať `200 ok`, `/assets/hero.wasm` musí mať `Content-Type: application/wasm`. `.dockerignore` povoľuje iba zdroje aplikácie a verejné assets; pracovné dokumenty a súbory z koreňa repozitára sa do image nekopírujú.

Po zostavení image spustí `npm run test:container` automatickú kontrolu na náhodnom lokálnom porte a testovací kontajner po skončení odstráni.

## Vercel

`Dockerfile.vercel` zostaví Rust server aj WASM a vloží priečinok `assets` do finálneho image. `vercel.json` smeruje všetky cesty vrátane `/assets/*`, jazykových verzií a `/contact` do rovnakého Axum servera. Predvolený port image je 80, čo zodpovedá smerovaniu Vercelu. Pracovný adresár `/app` je potrebný pre relatívnu cestu `ServeDir::new("assets")`.

1. V projekte Vercel nastavte **Root Directory** na koreň tohto repozitára, kde sú `Cargo.toml` a `Dockerfile.vercel`.
2. Odstráňte prípadné staré vlastné nastavenia **Build Command**, **Install Command** a **Output Directory** pre statický web. Kontajner zostavuje aplikáciu cez Cargo.
3. Nastavte `SITE_URL` na finálnu HTTPS adresu. `PORT` nechajte nenastavený; pri vlastnej hodnote ho nastavte v premenných projektu, aby rovnaký port používalo aj smerovanie Vercelu.
4. Nasaďte verziu repozitára obsahujúcu oba konfiguračné súbory. Pri nasadení cez CLI používa `.vercelignore` zoznam povolených zdrojov; pracovné dokumenty, lokálne buildy a testy sa nenahrávajú.
5. Overte `/healthz`, načítanie `/assets/style.css` ako `text/css` a `/assets/hero.wasm` ako `application/wasm`. Odpoveď `200` s HTML namiesto CSS znamená nesprávne smerovanie; `404` znamená, že súbor nie je dostupný.

Lokálne overenie kontajnera vrátane celej Playwright sady:

```powershell
docker build --file Dockerfile.vercel --tag smart-dawn-web:vercel-preview .
npm run test:vercel
node e2e/check-container.mjs --vercel --port-override
```

Prvý beh overí predvolený port 80, správne štýly na mobile aj desktope, JavaScript, WASM a všetky existujúce scenáre. Druhý overí zmenu portu cez `PORT=8080`. Testy po skončení odstránia vlastný kontajner. Toto overuje výsledný image lokálne; smerovanie a nastavenia účtu treba overiť aj na URL Vercelu.

Dokumentácia: [Rust a Docker na Verceli](https://vercel.com/kb/guide/deploy-rust-on-vercel-with-docker), [súbory nasadenia](https://vercel.com/docs/deployments/vercel-ignore).

## WASM

Pri zmene `hero-wasm/src/lib.rs` obnovte lokálny súbor:

```powershell
rustup target add wasm32-unknown-unknown
cargo build --manifest-path hero-wasm/Cargo.toml --target wasm32-unknown-unknown --release --locked
Copy-Item hero-wasm/target/wasm32-unknown-unknown/release/hero_wasm.wasm assets/hero.wasm
```

Pole používa 70 až 170 častíc podľa šírky pri načítaní. WASM počíta fyziku a proximity graf; JavaScript vykresľuje iba existujúce hrany. Toto rozdelenie nepredstavuje benchmark ani tvrdenie, že každý výpočet je vo WASM rýchlejší. Animácia sa zastaví mimo obrazovky, v skrytej karte alebo ovládacím tlačidlom. Pri zapnutom `prefers-reduced-motion` počas načítania sa WASM ani nesťahuje.

## Overenie

```powershell
npm ci
npx playwright install chromium
npm test
npm run test:e2e
npm run test:mutations
```

Playwright spúšťa vlastný Rust server na porte 3107. Scenáre overujú všetky štyri jazyky, tri typy dodávky, zapnuté aj vypnuté doplnky, zachovanie zadania, pozastavenie animácie, chýbajúce WASM/JS a rozloženie pri šírkach 360/768/1440 px. Testy používajú vlastné údaje a neodosielajú kontaktné správy. WASM testy načítajú skutočný binárny modul a overia hrany, opacity, limity a zmeny polohy. Mutačný beh podstrčí chybné správanie cez Playwright routing; zdroje nemení. Screenshoty a výstupy mutácií sú v ignorovanom `artifacts/`.

## Pred verejným spustením

Kontaktný endpoint `/contact` je pôvodný prototyp: validuje vstup a zapisuje metadáta do logu, ale správu neposiela ani trvalo neukladá. Zobrazené potvrdenie preto zatiaľ nepotvrdzuje doručenie. Pred spustením treba pripojiť doručovanie, správne spracovať jeho zlyhanie a overiť schránku `hello@smartdawn.eu`. Overte tiež finálnu doménu a firemné údaje.
