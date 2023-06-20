-- # Write your MySQL query statement below
select Product.product_id, Sales.year as first_year, Sales.quantity, Sales.price from Product
inner join Sales
on Product.product_id = Sales.product_id
where (Product.product_name, Sales.year) in (
    select Product.product_name, min(Sales.year) from Sales
    inner join Product
    on Sales.product_id = Product.product_id
    group by Product.product_name
)
