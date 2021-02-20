-- Write your MySQL query statement below
select
    l.id
from
    Weather n
    inner join Weather l on n.recordDate = date_sub(l.recordDate, INTERVAL 1 DAY)
where
    n.temperature < l.temperature
