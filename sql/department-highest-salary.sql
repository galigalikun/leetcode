-- Write your MySQL query statement below
select
    d.Name as Department,
    e.Name as Employee,
    e.Salary as Salary
from
    Employee e
    inner join Department d on e.DepartmentId = d.Id
where
    exists (
        select
            *
        from
            (
                select
                    DepartmentId,
                    max(Salary) as Salary
                from
                    Employee
                group by
                    DepartmentId
            ) t
        where
            t.DepartmentId = e.DepartmentId
            and t.Salary = e.Salary
    )
