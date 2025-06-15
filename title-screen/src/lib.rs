// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec

//rust is an inverted y-axis engine
//As y goes towards positive infinity, the position goes down the screen.


turbo::go!({ //this is Update - runs 60 times per second.
    
    sprite!("Worms_all2", x = 0, y = 0); //rust macro that draws a sprite. this draws
    
    sprite!("Button", x = 85, y = 10); //draws the button sprite on the screen.
    
});