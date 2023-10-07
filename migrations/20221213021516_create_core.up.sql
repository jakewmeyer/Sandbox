CREATE TABLE accounts(
  id UUID NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  status INT NOT NULL DEFAULT 0,
  created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted TIMESTAMPTZ
);

CREATE TABLE users(
  id UUID NOT NULL PRIMARY KEY,
  provider_id TEXT,
  stripe_customer_id TEXT NOT NULL,
  created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted TIMESTAMPTZ
);
CREATE UNIQUE INDEX users_provider_id_idx ON users(provider_id);

CREATE TABLE users_accounts(
  user_id UUID NOT NULL REFERENCES users(id),
  account_id UUID NOT NULL REFERENCES accounts(id),
  created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted TIMESTAMPTZ,
  PRIMARY KEY (user_id, account_id)
);
CREATE UNIQUE INDEX users_accounts_account_id_idx ON users_accounts(account_id);

CREATE TABLE subscriptions(
  id UUID NOT NULL PRIMARY KEY,
  account_id UUID NOT NULL REFERENCES accounts(id),
  stripe_subscription_id TEXT NOT NULL,
  created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted TIMESTAMPTZ
);
CREATE UNIQUE INDEX subscriptions_stripe_subscription_id_idx ON subscriptions(stripe_subscription_id);

INSERT INTO accounts(id, name, status) VALUES
('018ae25a-2c34-738b-b75e-c72e8e1f13a3', 'Jake''s Lawn Care', 1);

INSERT INTO users(id, provider_id, stripe_customer_id) VALUES
('018ae25a-725e-7a81-9b80-8292272c29b8', 'auth0|640697bfd9f505ef159beb14', 'cus_GkkgWE9SjOw326');

INSERT INTO users_accounts(user_id, account_id) VALUES
('018ae25a-725e-7a81-9b80-8292272c29b8', '018ae25a-2c34-738b-b75e-c72e8e1f13a3');
