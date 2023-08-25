-- # Write your MySQL query statement below
select product_id,min(case when change_date >= '2019-08-16' then 10 else new_price end) as price from Products
group by product_id
