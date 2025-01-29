-- # Write your MySQL query statement below
select t.employee_id, e.name, t.reports_count, t.average_age
from (
select reports_to as employee_id, count(*) as reports_count, round(avg(age)) as average_age
from Employees
where reports_to is not null
group by reports_to
) as t
inner join Employees e
on t.employee_id = e.employee_id
order by t.employee_id
