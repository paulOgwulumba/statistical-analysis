pub mod measures_of_central_tendency;
pub mod measures_of_dispersion; // range, interquartile range, standard deviation, variance

#[cfg(test)]
mod tests_for_measures_of_central_tendency {
    use crate::measures_of_central_tendency;

    #[test]
    fn get_mean_of_empty_data_set() {
        assert_eq!(None, measures_of_central_tendency::get_mean(Vec::new()));
    }

    #[test]
    fn get_mean_of_single_element_data_set() {
        assert_eq!(Some(-9.0), measures_of_central_tendency::get_mean(vec![-9]));
    }

    #[test]
    fn get_mean_of_random_data_set_1() {
        let data_set_1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(
            Some(5.5),
            measures_of_central_tendency::get_mean(data_set_1)
        );
    }

    #[test]
    fn get_mean_of_random_data_set2() {
        let data_set_2 = vec![-28948, 2345, 3, 4, 5, 62389, 793, -82, 9, 10];
        assert_eq!(
            Some(3652.8),
            measures_of_central_tendency::get_mean(data_set_2)
        );
    }

    #[test]
    fn get_mode_for_empty_data_set() {
        assert_eq!(
            Vec::<i32>::new(),
            measures_of_central_tendency::get_mode(Vec::new())
        );
    }

    #[test]
    fn get_mode_for_single_element_data_set() {
        assert_eq!(vec![9], measures_of_central_tendency::get_mode(vec![9]));
    }

    #[test]
    fn get_mode_for_uniform_data_set() {
        assert_eq!(
            vec![-2],
            measures_of_central_tendency::get_mode(vec![-2, -2, -2, -2, -2])
        );
    }

    #[test]
    fn get_mode_for_random_data_set_1() {
        let data_set_1 = vec![1, 2, 2, 2, 2, 5, 6, 7, 8, 10];
        assert_eq!(vec![2], measures_of_central_tendency::get_mode(data_set_1));
    }

    #[test]
    fn get_mode_for_random_data_set_2() {
        let data_set_2 = vec![17, 62, 45, 45, 23, 23, 89, 90];
        let data = measures_of_central_tendency::get_mode(data_set_2);
        assert_eq!(true, data.contains(&23));
        assert_eq!(true, data.contains(&45));
    }

    #[test]
    fn get_median_of_even_data_set() {
        let data_set_2 = vec![17, 62, 45, 45, 23, 23, 89, 90];

        assert_eq!(
            Some(45.0),
            measures_of_central_tendency::get_median(data_set_2)
        );
    }

    #[test]
    fn get_median_of_odd_data_set() {
        let data_set_1 = vec![1, 2, 2, 2, 2, 5, 6, 7, 8, 10, 13];

        assert_eq!(
            Some(5.0),
            measures_of_central_tendency::get_median(data_set_1)
        );
    }

    #[test]
    fn get_median_of_empty_data_set() {
        assert_eq!(
            None,
            measures_of_central_tendency::get_median(Vec::<i32>::new())
        );
    }

    #[test]
    fn get_median_of_single_element_data_set() {
        assert_eq!(Some(5.0), measures_of_central_tendency::get_median(vec![5]));
    }

    #[test]
    fn get_median_of_double_element_data_set() {
        assert_eq!(
            Some(-8.0),
            measures_of_central_tendency::get_median(vec![-8, -8])
        );
    }
}

#[cfg(test)]
mod tests_for_measures_of_dispersion {
    use crate::measures_of_dispersion;

    #[test]
    fn get_range_of_empty_data_set() {
        let data_set = Vec::<i32>::new();
        assert_eq!(None, measures_of_dispersion::get_range(data_set));
    }

    #[test]
    fn get_range_of_single_element_data_set() {
        let data_set = vec![-8];
        assert_eq!(Some(0), measures_of_dispersion::get_range(data_set));
    }

    #[test]
    fn get_range_of_sorted_data_set() {
        let data_set = vec![1, 2, 2, 2, 2, 5, 6, 7, 8, 10, 13];
        assert_eq!(Some(12), measures_of_dispersion::get_range(data_set));
    }

