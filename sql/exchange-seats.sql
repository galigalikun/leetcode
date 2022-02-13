-- Write your MySQL query statement below

select Seat.*,@num := @num + 1 as no from Seat , (select @num := 0) as t order by id desc
