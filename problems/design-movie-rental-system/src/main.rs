use std::collections::HashMap;
struct MovieRentingSystem {
    // Fields to store the state of the movie renting system
    // For example, you might want to store a list of movies, shops, and their rental status
    // You can use vectors, hash maps, or other data structures as needed
    // Example:
    movies: Vec<Movie>,
    shops: HashMap<i32, Vec<Movie>>,
    rentals: HashMap<i32, Vec<i32>>,
}

#[derive(Debug, Clone)]
struct Movie {
    id: i32,
    shop_id: i32,
    price: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {

    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        Self { movies: vec![], shops: HashMap::new(), rentals: HashMap::new() }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        // This method should return a list of shop IDs that have the specified movie available for rent
        // You can filter through the shops and their movies to find the matching ones
        let mut result = Vec::new();
        for (shop_id, movies) in &self.shops {
            if movies.iter().any(|m| m.id == movie) {
                result.push(*shop_id);
            }
        }
        result
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        // This method should handle the renting of a movie from a shop
        // You might want to check if the movie is available in the specified shop before renting
        if let Some(movies) = self.shops.get_mut(&shop) {
            if let Some(pos) = movies.iter().position(|m| m.id == movie) {
                // Remove the movie from the shop's inventory
                movies.remove(pos);
                // Add the rental to the rentals map
                self.rentals.entry(movie).or_default().push(shop);
            }
        }
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        // This method should handle dropping a rented movie back to the shop
        // You might want to check if the movie was rented from the specified shop before dropping
        if let Some(movies) = self.shops.get_mut(&shop) {
            // Add the movie back to the shop's inventory
            movies.push(Movie { id: movie, shop_id: shop, price: 0 }); // Assuming price is 0 when dropped
            // Remove the rental from the rentals map
            if let Some(rentals) = self.rentals.get_mut(&movie) {
                if let Some(pos) = rentals.iter().position(|&s| s == shop) {
                    rentals.remove(pos);
                }
            }
        }
    }

    fn report(&self) -> Vec<Vec<i32>> {
        // This method should return a report of all shops and their available movies
        let mut report = Vec::new();
        for (shop_id, movies) in &self.shops {
            let mut shop_report = vec![*shop_id];
            for movie in movies {
                shop_report.push(movie.id);
            }
            report.push(shop_report);
        }
        report.sort_by_key(|v| v[0]); // Sort by shop ID
        report
    }
}

/**
 * Your MovieRentingSystem object will be instantiated and called as such:
 * let obj = MovieRentingSystem::new(n, entries);
 * let ret_1: Vec<i32> = obj.search(movie);
 * obj.rent(shop, movie);
 * obj.drop(shop, movie);
 * let ret_4: Vec<Vec<i32>> = obj.report();
 */
fn main() {
    let mut obj = MovieRentingSystem::new(3, vec![vec![0, 1, 5], vec![0, 2, 6], vec![0, 3, 7], vec![1, 1, 4], vec![1, 2, 7], vec![2, 1, 5]]);
    let ret_1: Vec<i32> = obj.search(1);
    obj.rent(0, 1);
    obj.rent(1, 2);
    obj.report();
    obj.drop(1, 2);
    obj.search(2);
}
