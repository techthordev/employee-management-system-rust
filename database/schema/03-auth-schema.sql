-- ==========================================
-- AUTH SCHEMA
-- ==========================================

CREATE SCHEMA IF NOT EXISTS auth;
ALTER SCHEMA auth OWNER TO rustconnector;
GRANT ALL ON SCHEMA auth TO rustconnector;

-- ------------------
-- USERS
-- ------------------
CREATE TABLE auth.users (
    id       BIGSERIAL PRIMARY KEY,
    username VARCHAR(50)  NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL,
    enabled  BOOLEAN      NOT NULL DEFAULT TRUE
);

-- ------------------
-- ROLES
-- ------------------
CREATE TABLE auth.roles (
    id   BIGSERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

-- ------------------
-- USER ↔ ROLES (N:M)
-- ------------------
CREATE TABLE auth.user_roles (
    user_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,
    PRIMARY KEY (user_id, role_id),
    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
        REFERENCES auth.users (id)
        ON DELETE CASCADE,
    CONSTRAINT fk_role
        FOREIGN KEY (role_id)
        REFERENCES auth.roles (id)
        ON DELETE CASCADE
);

-- Ownership
ALTER TABLE auth.users OWNER TO rustconnector;
ALTER TABLE auth.roles OWNER TO rustconnector;
ALTER TABLE auth.user_roles OWNER TO rustconnector;

-- Privileges
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA auth TO rustconnector;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA auth TO rustconnector;

ALTER DEFAULT PRIVILEGES IN SCHEMA auth
    GRANT ALL PRIVILEGES ON TABLES TO rustconnector;

ALTER DEFAULT PRIVILEGES IN SCHEMA auth
    GRANT ALL PRIVILEGES ON SEQUENCES TO rustconnector;
