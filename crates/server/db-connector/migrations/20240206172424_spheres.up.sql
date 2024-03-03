-- Add up migration script here

-- spheres
CREATE TABLE IF NOT EXISTS spheres (
    id UUID NOT NULL PRIMARY KEY,
    name varchar(255) NOT NULL,
    description varchar(255),
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- sphere_users
CREATE TABLE IF NOT EXISTS sphere_users (
    sphere_id UUID NOT NULL REFERENCES spheres(id),
    user_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- planets
CREATE TABLE IF NOT EXISTS planets (
    id UUID NOT NULL PRIMARY KEY,
    sphere_id UUID NOT NULL REFERENCES spheres(id),
    kind varchar(20) NOT NULL,
    name varchar(255) NOT NULL,
    description varchar(255),
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- planet_messages
CREATE TABLE IF NOT EXISTS planet_messages (
    id UUID NOT NULL PRIMARY KEY,
    planet_id UUID NOT NULL REFERENCES planets(id),
    content TEXT NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);
