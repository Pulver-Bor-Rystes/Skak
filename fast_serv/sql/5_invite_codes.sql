create table InviteCode (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    code TEXT NOT NULL,
    passcode TEXT,
    privilege ENUM('admin', 'user') NOT NULL DEFAULT 'user',

    used BOOLEAN NOT NULL DEFAULT FALSE,

    used_at TIMESTAMP,
    used_by varchar(40) REFERENCES User(username),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by varchar(40) NOT NULL REFERENCES User(username),

    FOREIGN KEY (used_by) REFERENCES User(username) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES User(username) ON DELETE CASCADE
);
