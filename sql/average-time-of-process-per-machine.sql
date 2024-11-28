-- # Write your MySQL query statement below
select
    machine_id,
    round(2*sum(case when activity_type = 'end' then timestamp else 0 end - case when activity_type = 'start' then timestamp else 0 end)/count(*), 3) as processing_time
from Activity
group by machine_id
order by machine_id
