-- # Write your MySQL query statement below
WITH filtered AS (
	SELECT
		id,
		visit_date,
		people,
		id - ROW_NUMBER() OVER (ORDER BY id) AS grp
	FROM Stadium
	WHERE people >= 100
),
valid_groups AS (
	SELECT grp
	FROM filtered
	GROUP BY grp
	HAVING COUNT(*) >= 3
)
SELECT
	id,
	visit_date,
	people
FROM filtered
WHERE grp IN (SELECT grp FROM valid_groups)
ORDER BY visit_date ASC;
