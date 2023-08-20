-- # Write your MySQL query statement below
select Users.user_id as buyer_id, 
Users.join_date, 
count(distinct case when Orders.order_date between '2019-01-01' and '2019-12-31' then Orders.order_id else null end) as orders_in_2019 
from Users
left join Orders on Orders.buyer_id = Users.user_id
group by Users.user_id, Users.join_date
order by Users.user_id asc
