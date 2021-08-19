Typescript & Svelte - Boilerplate
============================

### Introduktion
Jeg brød mig ikke så meget om Sapper, SvelteKit og andre lignende projekter, da de alle sammen føltes ret låst fast. 

Derfor lavede jeg min egen boilerplate! Features er blandt andet:
- live-reload
- custom env fil
- typescript og sass/scss!



### Yarn kommandoer
Jeg er begyndt at bruge yarn, da det bare er lidt nemmere...

Installér Yarn: `npm i -g yarn`

`yarn start` Kompilerer server koden, hvorefter den starter serveren.

`yarn dev` Starter serveren med nodemon

`yarn src` Kompilere server koden i `server/`

`yarn svelte` Kompilere Svelte kode

`yarn pretty` Formatere koden efter prettier's regler

`yarn clean` Fjerner alle mapper, som indeholder kompileret kode: `dist & web/compiled_svelte`



### Mappe struktur

    .
    ├── config                  # Her ligger alle konfigurationsfilerne.
    │   ├── .prettierrc         # Regeler for hvordan filerne skal se ud.
    │   ├── env.json            # En environment fil til serveren
    │   ├── rollup.config.js    # Rollup konfigurationsfil. Bør ikke røres ved!
    │   ├── tsconfig.json       # Typescript konfigurationsfil. Bør heller ikke røres ved!
    ├── dist                    # Her ligger den kompilerede server kode. Mappen skal bare have lov at ligge i fred.
    ├── src                     # Her ligger den rå server kode!
    ├── web                     # Mappen som holder på alt frontend relateret
    │   ├── compiled_svelte     # Navnet forklarer det lidt... (er offentlig)
    │   ├── public              # Alt i denne mappe er offentligt via get requests
    │   ├── svelte              # Rå Svelte kode med Typescript!
    │   └── views               # Her ligger der html filer, som bliver pre-genereret!
    └── compile.js              # Et script som kompilerer alt, som skal kompileres...

