-- Add migration script here

create table if not exists User (
    username        varchar(40),
    rating          int default 1000 not null,
    primary key (username),
    check ( length(username) > 3 )
);

create table if not exists Password (
    hash        varchar(64) not null check ( length(hash) >= 8 and length(hash) < 64 ),
    owner       varchar(40) references User on delete cascade,
    changed     timestamp not null default now(),
    primary key (owner)
);

create table if not exists Cookie (
    id          int auto_increment,
    hash        varchar(64) not null check ( length(hash) >= 8 and length(hash) < 64 ),
    owner       varchar(40) references User on delete cascade,
    expires     timestamp not null default (now() + interval 1 day),
    primary key (id)
);

create table if not exists Friendship (
    user1       varchar(40) references User on delete cascade,
    user2       varchar(40) references User  on delete cascade,
    primary key (user1, user2)
);

create trigger no_friendship_duplicates
before insert on Friendship
for each row
begin
    IF EXISTS (
        SELECT 1
        FROM Friendship
        WHERE user1 = NEW.user2
          AND user2 = NEW.user1
    ) THEN
        SIGNAL SQLSTATE '45000'
            SET MESSAGE_TEXT = 'Friendship already exists (reverse duplicate)';
    END IF;
end;
