-- Add migration script here
CREATE TABLE subscriptions (
    id serial PRIMARY KEY,
    name VARCHAR (50) NOT NULL,
    password VARCHAR (255) NOT NULL,
    email VARCHAR (100) UNIQUE NOT NULL,
    last_logined_at TIMESTAMP NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT NULL 
)