-- Write your MySQL query statement below
select
    null as SecondHighestSalary
from
    (
        select
            Salary
        from
            Employee
        group by
            Salary
    ) t
having
    count(*) < 2
union
all (
    select
        Salary as SecondHighestSalary
    from
        Employee
    where
        exists (
            select
                1
            from
                (
                    select
                        Salary
                    from
                        Employee
                    group by
                        Salary
                ) t
            having
                count(*) > 1
        )
    group by
        Salary
    order by
        Salary desc
    limit
        1, 1
)
