use flexi_logger::{FileSpec, LogSpecification, Logger};

fn init_logger(){
    if cfg!(debug_assertions){
        Logger::with(LogSpecification::trace())
        .log_to_file(FileSpec::default())
        .start().unwrap();
    }else{
        Logger::with(LogSpecification::info())
        .log_to_file(FileSpec::default())
        .start().unwrap();
    }
}

fn main() {
    init_logger();
}
