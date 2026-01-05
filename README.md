# Skak
Projektet er skrevet I så meget Rust som overhovedet muligt 🤞

Fuck JS haha

## Vigtigt
- wasm-pack skal installeres `cargo install wasm-pack`
- nginx kræves for at få det til at spille sammen *(se nginx afsnittet)*


## Projekt oversigt


```
Skak projekt

.
├─── bevy_skak
│      Et projekt skrevet i Rust og Bevy.
│      Pt. gør det ikke rigtig noget ud over at vise et skakbræt, hvor man kan rykke rundt på brikkerne.
│      Tasten 'X' vender brættet
│      💿 cargo run
│
│
├─── chess_machine_lib
│      Et program skrevet i Rust, som holder styr på skakspillet.
│      Hvis den bliver kompileret med featuren 'time', kan den også holde styr på tiden.
│      Bliver brugt af: bevy_skak, server og front
│      💿 wasm-pack build --target web
│      ‼️ Må IKKE kompileres med featuren time, hvis det skal fungere i webbrowseren
│   
│
├── engine
│      Benjamins første skak engine
│      Bruges af server, så spillere kan spille imod den.
│      💿 make optimized
│
│
├── fast_serv
│      Serveren eller backenden, som binder det hele sammen.
│      Skrevet i Rust med multithreading-hjælp fra frameworket Actix.
│      Kører på port: 4000
│      💿 cargo run
│      ‼️ Kan IKKE køre uden at chess_machine_lib, stockfish og engine er kompileret.
│
│   
├── nginx.conf
│      Konfigurations fil til nginx.
│
│
├── README.md
│
├── sisyphus32
│      Benjamins anden skak engine! 🤯
│      Bruges af server, så spillere kan spille imod den.
│      💿 make optimized
│
│
├── stockfish
│      Kræves af server, så spillere kan spille imod den.
│      💿 cd src && make -j build eller make -j profile-build (hvis man selv kompilerer koden)
│      ‼️ Er faktisk ikke med i github repo, så man skal selv hente det fra https://stockfishchess.org
│
│      
└── web
       Selve hjemmesiden - skrevet i Svelte og TailwindCSS
       Kører på port 3000
       💿 npm run dev
```


## NGINX Konfiguration
Erstat din NGINX conf fil med dette:

```config
worker_processes 1;

events {
        worker_connections 1024;
}

http {
        server {
                listen 80;
                server_name localhost;

                location /api {
                        proxy_pass http://localhost:4000;
                }

                location /api/ws {
                        proxy_set_header X-Real-IP $remote_addr;
                        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
                        proxy_set_header Host $http_host;
                        proxy_set_header X-NginX-Proxy false;

                        proxy_pass http://localhost:4000;
                        proxy_redirect off;

                        proxy_http_version 1.1;
                        proxy_set_header Upgrade $http_upgrade;
                        proxy_set_header Connection "upgrade";
                }

                location / {
                        proxy_pass http://localhost:3000;
                        proxy_http_version 1.1;
                        proxy_set_header Upgrade $http_upgrade;
                        proxy_set_header Connection "upgrade";
                }
        }
}
```
