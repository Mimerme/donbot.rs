/// This is a function that filters out invalid filename characters
/// Filters using a Whitelist approach. Weird OSes have weird stuff going on
/// Approach taken from: https://stackoverflow.com/a/295146
/// Valid characters: -_.() abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789
pub fn filter_filename(filename_in : &mut String){
		const FILENAME_WHITELIST : &str = 
			"-_.() abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
		filename_in.retain(|x| {FILENAME_WHITELIST.contains(x)});
}
