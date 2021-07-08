
use std::fs;
use std::error::Error;
use std::env;

/// Config struct
/// 
/// Used to get the configuration of the grep execution
///
/// # Arguments
///
/// * `query` - The term being searched for
/// * `filename` - The search file
/// * `case_sensitive` - Environment variable for case sensitive/insensitive search
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

/// Config constructor
///
/// # Parameters
///
/// * `args` - String array of arguments
///
/// # Returns
/// 
/// * `Result<Config, &str>` - Config and simple error flag
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // Checks minimum arguments have been entered
        if args.len() < 3 {
            return Err("Some arguments appear to be missing");
        }
        
        // Get relevant arguments (first argument is the filepath to the executable)
        let query = args[1].clone();
        let filename = args[2].clone();

        // Get environment variable "CASE_INSENSITIVE"
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        // Create Config and return it with an Ok wrapper
        Ok(Config { query, filename, case_sensitive })
    }
}

/// Run method
///
/// Method that gets search content and decides what search type to run
///
/// # Parameters
///
/// `config` - The given config of the execution
///
/// # Returns
///
/// `Result<(), Box<dyn Error>>` - Simple error flag 
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Gets contents from the given file
    let contents = fs::read_to_string(config.filename)
        // Error handling
        .expect("Something went wrong reading the file");

    // Gets the results of the search
    let results = if config.case_sensitive {
        // If case sensitive use search()
        search(&config.query, &contents)
    } else {
        // If case insensitive use search_case_insensitive()
        search_case_insensitive(&config.query, &contents)
    };

    // Output results
    for line in results {
        println!("{}", line);
    }

    // Return Ok error flag
    Ok(())
}

/// Search method
///
/// Performs case sensitive search
///
/// # Arguments
///
/// * `query` - The search query -- see Config for more information
/// * `contents` - Contents of the file
///
/// # Returns
///
/// `Vec<&'a str>` - Vector of lines that contain the search query
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Builds results vector
    let mut results = Vec::new();

    // Iterates through contents lines
    for line in contents.lines() {
        // Check if the line contains the query
        if line.contains(query) {
            // If it does push it to results vector
            results.push(line);
        }
    }

    // Returns results
    results
}

/// Search method -- case insenitive
///
/// Performs a case insensitive search
///
/// # Arguments
///
/// * `query` - The search query -- see Config for more information
/// * `contents` - Contents of the file
///
/// # Returns
///
/// `Vec<&'a str>` - Vector of lines that contain the search query
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Changes query to lowercase
    let query = query.to_lowercase();

    // See search() for other comments
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

 
#[cfg(test)]
mod tests {
    use super::*;

    /// One result test
    ///
    /// Test if the correct result is returned when one is desired
    #[test]
    fn one_result() {
        let query = "Testing";
        let contents = "This is a string\nIt contains a line that says Testing which should be found by the program\nIt also contains another LINE that does not contain the above term that should not be found";
        
        assert_eq!(
            vec!["It contains a line that says Testing which should be found by the program"], 
            search(query, contents)
        );
    }

    /// Multiple result test
    ///
    /// Test if the correct result is returned when several are desired
    #[test]
    fn multiple_result() {
        let query = "contains";
        let contents = "This is a string\nIt contains a line that says Testing which should be found by the program\nIt also contains another LINE that does not contain the above term that should not be found";
        
        assert_eq!(
            vec!["It contains a line that says Testing which should be found by the program", "It also contains another LINE that does not contain the above term that should not be found"], 
            search(query, contents)
        );
    }

    /// Case sensitive result test
    ///
    /// Test if the correct result is returned case sensitive
    #[test]
    fn case_sensitive() {
        let query = "line";
        let contents = "This is a string\nIt contains a line that says Testing which should be found by the program\nIt also contains another LINE that does not contain the above term that should not be found";

        assert_eq!(
            vec!["It contains a line that says Testing which should be found by the program"], 
            search(query, contents)
        );
    }

    /// Case sensitive result test
    ///
    /// Test if the correct result is returned case insensitive
    #[test]
    fn case_insensitive() {
        let query = "lInE";
        let contents = "This is a string\nIt contains a line that says Testing which should be found by the program\nIt also contains another LINE that does not contain the above term that should not be found";

        assert_eq!(
            vec!["It contains a line that says Testing which should be found by the program", "It also contains another LINE that does not contain the above term that should not be found"], 
            search_case_insensitive(query, contents)
        );
    }
}