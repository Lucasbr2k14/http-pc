-- Add up migration script here
-- Add migration script here

-- Tabela de cargos(roles) que o usuário pode ter
CREATE TABLE IF NOT EXISTS roles (
    role_id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

-- Inserindo os primeiros cargos para os usuários

INSERT INTO roles(role_id, name) 
VALUES 
    (1, 'ADMIN'),
    (2, 'USER'),
    (3, 'REGULAR');


-- Criando a tabela de usuário
-- Com uma chave estrangeira para roles que seria o cargo que o usuário pode ter.

CREATE TABLE IF NOT EXISTS users (
    user_id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(150) UNIQUE NOT NULL,
    create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    role_id INT DEFAULT 3,
    uuid UUID NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    description TEXT,

    FOREIGN KEY (role_id) REFERENCES roles(role_id)
);
