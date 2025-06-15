// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec

//rust is an inverted y-axis engine
//As y goes towards positive infinity, the position goes down the screen.



turbo::go!({ //this is Update - runs 60 times per second.

    if !audio::is_playing("Bloopin") {
        audio::play("Bloopin");
}
    
    
    //this will get drawn first.
    sprite!("Worms_all2", x = 0, y = 0); //rust macro that draws a sprite. this draws the background.
    //
    sprite!("Button", x = 85, y = 10); //draws the button sprite on the screen.


    let p = pointer(); //get's the pointer data.

    let canvas_bounds = bounds::canvas(); // this creates a bounds for the full screen size.

    let bounds = Bounds::with_size(44, 28) //we create another bounds for our button.
        .anchor_center(&canvas_bounds) // set's the bound to the center of the canvas that we've created.
        .translate_x(38) //we move the bounds to the right.
        .translate_y(-27); //we move the bounds up. 
    
    //this is for us to see where the bounds is.
    // let button_Collider = rect!(
    //     color = 0xffffffff,
    //     
    //     xy = bounds.xy(),
    //     
    //     wh = bounds.wh(),
    // );
    
    let is_btn_hovered = p.xy().intersects_bounds(bounds); //create a variable that checks if the pointer is intersecting with our bounds. 
    
    if(is_btn_hovered && p.pressed()){ //if we intersect and press our pointer,
        sprite!("Button_press", x = 85, y = 10); //we change the sprite.
    }
});