# Macros

```bash
#[macro_export] -> used to export this macro outside its file
macro_rules! variable -> create your own custom macro
{
    ( pattern )=>{ body scope };
    ( pattern2 )=>{ body scope };
} -> block scope

```

# Each Pattern Must Be Unique
| Pattern      | Matches                                      |
| ------------ | -------------------------------------------- |
| `$x:expr`    | Any valid expression                         |
| `$x:ident`   | An identifier (e.g., variable/function name) |
| `$x:block`   | A block `{ ... }`                            |
| `$x:ty`      | A type (e.g., `i32`, `String`)               |
| `$x:tt`      | A single token tree                          |
| `$x:pat`     | A pattern (used in match arms, `let`, etc.)  |
| `$x:stmt`    | A single statement                           |
| `$x:literal` | A literal value (like `5` or `"hi"`)         |