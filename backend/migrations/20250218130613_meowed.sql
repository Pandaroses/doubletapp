-- Add migration script here
ALTER TABLE "user" 
    ALTER COLUMN password set not null,
    ALTER COLUMN username set not null;
