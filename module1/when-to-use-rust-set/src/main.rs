use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "insertion_and_lookup_performance")] {
        mod insertion_and_lookup_performance;
        use insertion_and_lookup_performance::run;
        fn main() {
            run();
        }

    } else if #[cfg(feature = "word_frequencies")] {
        mod word_frequencies;
        use word_frequencies::run;
        fn main() {
            let _ = run();
        }

    } else if #[cfg(feature = "priority_queue")] {
        mod priority_queue;
        use priority_queue::run;
        fn main() {
            run();
        }

    } else if #[cfg(feature = "struct_with_traits")] {
        mod struct_with_traits;
        use struct_with_traits::run;
        fn main() {
            run();
        }

    } else if #[cfg(feature = "custom_collection")] {
        mod custom_collection;
        use custom_collection::run;
        
        #[cfg(test)]
        mod custom_collections_test;

        fn main() {
            run();
        }

    }  else {
        fn main() {
            println!("Please enable a feature to run the corresponding example.");
        }
    }
}
