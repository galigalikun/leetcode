-- # Write your MySQL query statement below

select t.requester_id as id, count(*) as num from(
select requester_id from RequestAccepted
union all
select accepter_id from RequestAccepted
) as t
group by t.requester_id
order by num desc
limit 1
