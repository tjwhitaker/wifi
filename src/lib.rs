mod scanner;

#[cfg(test)]
mod tests {
    #[test]
    fn get_info_osx() {
        println!("> {:?}", super::scanner::get_info());
    }
}
