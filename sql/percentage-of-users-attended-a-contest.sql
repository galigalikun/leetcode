-- # Write your MySQL query statement below
select a.contest_id, round(100.0*count(*)/max(b.cnt), 2) as percentage
from Register a
cross join (
    select count(*) as cnt from Users
) b
group by a.contest_id
order by count(*) desc, a.contest_id