    #[test]
    fn get_range_of_unsorted_data_set() {
        let data_set = vec![17, 62, 45, 45, 23, 23, 89, 90];
        assert_eq!(Some(73), measures_of_dispersion::get_range(data_set));
    }

    #[test]
    fn get_interquartile_range_for_empty_data_set() {
        assert_eq!(
            None,
            measures_of_dispersion::get_interquartile_range(Vec::<i32>::new())
        );
    }

    #[test]
    fn get_interquartile_range_for_single_element_data_set() {
        assert_eq!(
            Some(0.0),
            measures_of_dispersion::get_interquartile_range(vec![9])
        );
    }

    #[test]
    fn get_interquartile_range_for_double_element_data_set() {
        assert_eq!(
            Some(6.0),
            measures_of_dispersion::get_interquartile_range(vec![1, -5])
        );
    }

    #[test]
    fn get_interquartile_range_for_odd_data_set() {
        let data_set: Vec<i32> = vec![1, 2, 5, 6, 7, 9, 12, 15, 18, 19, 27];
        assert_eq!(
            Some(13.0),
            measures_of_dispersion::get_interquartile_range(data_set)
        );
    }

    #[test]
    fn get_interquartile_range_for_odd_unordered_data_set() {
        let data_set: Vec<i32> = vec![1, 2, 12, 5, 7, 9, 18, 19, 27, 15, 6];
        assert_eq!(
            Some(13.0),
            measures_of_dispersion::get_interquartile_range(data_set)
        );
    }

    #[test]
    fn get_interquartile_range_for_even_data_set() {
        let data_set: Vec<i32> = vec![3, 5, 7, 8, 9, 11, 15, 16, 20, 21];
        assert_eq!(
            Some(9.0),
            measures_of_dispersion::get_interquartile_range(data_set)
        );
    }

    #[test]
    fn get_interquartile_range_for_even_unordered_data_set() {
        let data_set: Vec<i32> = vec![5, 11, 7, 8, 9, 20, 15, 16, 21, 3];
        assert_eq!(
            Some(9.0),
            measures_of_dispersion::get_interquartile_range(data_set)
        );
    }

    #[test]
    fn get_variance_for_empty_data_set() {
        assert_eq!(
            None,
            measures_of_dispersion::get_variance(Vec::<i32>::new())
        );
    }

    #[test]
    fn get_variance_for_single_data_set() {
        assert_eq!(Some(0.0), measures_of_dispersion::get_variance(vec![9]));
    }

    #[test]
    fn get_variance_for_random_data_set() {
        let data_set = vec![
            9, 2, 5, 4, 12, 7, 8, 11, 9, 3, 7, 4, 12, 5, 4, 10, 9, 6, 9, 4,
        ];
        assert_eq!(
            Some(8.9).unwrap_or_default() as u32,
            measures_of_dispersion::get_variance(data_set).unwrap_or_default() as u32
        );
    }

    #[test]
    fn get_variance_for_uniform_data_set() {
        let data_set = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
        assert_eq!(Some(0.0), measures_of_dispersion::get_variance(data_set));
    }

    #[test]
    fn get_standard_deviation_for_empty_data_set() {
        assert_eq!(
            None,
            measures_of_dispersion::get_standard_deviation(Vec::<i32>::new())
        );
    }

    #[test]
    fn get_standard_deviation_for_single_data_set() {
        assert_eq!(
            Some(0.0),
            measures_of_dispersion::get_standard_deviation(vec![9])
        );
    }

    #[test]
    fn get_standard_deviation_for_random_data_set() {
        let data_set = vec![
            9, 2, 5, 4, 12, 7, 8, 11, 9, 3, 7, 4, 12, 5, 4, 10, 9, 6, 9, 4,
        ];
        assert_eq!(
            Some(2.983).unwrap_or_default() as u32,
            measures_of_dispersion::get_standard_deviation(data_set).unwrap_or_default() as u32
        );
    }

    #[test]
    fn get_standard_deviation_for_uniform_data_set() {
        let data_set = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
        assert_eq!(
            Some(0.0),
            measures_of_dispersion::get_standard_deviation(data_set)
        );
    }
}
