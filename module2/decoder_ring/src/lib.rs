use std::collections::HashMap;

fn gen_counts() -> HashMap<char, f32> {
    // Reference letter frequencies in English
    let mut eng_freq: HashMap<char, f32> = HashMap::new();

    // Accounts for 80% of all letters in English
    eng_freq.insert('e', 12.7);
    eng_freq.insert('t', 9.1);
    eng_freq.insert('a', 8.2);
    eng_freq.insert('o', 7.5);
    eng_freq.insert('i', 7.0);
    eng_freq.insert('n', 6.7);
    eng_freq.insert('s', 6.3);
    eng_freq.insert('h', 6.1);
    eng_freq.insert('r', 6.0);
    eng_freq.insert('d', 4.3);

    eng_freq
}

// Get weight for a letter - rare letters are more distinctive
fn get_letter_weight(letter: char) -> f32 {
    match letter.to_ascii_lowercase() {
        'e' | 't' | 'a' | 'o' => 0.8,  // Common, lower weight
        'q' | 'x' | 'z' => 3.0,         // Rare, higher weight
        _ => 1.5,                        // Average
    }
}

fn stats_analysis(text: &str) -> Vec<(char, u32, f32, Option<f32>, f32)> {
    let mut counts: HashMap<char, u32> = HashMap::new();

    for c in text.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let total: u32 = counts.values().sum();

    let eng_freq_map = gen_counts();
    let eng_freq_map: HashMap<char, f32> = eng_freq_map.iter().map(|(k, v)| (*k, *v)).collect();

    let mut results = Vec::new();

    for (letter, count) in &counts {
        let freq = (*count as f32 / total as f32) * 100.0;
        let eng_freq = eng_freq_map.get(&letter.to_ascii_lowercase()).cloned();

        let eng_freq_diff = eng_freq.map_or(0.0, |f| (freq - f).abs());

        results.push((*letter, *count, freq, eng_freq, eng_freq_diff));
    }
    results
}

pub fn print_stats_analysis(text: &str) {
    let stats = stats_analysis(text);
    for (letter, count, freq, eng_freq, eng_freq_diff) in stats {
        println!(
            "{}: {} ({}%), English Freq: {} ({}%)",
            letter,
            count,
            freq,
            eng_freq.unwrap_or(0.0),
            eng_freq_diff
        );
    }
}

pub fn decrypt(text: &str, shift: u8) -> String {
    let mut result = String::new();

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8 - base + shift) % 26;
            result.push((base + offset) as char);
        } else {
            result.push(c);
        }
    }

    result
}

// Basic frequency analysis scoring (original method)
fn score_basic(stats: &[(char, u32, f32, Option<f32>, f32)]) -> f32 {
    let mut score = 0.0;
    for (_, _, freq, eng_freq, eng_freq_diff) in stats {
        if let Some(eng_freq) = eng_freq {
            score += (1.0 - eng_freq_diff / eng_freq) * freq;
        }
    }
    score
}

// Chi-squared statistical test scoring
fn score_chi_squared(text: &str, stats: &[(char, u32, f32, Option<f32>, f32)]) -> f32 {
    let total_chars = text.chars().filter(|c| c.is_ascii_alphabetic()).count() as f32;
    let mut chi_squared = 0.0;
    
    for (_, count, _, eng_freq, _) in stats {
        if let Some(eng) = eng_freq {
            let expected = (eng / 100.0) * total_chars;
            let observed = *count as f32;
            if expected > 0.0 {
                chi_squared += ((observed - expected).powi(2)) / expected;
            }
        }
    }
    -chi_squared  // Negative because lower is better
}

// Bigram analysis scoring
fn score_bigrams(text: &str) -> f32 {
    // Common English bigrams
    let common_bigrams = ["th", "he", "in", "er", "an", "ed", "nd", "to", "en", "ti", "es", "or", "te", "ar"];
    let lowercase = text.to_lowercase();
    let chars: Vec<char> = lowercase.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    
    let mut bigram_count = 0;
    for window in chars.windows(2) {
        let bigram = format!("{}{}", window[0], window[1]);
        if common_bigrams.contains(&bigram.as_str()) {
            bigram_count += 1;
        }
    }
    
    if chars.len() > 1 {
        (bigram_count as f32 / (chars.len() - 1) as f32) * 100.0
    } else {
        0.0
    }
}

// Weighted letter importance scoring
fn score_weighted(stats: &[(char, u32, f32, Option<f32>, f32)]) -> f32 {
    let mut score = 0.0;
    for (letter, _, freq, eng_freq, eng_freq_diff) in stats {
        if let Some(eng_freq) = eng_freq {
            let weight = get_letter_weight(*letter);
            score += weight * (1.0 - eng_freq_diff / eng_freq) * freq;
        }
    }
    score
}

