CREATE TABLE sets(
  id VARCHAR(36) NOT NULL PRIMARY KEY DEFAULT (UUID()),
  user_id VARCHAR(36) NOT NULL,
  exercise_workout_id VARCHAR(36) NOT NULL,
  quality FLOAT NOT NULL, -- km or kg
  quantity FLOAT NOT NULL, -- time(second) or reps
  note VARCHAR(255),
  set_type ENUM('warmup', 'normal') NOT NULL,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  FOREIGN KEY (exercise_workout_id) REFERENCES exercise_workout(id) ON DELETE CASCADE
);
