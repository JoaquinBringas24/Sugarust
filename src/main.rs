mod vulkanapp;
use vulkanapp::VulkanApp;
use std::io;

fn main() -> Result<(), io::Error> {
    
    let app = VulkanApp{
        width:800, 
        height:800,
        title: "Sugarust".to_string()
    };

    app.run();

    Ok(println!("EXITSUCCESS"))

    

}