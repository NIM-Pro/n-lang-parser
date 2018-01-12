INSERT INTO foo (a, bar, baz)
SELECT a, bar, bar * 2 AS baz
FROM boo
