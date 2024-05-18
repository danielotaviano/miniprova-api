-- Your SQL goes here
CREATE TABLE exams (
    id SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "start_date" TIMESTAMP NOT NULL,
    end_date TIMESTAMP NOT NULL,
    class_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (class_id) REFERENCES classes(id)
);

CREATE TABLE questions (
    id SERIAL PRIMARY KEY,
    "question" VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE answers (
    id SERIAL PRIMARY KEY,
    "answer" VARCHAR NOT NULL,
    is_correct BOOLEAN NOT NULL,
    question_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (question_id) REFERENCES questions(id)
);

CREATE TABLE exam_questions (
    exam_id INT NOT NULL,
    question_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (exam_id) REFERENCES exams(id),
    FOREIGN KEY (question_id) REFERENCES questions(id),
    PRIMARY KEY (exam_id, question_id)
);