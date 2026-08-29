-- Add migration script here

-- Tabela de cargos(roles) que o usuário pode ter
CREATE TABLE IF NOT EXISTS roles (
    role_id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

-- Inserindo os primeiros cargos para os usuários

INSERT INTO roles(name) 
VALUES 
    ('ADMIN'),
    ('REGULAR'),
    ('USER');


-- Criando a tabela de usuário
-- Com uma chave estrangeira para roles que seria o cargo que o usuário pode ter.

CREATE TABLE IF NOT EXISTS users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(150) UNIQUE NOT NULL,
    create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    role_id INT,
    uuid TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    description TEXT,
    -- Chaves chave da role
    FOREIGN KEY (role_id) REFERENCES roles(role_id)
);
