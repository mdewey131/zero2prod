-- migrations/{timestamp}_create_subscriptions_table.sql
--- Creates the subscriptions table
CREATE TABLE subscriptions(
    id uuid NOT NULL
    , PRIMARY KEY (id)
    , email TEXT NOT NULL UNIQUE
    , name TEXT NOT NULL 
    , subscribed_at timestamptz NOT NULL 
);