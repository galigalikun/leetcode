-- # Write your MySQL query statement below
select
    num
from (
select
    num
from MyNumbers
group by num
having count(*) = 1
union all
select null
) as t
order by num desc
limit 1
