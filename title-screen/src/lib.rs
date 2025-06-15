// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec

//rust is an inverted y-axis engine
//As y goes towards positive infinity, the position goes down the screen.

const BUTTON_COLOR: u32 = 0x4169e1ff;


turbo::go!({
    
    clear(0x00ffffff);
    
    
    let canvas_bounds = bounds::canvas();
    
    sprite!("justworms", x = 0, y = 0);
    
    //sprite!("Worms_all2", x = 128, y = 72);

    text!(
        "Hello, to the bueatiful turbo world!!",
        x = 50,
        y = 50,
        font = "medium");
    
    draw_button(60, 20, 150, 30);
    
});

fn draw_button(w: i32, h: i32, x: i32, y: i32){
    
    
    
    let mut buttonTween = Tween::new(0.0);
    
    buttonTween.set(20.0); 
    rect!(
        w = w,
        h = h,
        x = x,
        y = y,
        color = BUTTON_COLOR,
        border_radius = 2
    );;
}