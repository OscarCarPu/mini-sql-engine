# Mini SQL Engine 

A mini SQL Engine on Rust 

## Core functionality (temporal plan)

- [x] Store data in memory (one table, typed columns, `CREATE TABLE`, `INSERT`, `INT`, `FLOAT`, `TEXT`, `BOOL`)
- [x] Interactive terminal (REPL)
- [ ] `SELECT` with column lists, `*`, aliases, `DISTINCT`
- [ ] `WHERE` with basic operators (`=`, `!=`, `<`, `>`, `AND`, `OR`, `NOT`, `IS NULL`)
- [ ] `ORDER BY` and `LIMIT`
- [ ] Aggregates: `COUNT`, `SUM`, `AVG`, `MIN`, `MAX`
- [ ] `GROUP BY` and `HAVING`
- [ ] NULL handling (three-valued logic)
- [ ] `UPDATE` and `DELETE`
- [ ] Persist data to disk (and load from CSV)
- [ ] Multiple tables in one database
- [ ] `PRIMARY KEY` and `FOREIGN KEY` (checked on `INSERT`, `UPDATE`, `DELETE`)
- [ ] Table aliases and qualified columns (`a.id`)
- [ ] `INNER JOIN` and `LEFT JOIN`
- [ ] `EXPLAIN`

## Constraints 

- Allowed libraries: `rustyline` (REPL), `csv`
- No `unwrap`, `expect`, `unsafe`.
- Testing 
