CREATE TABLE users (
  user_id SERIAL PRIMARY KEY,
  user_uuid UUID NOT NULL,
  hash VARCHAR(122),
  email VARCHAR(100) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  name VARCHAR NOT NULL
);

INSERT INTO users(user_uuid, hash, email, name) VALUES (
'7a1508e8-5e94-4743-8e3f-902fa23cc34f',
'edb01e159a4e3f3134861207f5fc5087',
'contact.lenne@gmail.com',
'Julien Lenne');