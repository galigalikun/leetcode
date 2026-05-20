-- # Write your MySQL query statement below
with tmp as (
select e.name, b.bonus, row_number() over (partition by e.empId) as rn
from Employee e
left join Bonus b on e.empId = b.empId
where b.bonus is null or b.bonus < 1000
)
select name, bonus
from tmp
where rn = 1
