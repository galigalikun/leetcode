CREATE FUNCTION getNthHighestSalary(N INT) RETURNS INT
BEGIN
    DECLARE postion INT DEFAULT 0;
    SELECT N - 1 INTO postion;
    RETURN (
        -- Write your MySQL query statement below.
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
    count(*) < N
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
                count(*) > postion
        )
    group by
        Salary
    order by
        Salary desc
    limit
        postion, 1
)
    );
END
