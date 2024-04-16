CREATE TABLE targets(
  id VARCHAR(36) NOT NULL UNIQUE PRIMARY KEY DEFAULT (UUID()),
  name VARCHAR(36) NOT NULL,
  sort int NOT NULL
);

INSERT INTO targets (id, name, sort) VALUES ('93f25eb0-6057-4db4-9281-684440aaab77', 'Shoulders', 1),
  ('f490e053-f337-4c8a-8f04-12d4fef418b2', 'Chest', 2),
  ('0bb07d10-1cdb-4df8-a6e2-614e3c46d992', 'Upper back', 3),
  ('722c7081-d7f3-4d20-a236-42cc7e7b63b3', 'Lower back', 4),
  ('46bd72d3-1035-4616-87d6-820364728211', 'Core', 5),
  ('8bb01634-0995-4e35-ab10-6e05d7999a35', 'Thighs', 6),
  ('dbe6e212-80d9-4a41-be09-77e0f67ed943', 'Calfs', 7),
  ('ec791df4-ccae-43fd-be25-f15033efe958', 'Biceps', 8),
  ('4694ebc3-1c92-4417-8e4d-2ffea39140f2', 'Triceps', 9),
  ('62debec8-748c-4757-b65d-0d13f8b7d90f', 'Lower arms', 10),
  ('bce1c62d-7fb1-431b-9bc4-f7b839a495d0', 'Cardio', 11);
