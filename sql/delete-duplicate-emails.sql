-- Write your MySQL query statement below
delete p1
from Person p1
inner join(
select
    min(Id) as Id,
    Email
from
    Person
group by
    Email
having count(*) > 1
) p2
on p1.Id <> p2.Id and p1.Email = p2.Email

