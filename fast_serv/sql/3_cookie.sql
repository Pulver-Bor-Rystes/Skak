CREATE TABLE Cookie (
  id            int             NOT NULL AUTO_INCREMENT,
  hash          varchar(64)     NOT NULL,
  owner         varchar(40)     NOT NULL,
  expires       timestamp       NOT NULL DEFAULT ((now() + interval 1 day)),
  PRIMARY KEY (id),
  FOREIGN KEY (owner) references User(username) ON DELETE CASCADE,
  CONSTRAINT cookie_chk_1 CHECK (((length(`hash`) >= 8) and (length(`hash`) < 64)))
);
