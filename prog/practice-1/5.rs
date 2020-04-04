use std::string::String;

struct My_box {
    x : i32,
    y : i32
}

impl My_box {
    pub fn new(_x: i32, _y: i32) -> My_box{
        My_box{x: _x, y: _y}
    }
}

struct My_circle {
    x : i32,
    y : i32,
    r : i32
}


impl My_circle {
    pub fn new(_x: i32, _y: i32, _r: i32) -> My_circle{
        My_circle{x: _x, y: _y, r: _r}
    }
}

trait Drawable {
    fn draw(&self) -> Result<T,E>;
}

impl Drawable for My_box {
    fn draw(&self) -> Result<String,()> {
        let result = format!("사각형을 x : {}, y : {} 에 그립니다.", self.x, self.y);
        Ok(result)
    }
}

impl Drawable for My_circle {
    fn draw(&self) -> Result<String,()> {
        let result = format!("원을 x : {}, y : {} 에 반지름 r : {} 그립니다.", self.x, self.y, self.r);
        Ok(result)
    }
}

fn main () -> std::io::Result<()> {

    let mybox = My_box::new(1,1);
    let mycircle = My_circle::new(1,1,1);

    println!("{}",mybox.draw().unwrap());
    println!("{}",mycircle.draw().unwrap());

    Ok(())
}