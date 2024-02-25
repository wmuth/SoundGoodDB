CREATE TABLE "instrument_types" (
  "instrument_type_id" int UNIQUE NOT NULL,
  "instrument_type" varchar(100) UNIQUE NOT NULL,
  PRIMARY KEY ("instrument_type_id")
);

CREATE TABLE "instruments" (
  "instrument_id" int GENERATED ALWAYS AS IDENTITY,
  "instrument_type_id" int NOT NULL,
  "brand" varchar(100) NOT NULL,
  "model" varchar(100) NOT NULL,
  "price" numeric(10, 2) NOT NULL,
  "count" int NOT NULL,
  PRIMARY KEY ("instrument_id"),
  CONSTRAINT "FK_instruments.instrument_type_id"
    FOREIGN KEY ("instrument_type_id")
      REFERENCES "instrument_types"("instrument_type_id")
);

CREATE TABLE "business_rules" (
  "business_rule_id" int GENERATED ALWAYS AS IDENTITY,
  "name" varchar(50) UNIQUE NOT NULL,
  "value" varchar(50) NOT NULL,
  PRIMARY KEY ("business_rule_id")
);

CREATE TABLE "addresses" (
  "address_id" int GENERATED ALWAYS AS IDENTITY,
  "line_1" varchar(100) NOT NULL,
  "line_2" varchar(100),
  "city" varchar(100) NOT NULL,
  "zip" varchar(5) NOT NULL,
  PRIMARY KEY ("address_id")
);

CREATE TABLE "person_details" (
  "person_details_id" int GENERATED ALWAYS AS IDENTITY,
  "name" varchar(100) NOT NULL,
  "ssn" varchar(12) UNIQUE,
  "address_id" int NOT NULL,
  "phone" varchar(12) NOT NULL,
  "email" varchar(100) NOT NULL,
  PRIMARY KEY ("person_details_id"),
  CONSTRAINT "FK_person_details.address_id"
    FOREIGN KEY ("address_id")
      REFERENCES "addresses"("address_id")
);

CREATE TABLE "students" (
  "student_id" int GENERATED ALWAYS AS IDENTITY,
  "person_details_id" int NOT NULL,
  PRIMARY KEY ("student_id"),
  CONSTRAINT "FK_students_person_details.id"
    FOREIGN KEY ("person_details_id")
      REFERENCES "person_details"(person_details_id)
);

CREATE TABLE "instructors" (
  "instructor_id" int GENERATED ALWAYS AS IDENTITY,
  "person_details_id" int NOT NULL,
  PRIMARY KEY ("instructor_id"),
  CONSTRAINT "FK_instructors_person_details.id"
    FOREIGN KEY ("person_details_id")
      REFERENCES "person_details"(person_details_id)
);

CREATE TABLE "rooms" (
  "room_id" int GENERATED ALWAYS AS IDENTITY,
  "room_number" int UNIQUE NOT NULL,
  "max_capacity" int NOT NULL,
  PRIMARY KEY ("room_id")
);

CREATE TABLE "lessons" (
  "lesson_id" int GENERATED ALWAYS AS IDENTITY,
  "room_id" int NOT NULL,
  "topic" varchar(100) NOT NULL,
  "skill_level" int NOT NULL,
  "min_places" int NOT NULL,
  "max_places" int NOT NULL,
  "genre" varchar(100),
  "start_date" timestamptz NOT NULL,
  "end_date" timestamptz NOT NULL,
  "cost" numeric(10, 2) NOT NULL,
  PRIMARY KEY ("lesson_id"),
  CONSTRAINT "FK_lessons.room_id"
    FOREIGN KEY ("room_id")
      REFERENCES "rooms"("room_id")
);

CREATE TABLE "rentings" (
  "rent_id" int GENERATED ALWAYS AS IDENTITY,
  "student_id" int NOT NULL,
  "instrument_id" int NOT NULL,
  "start_date" timestamptz NOT NULL,
  "end_date" timestamptz,
  PRIMARY KEY ("rent_id"),
  CONSTRAINT "FK_rentings.student_id"
    FOREIGN KEY ("student_id")
      REFERENCES "students"("student_id"),
  CONSTRAINT "FK_rentings.instrument_id"
    FOREIGN KEY ("instrument_id")
      REFERENCES "instruments"("instrument_id")
);

CREATE TABLE "skill_levels" (
  "skill_value" int UNIQUE,
  "skill_level" varchar(100) UNIQUE,
  PRIMARY KEY ("skill_value")
);

CREATE TABLE "contacts" (
  "contact_id" int GENERATED ALWAYS AS IDENTITY,
  "person_details_id" int NOT NULL,
  PRIMARY KEY ("contact_id"),
  CONSTRAINT "FK_contacts.person_details_id"
    FOREIGN KEY ("person_details_id")
      REFERENCES "person_details"("person_details_id")
);

