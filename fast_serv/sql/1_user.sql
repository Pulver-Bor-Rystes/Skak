create table User (
    username    varchar(40)     not null,
    rating      int             not null default 1000,
    primary key (username),
    constraint username_check check (length(username) > 3)
);
