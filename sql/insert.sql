INSERT INTO instrument_types values (1, 'guitar');
INSERT INTO instrument_types values (2, 'piano');

INSERT INTO instruments (instrument_type_id, brand, model, price, count) values (1, 'Gibson', 'J-45 Studio Walnut', 101.01, 1);
INSERT INTO instruments (instrument_type_id, brand, model, price, count) values (2, 'Steinway & Sons', 'K-132', 202.02, 2);
INSERT INTO instruments (instrument_type_id, brand, model, price, count) values (1, 'Taylor', '414ce', 303.03, 3);
INSERT INTO instruments (instrument_type_id, brand, model, price, count) values (2, 'Sauter', 'Alpha 160', 404.04, 4);

INSERT INTO business_rules (name, value) values ('rent_max_count', '2');
INSERT INTO business_rules (name, value) values ('rent_max_time', '12');

INSERT INTO addresses (line_1, line_2, city, zip) values ('Ap #231-7514 Tellus. Rd.','Luctus Ltd','Gävle','43974');
INSERT INTO addresses (line_1, city, zip) values ('482-660 Ut Rd.','Hudiksvall','36473');
INSERT INTO addresses (line_1, city, zip) values ('Svea Vägen 1111','Stockholm','36213');
INSERT INTO addresses (line_1, city, zip) values ('Danmarksvägen 11','Malmö','36083');
INSERT INTO addresses (line_1, city, zip) values ('11 Allén','Göteborg','12345');

INSERT INTO person_details (name, ssn, address_id, phone, email) values ('Liberty Melton', '190001011234', 1, '07744 88973', 'ac.eleifend@outlook.edu');
INSERT INTO person_details (name, address_id, phone, email) values ('Leila Kerr', 2, '+46707070707', 'duis@icloud.com');
INSERT INTO person_details (name, ssn, address_id, phone, email) values ('Chelsea Skinner', '190001012345', 3, '07744 37988', 'ac.ddddd@outlook.edu');
INSERT INTO person_details (name, address_id, phone, email) values ('Kerr Orli', 4, '+46707070708', 'babldl@icloud.com');
INSERT INTO person_details (name, ssn, address_id, phone, email) values ('Contact Melton', '200001011235', 5, '88973 0744', 'kth.eleifend@outlook.edu');
INSERT INTO person_details (name, ssn, address_id, phone, email) values ('Student Studentsson', '211001011235', 5, '88973 0744', 'kth.ddna@outlook.edu');

INSERT INTO students (person_details_id) values (1);
INSERT INTO students (person_details_id) values (2);
INSERT INTO students (person_details_id) values (5);

INSERT INTO instructors (person_details_id) values (3);
INSERT INTO instructors (person_details_id) values (4);

INSERT INTO rooms (room_number, max_capacity) values (101, 30);
INSERT INTO rooms (room_number, max_capacity) values (201, 40);

INSERT INTO lessons (room_id, topic, skill_level, min_places, max_places, start_date, end_date, cost) values (1, 'basic guitar', 0, 1, 1, '2022-05-20 15:00:10-09', '2022-05-20 16:00:10-09', 101.10);
INSERT INTO lessons (room_id, topic, skill_level, min_places, max_places, genre, start_date, end_date, cost) values (2, 'basic piano', 2, 10, 40, 'classic piano', '2022-05-20 15:00:10-09', '2022-05-20 16:00:10-09', 202.20);

INSERT INTO rentings (student_id, instrument_id, start_date) values (1, 2, '2022-04-01 00:00:00-09');
INSERT INTO rentings (student_id, instrument_id, start_date, end_date) values (2, 2, '2022-04-01 00:00:00-09', '2022-05-15 00:00:00-09');

INSERT INTO skill_levels (skill_value, skill_level) values (0, 'Beginner');
INSERT INTO skill_levels (skill_value, skill_level) values (1, 'Intermediate');
INSERT INTO skill_levels (skill_value, skill_level) values (2, 'Advanced');

INSERT INTO contacts (person_details_id) values (5);

INSERT INTO payments (student_id, instructor_id, payment_for, amount, due_date, paid, outgoing) values (1, 1, 'basic guitar teaching', 1010101.10, '2022-05-31 23:59:00-09', false, false);
INSERT INTO payments (instructor_id, payment_for, amount, due_date, paid, outgoing) values (1, 'basic guitar teaching', 0.01, '2022-05-31 23:59:00-09', false, true);

INSERT INTO siblings (first_student_id, second_student_id) values (1, 2);

INSERT INTO student_contacts (student_id, contact_id) values (1, 1);

INSERT INTO students_lesson (student_id, lesson_id) values (1, 1);
INSERT INTO students_lesson (student_id, lesson_id) values (2, 2);

INSERT INTO instructors_lesson (instructor_id, lesson_id) values (1, 1);
INSERT INTO instructors_lesson (instructor_id, lesson_id) values (2, 2);

INSERT INTO student_instruments (student_id, instrument_type_id) values (1, 1);
INSERT INTO student_instruments (student_id, instrument_type_id) values (2, 2);

INSERT INTO instructor_instruments (instructor_id, instrument_type_id) values (1, 1);
INSERT INTO instructor_instruments (instructor_id, instrument_type_id) values (2, 2);

INSERT INTO Student_skills (student_id, skill_value, instrument_type_id) values (1, 0, 1);
INSERT INTO Student_skills (student_id, skill_value, instrument_type_id) values (2, 2, 2);
