-- Write your MySQL query statement below

select a.no as id , a.student as student from
(
select @num := @num + 1 as no,case when mod(id, 2) = 0 then id/2 else 1+id/2 end as s,student
from Seat,
(select @num := 0) as t
order by s
) a
