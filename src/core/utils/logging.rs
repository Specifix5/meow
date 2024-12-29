#[macro_export(local_inner_macros)]
macro_rules! logger {
  ($text:expr) => {
    ::std::println!("{} - {}", ::chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"), $text);
  };
}
