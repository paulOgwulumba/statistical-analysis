pub mod measures_of_central_tendency;
//mod measures_of_dispersion; // range, interquartile range, standard deviation, variance

#[cfg(test)]
mod tests {
    use crate::measures_of_central_tendency;

    #[test]
    fn test_get_mean() {
        let data_set_1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let data_set_2 = vec![-28948, 2345, 3, 4, 5, 62389, 793, -82, 9, 10];
        println!("Testing the get_mean function");
        assert_eq!(
            Some(5.5),
            measures_of_central_tendency::get_mean(data_set_1)
        );
        assert_eq!(None, measures_of_central_tendency::get_mean(Vec::new()));
        assert_eq!(
            Some(3652.8),
            measures_of_central_tendency::get_mean(data_set_2)
        );
    }

    #[test]
    fn test_get_mode() {
        let data_set_1 = vec![1, 2, 2, 2, 2, 5, 6, 7, 8, 10];
        let data_set_2 = vec![17, 62, 45, 45, 23, 23, 89, 90];

        assert_eq!(vec![2], measures_of_central_tendency::get_mode(data_set_1));
        assert_eq!(
            vec![23, 45],
            measures_of_central_tendency::get_mode(data_set_2)
        );
        assert_eq!(
            Vec::<i32>::new(),
            measures_of_central_tendency::get_mode(Vec::new())
        );
    }

    #[test]
    fn test_get_median_of_even_data_set() {
        let data_set_2 = vec![17, 62, 45, 45, 23, 23, 89, 90];

        assert_eq!(
            Some(45.0),
            measures_of_central_tendency::get_median(data_set_2)
        );
    }

    #[test]
    fn test_get_median_of_odd_data_set() {
        let data_set_1 = vec![1, 2, 2, 2, 2, 5, 6, 7, 8, 10, 13];

        assert_eq!(
            Some(5.0),
            measures_of_central_tendency::get_median(data_set_1)
        );
    }
}
