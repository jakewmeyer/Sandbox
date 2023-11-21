CREATE TABLE tasks(
  id UUID NOT NULL PRIMARY KEY,
  type TEXT NOT NULL,
  priority INT NOT NULL DEFAULT 0,
  payload JSONB NOT NULL,
  created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE(type, payload)
);
CREATE INDEX tasks_priority_idx ON tasks(priority);
