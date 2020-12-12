#![feature(test)]
#![feature(proc_macro_hygiene)]

pub use leetcode_prelude::{
    assert_eq_sorted, btree, leetcode_test, linkedlist, vec_string, ListNode, TreeNode,
};
pub mod utils;

pub mod s001;
pub mod s002;
pub mod s003;
pub mod s004;
pub mod s005;

pub mod s018_4sum;
pub mod s019_remove_nth_node_from_end_of_list;
pub mod s020;
pub mod s021;
pub mod s022_generate_parentheses;

pub mod s026;
pub mod s027;
pub mod s028;

pub mod s035;
pub mod s036_valid_sudoku;
pub mod s037_sudoku_solver;

pub mod s038;

pub mod s053;

pub mod s058;

pub mod s066;
pub mod s067;

pub mod s069;
pub mod s070;

pub mod s083;

pub mod s088;
pub mod s100;
pub mod s101;

pub mod s104;

pub mod s107;
pub mod s108;
pub mod s110;

pub mod s111;
pub mod s112;

pub mod s118;
pub mod s119;

pub mod s121;
pub mod s122;

pub mod s125;

pub mod s136;

pub mod s155;

pub mod s167;
pub mod s168;
pub mod s169;

pub mod s171;
pub mod s172;

pub mod s179_largest_number;

pub mod s189;

pub mod s198;

pub mod s202;
pub mod s203_remove_liked_list_elemnets;
pub mod s204_count_primes;
pub mod s205_isomorphic_strings;
pub mod s206_reverse_linked_list;

pub mod s217_contains_duplicate;

pub mod s219_contains_duplicate_ii;

pub mod s225_implement_stack_using_queues;
pub mod s226_invert_binary_tree;

pub mod s231_power_of_two;
pub mod s232_implement_queue_using_stacks;

pub mod s234_palindrome_linked_list;

pub mod s242_valid_anagram;

pub mod s257_binary_tree;
pub mod s258_add_digits;

pub mod s263_ugly_number;

pub mod s268_missing_number;

pub mod s283_move_zeroes;

pub mod s290_word_pattern;

pub mod s292_nim_game;

pub mod s345_reverse_vowels_of_a_string;
pub mod s349_intersection_of_two_arrays;
pub mod s350_intersection_of_two_arrays_ii;

pub mod s352_summary_ranges;
pub mod s354_max_envelpoes;
pub mod s355_twitter_design;

pub mod s367_valid_perfect_square;

pub mod s383_ransom_note;

pub mod s387_first_unique_character_in_a_string;

pub mod s389_find_the_difference;

pub mod s400_nth_digit;
pub mod s401_binary_watch;

pub mod s404_sum_of_left_leaves;
pub mod s405_convert_a_number_to_hexadecimal;

pub mod s409_longest_palindrome;

pub mod s412_fizz_buzz;

pub mod s414_third_maximum_number;
pub mod s415_add_strings;

pub mod s434_number_of_segments_in_a_string;

pub mod s437_path_sum_iii;
pub mod s438_find_all_anagrams_in_a_string;

pub mod s441_arranging_coins;

pub mod s443_string_compression;

pub mod s447_number_of_boomerangs;
pub mod s448_find_all_numbers_disappeared_in_array;

pub mod s453_minimum_moves_to_equal_array_elements;

pub mod s455_assign_cookies;

pub mod s461_hamming_distance;

pub mod s463_island_perimeter;

pub mod s475_heaters;
pub mod s476_number_complement;

pub mod s482_license_key_formatting;

pub mod s485_max_consecutive_ones;

pub mod s492_construct_the_rectangle;
pub mod s493_reverse_pairs;

pub mod s496_next_greater_element;

pub mod s500_keyboard_row;

pub mod s504_base_7;

pub mod s506_relative_ranks;
pub mod s507_perfect_number;

pub mod s509_fibonacci_number;

pub mod s520_detect_capital;
pub mod s521_longest_uncommon_subsequence;

pub mod s530_minimum_difference;

pub mod s532_diff_pairs_in_array;

pub mod s538_convert_bst_greater_tree;

pub mod s541_reverse_string_ii;

pub mod s1281_subtract_product_and_sum;

pub mod s1287_find_special_integer;

pub mod s1290_get_decimal_value;

pub mod s1295_find_numbers;

pub mod s1299_replace_elements_by_right_max;

pub mod s1304_sum_of_n_nums_is_zero;

pub mod s1309_freq_alphabets;

pub mod s1313_decompress_rl_elist;

pub mod s1317_get_no_zero_integers;

pub mod s1323_maximum69_number;

pub mod s1331_array_rank_transform;
pub mod s1332_remove_palindrome_sub;

pub mod s1337_k_weakest_rows;

pub mod s1342_number_of_steps;

pub mod s1346_check_if_double_exist;

pub mod s1351_count_negatives;

pub mod s1356_sort_by_bits;

pub mod s1360_days_between_dates;

pub mod s1365_smaller_numbers_than_current;

pub mod s1370_increasing_decreasing_string;

pub mod s1374_generate_the_string;

pub mod s1380_lucky_numbers_in_matrix;

pub mod s1385_find_the_distance_value_of_two_arr;

pub mod s1389_create_target_array;

pub mod s1394_find_lucky_in_arr;

pub mod s1399_count_largest_group;

pub mod s1403_min_subsequence_in_arr;

pub mod s1408_string_matching_in_an_array;

pub mod s1413_min_start_value;

pub mod s1417_reformat_string;

pub mod s1422_max_score_split_string;

pub mod s1431_kids_with_candies;

pub mod s1436_dest_city;

pub mod s1441_build_array_from_stack;

pub mod s1446_max_power;

pub mod s1450_busy_student;

pub mod s1455_is_prefix_of_word;

pub mod s1460_arr_can_be_equal;

pub mod s1463_cherry_pick_ii;
pub mod s1464_max_product;

pub mod s1470_shuffle_arr;

pub mod s1475_final_prices;

pub mod s1480_running_sum;

pub mod s1486_xor_operation;

pub mod s1491_average_salary;

pub mod s1496_is_path_crossing;

pub mod s1502_can_make_arithmetic_progression;

pub mod s1507_reformat_date;

pub mod s1512_num_identical_pairs;

pub mod s1518_num_water_bottles;

pub mod s1523_count_odds;

pub mod s1528_restore_string;

pub mod s1534_count_good_triplets;

pub mod s1539_find_kth_positive;

pub mod s1544_make_good;

pub mod s1550_three_consecutive_odds;

pub mod s1556_thousand_separator;

pub mod s1560_most_visited;

pub mod s1566_contains_pattern;

pub mod s1572_diagonal_sum;

pub mod s1576_modify_string;

pub mod s1582_num_special;

pub mod s1588_sum_odd_length_subarrays;

pub mod s1592_reorder_spaces;

pub mod s1598_min_operations;

pub mod s1603_parking_system;

pub mod s1608_special_array;
pub mod s1614_max_depth;
