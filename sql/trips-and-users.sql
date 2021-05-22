-- Write your MySQL query statement below
select
    Trips.Request_at as Day,
    round(
        sum(
            case
                when Clients.Banned = 'No'
                and Trips.Status in('cancelled_by_driver', 'cancelled_by_client') then 1
                else 0
            end
        ) / sum(
            case
                when Clients.Banned = 'No' then 1
                else 0
            end
        ),
        2
    ) as "Cancellation Rate"
from
    Trips
    inner join Users Clients on Trips.Client_Id = Clients.Users_Id
    inner join Users Drivers on Trips.Driver_Id = Drivers.Users_Id
where
    Trips.Request_at >= '2013-10-01'
    and Trips.Request_at <= '2013-10-03'
    and not(
        Clients.Banned = 'Yes'
        and Trips.Status = 'completed'
    )
    and Drivers.Banned = 'No'
group by
    Trips.Request_at
