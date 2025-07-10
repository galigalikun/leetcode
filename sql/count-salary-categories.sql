-- # Write your MySQL query statement below
with aaa as(
select
case when income < 20000 then 'Low Salary'
     when income >= 20000 and income <= 50000 then 'Average Salary'
     when income > 50000 then 'High Salary' end as salary_category,
account_id
 from Accounts
), bbb as(
    select 'Low Salary' as salary_category, 0 as dummy
    union all
    select 'Average Salary' as salary_category, 0 as dummy
    union all
    select 'High Salary' as salary_category, 0 as dummy
)
select
    salary_category as category,
    count(distinct aaa.account_id) as accounts_count
from aaa
right join bbb using(salary_category)
group by salary_category
order by salary_category
