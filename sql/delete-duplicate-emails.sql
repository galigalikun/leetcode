-- Write your MySQL query statement below
select
    min(Id) as Id,
    Email
from
    Person
group by
    Email
