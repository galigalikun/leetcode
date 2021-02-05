-- Write your MySQL query statement below
select
    t.Num as ConsecutiveNums
from
    (
        select
            Num,
            case
                when @before_num = Num then @c := ifnull(@c, 1) + 1
                else @c := 0
            end as total,
            @before_num := Num
        from
            Logs
            inner join (
                select
                    @c := 1,
                    @before_num := null
            ) work
        order by
            Num
    ) t
where
    t.total >= 3
