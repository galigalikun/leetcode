-- Write your MySQL query statement below
select
    w.Department as Department,
    m.Name as Employee,
    m.Salary as Salary
from
    Employee m
    inner join (
        select
            t.DepartmentId as DepartmentId,
            t.Department as Department,
            substring_index(
                substring_index(t.Salary, ',', n.digit + 1),
                ',',
                -1
            ) as Salary
        from
            (
                select
                    d.Id as DepartmentId,
                    d.Name as Department,
                    substring_index(
                        group_concat(
                            distinct e.Salary
                            order by
                                e.Salary desc
                        ),
                        ',',
                        3
                    ) as Salary
                from
                    Employee e
                    inner join Department d on e.DepartmentId = d.Id
                group by
                    d.Id,
                    d.Name
            ) t
            inner join (
                SELECT
                    0 as digit
                UNION
                ALL
                SELECT
                    1
                UNION
                ALL
                SELECT
                    2
                UNION
                ALL
                SELECT
                    3
            ) n on length(replace(t.Salary, ',', '')) <= length(t.Salary) - n.digit
    ) w on m.Salary = w.Salary
    and m.DepartmentId = w.DepartmentId
order by
    m.DepartmentId,
    m.Salary desc
