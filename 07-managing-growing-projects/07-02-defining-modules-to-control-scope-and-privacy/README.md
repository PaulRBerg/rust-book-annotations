# Modules

_src/main.rs_ and _src/lib.rs_ are called crate roots. The reason for their name is that the contents of either of these
two files form a module named crate at the root of the crate’s module structure, known as the module tree.

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
