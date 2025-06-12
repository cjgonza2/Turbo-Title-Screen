// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec

//rust is an inverted y-axis engine
//As y goes towards positive infinity, the position goes down the screen.


turbo::go!({
    text!(
        "Hello, to the bueatiful turbo world!!",
        x = 50,
        y = 50,
        font = "medium");
    
    rect!(
        w = 20,
        h = 20,
        x = 40,
        y = 60
    );
});