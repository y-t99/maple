# maple db

To learn Rust.

## string

1. `GET key`: Get the value of key. If the key does not exist the special value ? is returned. 
2. `GETDEL key`: Get the value of key and delete the key. This command is similar to GET, except for the fact that it also deletes the key on success.
3. `SET key value`: Set key to hold the string value. If key already holds a value, it is overwritten.
4. `STRLEN key value`: Returns the length of the string value stored at key. If the key does not exist the special value ? is returned. 