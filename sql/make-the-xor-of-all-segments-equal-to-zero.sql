-- # Write your MySQL query statement below
select employee_id, department_id
from(
select employee_id,  department_id, primary_flag, row_number() over(partition by employee_id order by primary_flag) as rn
from Employee) as t
where rn = 1