CREATE TABLE "payments" (
  "payment_id" int GENERATED ALWAYS AS IDENTITY,
  "student_id" int,
  "instructor_id" int NOT NULL,
  "payment_for" varchar(100) NOT NULL,
  "amount" numeric(10, 2) NOT NULL,
  "due_date" timestamptz NOT NULL,
  "paid" boolean NOT NULL,
  "outgoing" boolean NOT NULL,
  PRIMARY KEY ("payment_id"),
  CONSTRAINT "FK_payments.student_id"
    FOREIGN KEY ("student_id")
      REFERENCES "students"("student_id"),
  CONSTRAINT "FK_payments.instructor_id"
    FOREIGN KEY ("instructor_id")
      REFERENCES "instructors"("instructor_id")
);

CREATE TABLE "siblings" (
  "first_student_id" int NOT NULL,
  "second_student_id" int NOT NULL,
  PRIMARY KEY ("first_student_id", "second_student_id"),
  CONSTRAINT "FK_siblings.second_student_id"
    FOREIGN KEY ("second_student_id")
      REFERENCES "students"("student_id"),
  CONSTRAINT "FK_siblings.first_student_id"
    FOREIGN KEY ("first_student_id")
      REFERENCES "students"("student_id")
);

CREATE TABLE "student_contacts" (
  "student_id" int NOT NULL,
  "contact_id" int NOT NULL,
  PRIMARY KEY ("student_id", "contact_id"),
  CONSTRAINT "FK_student_contacts.student_id"
    FOREIGN KEY ("student_id")
      REFERENCES "students"("student_id"),
  CONSTRAINT "FK_student_contacts.contact_id"
    FOREIGN KEY ("contact_id")
      REFERENCES "contacts"("contact_id")
);

CREATE TABLE "students_lesson" (
  "student_id" int NOT NULL,
  "lesson_id" int NOT NULL,
  PRIMARY KEY ("student_id", "lesson_id"),
  CONSTRAINT "FK_students_lesson.lesson_id"
    FOREIGN KEY ("lesson_id")
      REFERENCES "lessons"("lesson_id"),
  CONSTRAINT "FK_students_lesson.student_id"
    FOREIGN KEY ("student_id")
      REFERENCES "students"("student_id")
);

CREATE TABLE "instructors_lesson" (
  "instructor_id" int NOT NULL,
  "lesson_id" int NOT NULL,
  PRIMARY KEY ("instructor_id", "lesson_id"),
  CONSTRAINT "FK_instructors_lesson.lesson_id"
    FOREIGN KEY ("lesson_id")
      REFERENCES "lessons"("lesson_id"),
  CONSTRAINT "FK_instructors_lesson.instructor_id"
    FOREIGN KEY ("instructor_id")
      REFERENCES "instructors"("instructor_id")
);

CREATE TABLE "student_instruments" (
  "student_id" int NOT NULL,
  "instrument_type_id" int NOT NULL,
  PRIMARY KEY ("student_id", "instrument_type_id"),
  CONSTRAINT "FK_student_instruments.instrument_type_id"
    FOREIGN KEY ("instrument_type_id")
      REFERENCES "instrument_types"("instrument_type_id"),
  CONSTRAINT "FK_student_instruments.student_id"
    FOREIGN KEY ("student_id")
      REFERENCES "students"("student_id")
);

CREATE TABLE "instructor_instruments" (
  "instructor_id" int NOT NULL,
  "instrument_type_id" int NOT NULL,
  PRIMARY KEY ("instructor_id", "instrument_type_id"),
  CONSTRAINT "FK_instructor_instruments.instrument_type_id"
    FOREIGN KEY ("instrument_type_id")
      REFERENCES "instrument_types"("instrument_type_id"),
  CONSTRAINT "FK_instructor_instruments.instructor_id"
    FOREIGN KEY ("instructor_id")
      REFERENCES "instructors"("instructor_id")
);

CREATE TABLE "student_skills" (
  "student_id" int NOT NULL,
  "skill_value" int NOT NULL,
  "instrument_type_id" int NOT NULL,
  PRIMARY KEY ("student_id", "skill_value", "instrument_type_id"),
  CONSTRAINT "FK_student_skills.skill_value"
    FOREIGN KEY ("skill_value")
      REFERENCES "skill_levels"("skill_value"),
  CONSTRAINT "FK_student_skills.instrument_type_id"
    FOREIGN KEY ("instrument_type_id")
      REFERENCES "instrument_types"("instrument_type_id"),
  CONSTRAINT "FK_student_skills.student_id"
    FOREIGN KEY ("student_id")
      REFERENCES "students"("student_id")
);
