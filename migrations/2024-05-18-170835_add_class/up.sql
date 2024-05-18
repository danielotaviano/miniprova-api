-- Your SQL goes here
CREATE TABLE classes (
    id SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    code VARCHAR NOT NULL,
    "description" TEXT NOT NULL,
    user_id INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES "users"(id),
    UNIQUE (code)
);

CREATE TABLE classes_students (
    class_id INT NOT NULL,
    student_id INT NOT NULL,
    FOREIGN KEY (class_id) REFERENCES classes(id),
    FOREIGN KEY (student_id) REFERENCES "users"(id),
    PRIMARY KEY (class_id, student_id)
);