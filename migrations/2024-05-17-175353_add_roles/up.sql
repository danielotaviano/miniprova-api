-- Your SQL goes here
CREATE TABLE roles (
  "name" TEXT NOT NULL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL
);

CREATE TABLE users_roles (
  user_id INT NOT NULL,
  role_name TEXT NOT NULL,
  PRIMARY KEY (user_id, role_name),
  FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
  FOREIGN KEY (role_name) REFERENCES roles (name) ON DELETE CASCADE
);

INSERT INTO roles (name, created_at) VALUES ('ADMIN', NOW());
INSERT INTO roles (name, created_at) VALUES ('STUDENT', NOW());
INSERT INTO roles (name, created_at) VALUES ('TEACHER', NOW());
INSERT INTO roles (name, created_at) VALUES ('MONITOR', NOW());