Skakk
============================

### Kom godt i gang

Du har brug for at installere:
 - [NodeJS](https://nodejs.org) v12 eller v14, men ikke v16
 - [MongoDB Community](https://www.mongodb.com/try/download/community) Så du kan køre databasen lokalt
 - Eventuelt også [MongoDB Compass](https://www.mongodb.com/products/compass) så du kan observere databasen.


### Mappe struktur

    .
    ├── compiled_server         # Her ligger den kompilerede server kode. Mappen skal bare have lov at ligge i fred.
    ├── config
    │   ├── .prettierrc         # Regeler for hvordan filerne skal se ud.
    │   ├── env.json            # En environment fil til serveren, som bliver automatisk genereret
    │   ├── tsconfig.json       # Typescript konfigurationsfil. Bør heller ikke røres ved!
    │   ├── webpack.config.js   # Rollup konfigurationsfil. Bør ikke røres ved!
    ├── server                  # Her ligger serveren og koger
    ├── web                     # Mappen som holder på alt frontend relateret, kun mappen `compiled` er offentlig
    │   ├── compiled            # Navnet forklarer det lidt... (er offentlig)
    │   ├── css
    │   ├── js
    │   │   ├── svelte          # Alle svelte filer ligger her
    │   │   ├── app.js          # Der sker egentlig ikke så meget her, men den fungerer som et indgangspunkt
    │   │   ├── sveltegen.js    # Sørger for at man kan bruge forskellige svelte filer på hjemmesiden
    │   ├── static              # En mappe til scripts som ikke skal integreres med resten af klientkoden.
    │   └── views               # Her ligger de html filer som bruges til de forskellige sider :)
    └── compile.js              # Et script som kompilerer alt, som skal kompileres...
    └── ideas.med               # Kan bruges til at notere idéer uden at de bliver smidt op på github

