# SQL documentation

## DDL (Data Definition Language)

#### CREATE
This DDL-statement exposes database objects, such as tables, views or indexes
```
CREATE TABLE IF NOT EXISTS table_name (
    user_id serial PRIMARY KEY,
    username VARCHAR ( 50 ) NOT NULL,
    last_login TIMESTAMP
);
```

#### ALTER
Statement uses to modify current database structure
```
ALTER TABLE old_table_name RENAME TO new_table_name;
```

#### DROP
Command is using to delete database objects, such as tables, views or indexes
```
DROP TABLE new_table_name;
```

## DML (Data Manipulation Language)

#### SELECT
Instruction to get tuples from a table
```
SELECT user_id, username FROM table_name;
```

#### INSERT INTO
Keyword uses to insert new row to a table
```
INSERT INTO table_name(user_id, username, last_login) VALUES(1, 'Ivan Petrov', NULL)
```

#### DELETE
Command to delete some row by specific rule
```
DELETE FROM table_name WHERE username = 'nick';
```

#### UPDATE
DML command to update table values
```
UPDATE table_name SET username = 'newnick' WHERE user_id = 1;
```

## DCL (Data Control Language)
Used to grant users access rights to the database
#### GRANT
```
GRANT INSERT ON my_table TO user;
```

#### REVOKE
User to ungrant users access rights to the database
```
REVOKE INSERT ON my_table TO user;
```

## TCL (Transaction Control Language)

#### BEGIN/COMMIT
Command to declare transaction. All commands inside of transaction or successfully completed or no one of command changing state.
```
BEGIN;
UPDATE my_table SET balance = balance - 100 WHERE used_id = 10;
UPDATE my_table SET balance = balance + 100 WHERE used_id = 20;
COMMIT;
```

#### ROLLBACK
Rolls back transaction all new changes
```
BEGIN;
UPDATE my_table SET balance = balance - 100 WHERE used_id = 10;
UPDATE my_table SET balance = balance + 100 WHERE used_id = 20;
ROLLBACK
COMMIT;
```
