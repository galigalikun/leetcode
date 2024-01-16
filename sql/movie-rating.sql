-- # Write your MySQL query statement below
select m.movie_name, u.user_name, mr.rating, mr.created_at
from MovieRating mr
inner join Movie m on m.movie_id = mr.movie_id
inner join User u on u.user_id = mr.user_id
where mr.created_at between '2020-02-01' and '2020-02-29'
group by mr.rating
having mr.rating >= 3
order by mr.created_at desc, m.movie_name asc;
