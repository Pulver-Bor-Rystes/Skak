Skakk
============================



### Påkrævet
 - NodeJS
 - NGINX *(bruges som reverse proxy)*
 - Mongodb



### Mappe struktur

En oversigt over hvor alt det spændende sker :))

    .
    ├── server
    │   ├── models
    │   │   └── user.model.ts
    │   ├── routes
    │   │   ├── legacy
    │   │   │   └── example.router.ts
    │   │   └── v1
    │   │       ├── auth.router.ts
    │   │       ├── testing.router.ts
    │   │       ├── user_info.router.ts
    │   │       └── users.router.ts
    │   ├── main.ts - alt socket.io halløj sker pt. her
    ├── shared - Idéelt skal alt kode, som skal bruges af både server og klient placeres her.
    │   ├── functions
    │   ├── functions.ts
    │   └── types.ts
    ├── website
    │   ├── src
    │   │   ├── components
    │   │   ├── routes
    │   │   ├── stores
    │   │   └── app.html
    │   ├── static
    │   │   └── favicon.png

