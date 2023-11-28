CREATE TABLE tasks(
  id BIGINT PRIMARY KEY NOT NULL GENERATED ALWAYS AS IDENTITY,
  state SMALLINT NOT NULL DEFAULT 0,
  priority SMALLINT NOT NULL DEFAULT 100 CHECK (priority >= 0),
  attempt SMALLINT NOT NULL DEFAULT 0 CHECK (attempt >= 0),
  max_attempts SMALLINT NOT NULL CHECK (max_attempts > 0),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  attempted_at TIMESTAMPTZ,
  scheduled_at TIMESTAMPTZ,
  queue TEXT NOT NULL DEFAULT 'main' CHECK (char_length(queue) > 0 AND char_length(queue) < 128),
  name TEXT NOT NULL CHECK (char_length(name) > 0 AND char_length(name) < 128),
  payload JSONB NOT NULL
);

CREATE UNIQUE INDEX tasks_row_unique_idx ON tasks USING btree(name, payload);
CREATE INDEX tasks_fetch_idx ON tasks USING btree(state, queue, priority, scheduled_at, id);
