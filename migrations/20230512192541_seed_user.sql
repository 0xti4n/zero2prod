-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    '13ceaeae-30d8-41e1-8d58-4da6cd6fed27',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$UNlZnMy5mPVFkq8ZMWoEtA$0CO3KX18MawdMsgop++hFons0t/WHgWi5NdzIf258Rs'
)
