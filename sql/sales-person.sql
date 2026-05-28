-- # Write your MySQL query statement below
select s.name
from SalesPerson s
where not exists (
		select 1
		from Orders o
		join Company c on c.com_id = o.com_id
		where o.sales_id = s.sales_id
			and c.name = 'RED'
)
