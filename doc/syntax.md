#Syntax of N

Language syntax delimits on two parts.

The first part includes data manipulation and algorithm languages.
It let you write some code like if-conditions, while- and for-cycles,
variable assignment and modification, database queries and more.  

The second part is data definition language. It let you define your structures,
enums and tables by cool and simple code.

# Data manipulation and algorithm languages

As said early, data manipulation language delimits on algorithm and data manipulation languages.
Let see what is this means.

## Data manipulation language

DML of N is very very like MySQL's DML. Look on `SELECT`-statement syntax.

```
select_stmt = SELECT select_modifiers select_expressions
    FROM select_source
    [select_where_clause]
    [select_group_by_clause]
    [select_having_clause]
    [select_order_by_clause]
    [select_limit_clause]
```

Where `select_` is rules of parser. Lets look on their syntax. 

```
select_modifiers =
    select_distinct_mod
    [HIGH_PRIORITY]
    [STRAIGHT_JOIN]
    select_result_size
    [SQL_BUFFER_RESULT]

select_distinct_mod = [ALL | DISTINCT | DISTINCTROW]

select_result_size = [SQL_BIG_RESULT | SQL_SMALL_RESULT]

select_expressions =
    / *
    / select_expression[, ...[, select_expression]]

select_expression = expression [AS identifier]

select_where_clause = WHERE expression

select_having_clause = HAVING expression

select_group_by_clause = GROUP BY
    select_order_item[, ...[, select_order_item]]
    [WITH ROLLUP]

select_order_by_clause = ORDER BY
    select_order_item[, ...[, select_order_item]]

select_order_item = expression [ASC | DESC]

select_limit_clause =
    / LIMIT integer[, integer]
    / LIMIT integer OFFSET integer
```

The `WHERE` clause, if given, indicates the condition or conditions that
rows must satisfy to be selected. where_condition is an expression that
evaluates to true for each row to be selected. The statement selects all
rows if there is no `WHERE` clause.
In the `WHERE` expression, you can use any of the functions and operators
that N supports, except for aggregate (summary) functions.

In general, clauses used must be given in exactly the order shown in the
syntax description. For example, a `HAVING` clause must come after any
`GROUP BY` clause and before any ORDER BY clause.

[comment]: <> (TODO Keywords explaining)

Explaining of keywords like HIGH_PRIORITY or DISTINCTROW will be added later.
Rule select_source see in next part. 
