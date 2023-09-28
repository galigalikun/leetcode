-- # Write your MySQL query statement below
select query_name,
round(sum(rating/position)/count(*),2) as quality,
round((sum(case when rating > 2 then 0 else 1 end) / count(*)) * 100,2) as poor_query_percentage 
from Queries
group by query_name
