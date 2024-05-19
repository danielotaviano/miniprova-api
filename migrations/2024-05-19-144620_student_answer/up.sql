-- Your SQL goes here


CREATE TABLE student_answers (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    exam_id INT NOT NULL,
    question_id INT NOT NULL,
    answer_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (exam_id) REFERENCES exams(id),
    FOREIGN KEY (question_id) REFERENCES questions(id),
    FOREIGN KEY (answer_id) REFERENCES answers(id),
    UNIQUE (user_id, exam_id, question_id)
);