-- Add up migration script here

-- chat_room
CREATE TABLE IF NOT EXISTS chat_rooms (
    id UUID NOT NULL PRIMARY KEY,
    name varchar(255) NOT NULL,
    description varchar(255),
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- chat_messages
CREATE TABLE IF NOT EXISTS chat_messages (
    id UUID NOT NULL PRIMARY KEY,
    room_id UUID NOT NULL REFERENCES chat_rooms(id),
    content TEXT NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- chat_members
CREATE TABLE IF NOT EXISTS chat_members (
    room_id UUID NOT NULL REFERENCES chat_rooms(id),
    user_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);
