-- # Write your MySQL query statement below

WITH first_orders AS (
	SELECT
		d.customer_id,
		d.order_date,
		d.customer_pref_delivery_date
	FROM Delivery d
	JOIN (
		SELECT customer_id, MIN(order_date) AS first_order_date
		FROM Delivery
		GROUP BY customer_id
	) f
		ON d.customer_id = f.customer_id
		AND d.order_date = f.first_order_date
)
SELECT ROUND(
	100.0 * SUM(order_date = customer_pref_delivery_date) / COUNT(*),
	2
) AS immediate_percentage
FROM first_orders;
