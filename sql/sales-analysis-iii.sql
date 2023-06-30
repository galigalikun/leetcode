-- # Write your MySQL query statement below
select Product.product_id, Product.product_name from Product 
inner join (select product_id, max(sale_date) as max_sale_date, min(sale_date) as min_sale_date from Sales group by product_id) as Sales
on Product.product_id = Sales.product_id
where Sales.max_sale_date between '2019-01-01' and '2019-03-31'
and Sales.min_sale_date between '2019-01-01' and '2019-03-31'
