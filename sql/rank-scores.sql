-- Write your MySQL query statement below
select
    t.s as `score`,
    cast(t.r as UNSIGNED) as `Rank`
from
    (
        SELECT
            case
                when @before_score = t1.Score then 1
                else @rank := @rank + 1
            end as `num`,
            @before_score := t1.Score,
            @rank as r,
            t1.Score as s
        FROM
            (
                SELECT
                    *
                FROM
                    Scores
                order by
                    Score desc
            ) t1
            inner join (
                SELECT
                    @before_score := null
            ) t2
            inner join (
                SELECT
                    @rank := 0
            ) t3
    ) t