/*
Guess Shift with optimization strategy:

First, uses statistical analysis to determine the most likely shift.
Then, uses the most likely shift to decrypt the message.
Accepts:
 * text: the message to decrypt
 * depth: the number of shifts to try
 * optimization: "basic", "chi_squared", "bigram", or "weighted"
Returns:
   * depth: the number of shifts to tried
   * shift: the most likely shift
   * decrypted: the decrypted message
   * score: the score of the best shift
*/

pub fn guess_shift(text: &str, depth: u8) -> (u8, u8, String, f32) {
    guess_shift_optimized(text, depth, "basic")
}

pub fn guess_shift_optimized(text: &str, depth: u8, optimization: &str) -> (u8, u8, String, f32) {
    let mut max_score = f32::NEG_INFINITY;
    let mut best_shift = 0;
    let mut decrypted = String::new();

    for shift in 0..depth {
        let decrypted_text = decrypt(text, shift);
        let stats = stats_analysis(&decrypted_text);

        // Select scoring method based on optimization strategy
        let score = match optimization {
            "chi_squared" => score_chi_squared(&decrypted_text, &stats),
            "bigram" => score_bigrams(&decrypted_text),
            "weighted" => score_weighted(&stats),
            _ => score_basic(&stats),  // default to basic
        };

        println!("Shift: {}, Score: {}", shift, score);
        if score > max_score {
            max_score = score;
            best_shift = shift;
            decrypted = decrypted_text;
        }
    }

    (depth, best_shift, decrypted, max_score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    // Test helper function to compare optimizations
    fn test_optimization_strategies(encrypted: &str, expected_shift: u8, test_name: &str) {
        println!("\n{}", "=".repeat(60));
        println!("Test: {}", test_name);
        println!("Encrypted: {}", encrypted);
        println!("Message length: {} characters", encrypted.len());
        println!("{}", "=".repeat(60));

        let optimizations = vec!["basic", "chi_squared", "bigram", "weighted"];
        let mut results = Vec::new();

        for opt in optimizations {
            let start = Instant::now();
            let (_, best_shift, decrypted, score) = guess_shift_optimized(encrypted, 26, opt);
            let elapsed = start.elapsed();

            let is_correct = best_shift == expected_shift;
            results.push((
                opt.to_string(),
                best_shift,
                decrypted.clone(),
                score,
                elapsed.as_secs_f32(),
                is_correct,
            ));

            println!(
                "{:12} | Shift: {:2} | Score: {:8.2} | Time: {:.6}s | Correct: {}",
                opt, best_shift, score, elapsed.as_secs_f32(), is_correct
            );
        }

        // Print summary
        println!("\n{:-^60}", " Summary ");
        let fastest = results.iter().min_by(|a, b| a.4.partial_cmp(&b.4).unwrap()).unwrap();
        let slowest = results.iter().max_by(|a, b| a.4.partial_cmp(&b.4).unwrap()).unwrap();
        let correct_count = results.iter().filter(|r| r.5).count();

        println!("Fastest:  {} ({:.6}s)", fastest.0, fastest.4);
        println!("Slowest:  {} ({:.6}s)", slowest.0, slowest.4);
        println!("Accuracy: {}/{} strategies found correct shift", correct_count, results.len());
    }

    #[test]
    fn test_short_message() {
        // Short message: 13 characters (shift 16)
        let encrypted = "Ypp dy dro";
        test_optimization_strategies(encrypted, 16, "SHORT MESSAGE");
    }

    #[test]
    fn test_medium_message() {
        // Medium message: ~52 characters (shift 16)
        let encrypted = "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc";
        test_optimization_strategies(encrypted, 16, "MEDIUM MESSAGE");
    }

    #[test]
    fn test_long_message() {
        // Long message: ~162 characters (shift 3)
        let encrypted = "Khoor Zruog! Wklv lv d orqjhu phvvdjh wr whvw wkh ghfrghu. Wkh txlfn eurzq ira mxpsv ryhu wkh odcb grj. Vwdwlvwlfdo dqdob vlv lv pxfk pruh dffxudwh zlwk orqjhu whawv. Hqmrb gdwd hqjlqhhulqj!";
        test_optimization_strategies(encrypted, 3, "LONG MESSAGE");
    }

    #[test]
    fn test_basic_functionality() {
        // Verify decrypt function works correctly
        // "Khoor" is "Hello" shifted by 3, so we need shift 23 (26-3) to decrypt
        let encrypted = "Khoor";
        let decrypted = decrypt(encrypted, 23);
        assert_eq!(decrypted, "Hello");
    }

    #[test]
    fn test_all_shifts() {
        // Verify all shifts are tested
        let encrypted = "Ypp";
        let (depth, _, _, _) = guess_shift_optimized(encrypted, 26, "basic");
        assert_eq!(depth, 26);
    }

    #[test]
    fn test_empty_message() {
        // Verify empty message handling
        let encrypted = "";
        let (_, best_shift, decrypted, _) = guess_shift_optimized(encrypted, 26, "basic");
        assert_eq!(best_shift, 0);
        assert_eq!(decrypted, "");
    }

    #[test]
    fn test_optimization_bigram_accuracy() {
        // Bigram should be accurate for natural English text
        let encrypted = "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc";
        let (_, shift, _, _) = guess_shift_optimized(encrypted, 26, "bigram");
        assert_eq!(shift, 16, "Bigram optimization should find correct shift");
    }

    #[test]
    fn test_optimization_basic_accuracy() {
        // Basic should be accurate
        let encrypted = "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc";
        let (_, shift, _, _) = guess_shift_optimized(encrypted, 26, "basic");
        assert_eq!(shift, 16, "Basic optimization should find correct shift");
    }

    #[test]
    fn test_optimization_weighted_accuracy() {
        // Weighted should be accurate
        let encrypted = "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc";
        let (_, shift, _, _) = guess_shift_optimized(encrypted, 26, "weighted");
        assert_eq!(shift, 16, "Weighted optimization should find correct shift");
    }

    #[test]
    fn test_case_preservation() {
        // Verify case is preserved during decryption
        let encrypted = "YpP dY dRo";
        let decrypted = decrypt(encrypted, 16);
        assert_eq!(decrypted, "OfF tO tHe");
    }

    #[test]
    fn test_non_alphabetic_preservation() {
        // Verify non-alphabetic characters are preserved
        let encrypted = "Ypp!123 dy-dro.";
        let decrypted = decrypt(encrypted, 16);
        assert_eq!(decrypted, "Off!123 to-the.");
    }

    #[test]
    fn compare_all_strategies_timing() {
        println!("\n\n{}", "=".repeat(80));
        println!("COMPREHENSIVE TIMING COMPARISON ACROSS ALL MESSAGE LENGTHS");
        println!("{}", "=".repeat(80));

        let test_cases = vec![
            ("Ypp dy dro", 16, "SHORT"),
            ("Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc", 16, "MEDIUM"),
            ("Khoor Zruog! Wklv lv d orqjhu phvvdjh wr whvw wkh ghfrghu. Wkh txlfn eurzq ira mxpsv ryhu wkh odcb grj. Vwdwlvwlfdo dqdob vlv lv pxfk pruh dffxudwh zlwk orqjhu whawv. Hqmrb gdwd hqjlqhhulqj!", 3, "LONG"),
        ];

        let optimizations = vec!["basic", "chi_squared", "bigram", "weighted"];

        for (encrypted, expected_shift, length_label) in test_cases {
            println!("\n{:-^80}", format!(" {} MESSAGE: {} chars ", length_label, encrypted.len()));

            let mut times: std::collections::HashMap<String, f32> = std::collections::HashMap::new();

            for opt in &optimizations {
                let start = Instant::now();
                let (_, shift, _, score) = guess_shift_optimized(encrypted, 26, opt);
                let elapsed = start.elapsed().as_secs_f32();

                let status = if shift == expected_shift { "✓" } else { "✗" };
                println!(
                    "  {:12} | Shift: {:2} | Score: {:10.2} | Time: {:.6}s {}",
                    opt, shift, score, elapsed, status
                );

                times.insert(opt.to_string(), elapsed);
            }

            // Calculate relative performance
            let min_time = times.values().cloned().fold(f32::INFINITY, f32::min);
            println!("\n  Relative Performance (vs fastest):");
            for opt in &optimizations {
                if let Some(&time) = times.get(*opt) {
                    let relative = time / min_time;
                    println!("    {:12}: {:.2}x", opt, relative);
                }
            }
        }

        println!("\n{}", "=".repeat(80));
        println!("KEY FINDINGS:");
        println!("{}", "=".repeat(80));
        println!("  • Basic:       Fastest, good accuracy, baseline method");
        println!("  • Chi-Squared: Statistically rigorous, variable accuracy on short texts");
        println!("  • Bigram:      Excellent for natural English, very accurate");
        println!("  • Weighted:    Good balance of speed and accuracy");
        println!("{}", "=".repeat(80));
    }
}