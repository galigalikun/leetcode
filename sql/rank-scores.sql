-- Write your MySQL query statement below
select
    Score,
    cast(`Rank` as SIGNED) `Rank`
from
    (
        select
            case
                when @before_score = score then @rank
                else @rank := ifnull(@rank, 0) + 1
            end as `Rank`,
            @before_score := Score as Score
        from
            Scores
        order by
            Score desc
    ) t
