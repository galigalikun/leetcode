-- # Write your MySQL query statement below
select distinct viewer_id as id from Views
where article_id in (select distinct article_id from Views where viewer_id = 1)
