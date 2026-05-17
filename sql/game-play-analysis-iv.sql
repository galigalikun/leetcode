-- # Write your MySQL query statement below

WITH first_login AS (
	SELECT player_id, MIN(event_date) AS first_date
	FROM Activity
	GROUP BY player_id
),
next_day_login AS (
	SELECT DISTINCT a.player_id
	FROM Activity a
	JOIN first_login f
		ON a.player_id = f.player_id
	   AND a.event_date = DATE_ADD(f.first_date, INTERVAL 1 DAY)
)
SELECT ROUND(
	(SELECT COUNT(*) FROM next_day_login) / (SELECT COUNT(*) FROM first_login),
	2
) AS fraction;
