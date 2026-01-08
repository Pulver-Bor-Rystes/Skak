CREATE TABLE Friendship (
  user1         varchar(40)     NOT NULL,
  user2         varchar(40)     NOT NULL,
  PRIMARY KEY (user1, user2),
  FOREIGN KEY (user1) references User(username) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (user2) references User(username) ON DELETE CASCADE ON UPDATE CASCADE
);
