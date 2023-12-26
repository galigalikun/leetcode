-- # Write your MySQL query statement below
WITH DateRange AS (
  SELECT
    MIN(visited_on) AS start_date,
    MAX(visited_on) AS end_date
  FROM
    Customer
)

SELECT
  c.visited_on,
  SUM(c.amount) AS amount,
  ROUND(AVG(c.amount) OVER (ORDER BY c.visited_on ROWS BETWEEN 6 PRECEDING AND CURRENT ROW), 2) AS average_amount
FROM
  DateRange d
  JOIN Customer c ON c.visited_on BETWEEN DATE_SUB(d.end_date, INTERVAL 6 DAY) AND d.end_date
GROUP BY
  c.visited_on
ORDER BY
  c.visited_on;
