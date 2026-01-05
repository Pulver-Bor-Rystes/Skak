CREATE TABLE `User` (
  `username` varchar(40) NOT NULL,
  `rating` int NOT NULL DEFAULT '1000',
  PRIMARY KEY (`username`),
  CONSTRAINT `user_chk_1` CHECK ((length(`username`) > 3))
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci


CREATE TABLE `Friendship` (
  `user1` varchar(40) NOT NULL,
  `user2` varchar(40) NOT NULL,
  PRIMARY KEY (`user1`,`user2`),
  FOREIGN KEY (user1) references User(username) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (user2) references User(username) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci


CREATE TABLE `Cookie` (
  `id` int NOT NULL AUTO_INCREMENT,
  `hash` varchar(64) NOT NULL,
  `owner` varchar(40) DEFAULT NULL,
  `expires` timestamp NOT NULL DEFAULT ((now() + interval 1 day)),
  PRIMARY KEY (`id`),
  FOREIGN KEY (owner) references User(username) ON DELETE CASCADE,
  CONSTRAINT `cookie_chk_1` CHECK (((length(`hash`) >= 8) and (length(`hash`) < 64)))
) ENGINE=InnoDB AUTO_INCREMENT=25 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci


CREATE TABLE `Password` (
  `hash` varchar(64) NOT NULL,
  `owner` varchar(40) NOT NULL,
  `changed` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`owner`),
  FOREIGN KEY (owner) references User(username) ON DELETE CASCADE
  CONSTRAINT `password_chk_1` CHECK (((length(`hash`) >= 8) and (length(`hash`) < 64)))
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci



