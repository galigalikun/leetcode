-- # Write your MySQL query statement below
select person_name from (
select turn,person_id,person_name,weight,sum(weight) over (order by turn) as total_weight from Queue
order by turn) t
where t.total_weight <=1000
order by t.total_weight desc
limit 1
