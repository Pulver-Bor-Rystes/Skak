CREATE TABLE Password (
  hash          varchar(64)     NOT NULL,
  owner         varchar(40)     NOT NULL,
  changed       timestamp       NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (owner),
  FOREIGN KEY (owner)           references User(username) ON DELETE CASCADE,
  CONSTRAINT password_chk_1     CHECK ((length(`hash`) >= 8) and (length(`hash`) < 64))
);
