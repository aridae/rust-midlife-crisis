//=============================================================
// U r not special
// for winning a game
// with someone who you know was never playing.
//=============================================================

package main

import (
	"github.com/faiface/pixel"
	_ "github.com/faiface/pixel"
	"github.com/faiface/pixel/pixelgl"
)

func run() {
	cfg := pixelgl.WindowConfig{
		Title:  "I wanna kms!",
		Bounds: pixel.R(0, 0, 1024, 768),
		VSync:  true, // to update the window at the same rate as is the refresh rate of your monitor
	}
	win, err := pixelgl.NewWindow(cfg)
	if err != nil {
		panic(err)
	}

	// main game loop
	for !win.Closed() {
		win.Update()
	}
}

func main() {
	/* There's one ugly thing about graphics and operating systems.
	That one thing is that most operating systems require all graphics and
	windowing code to be executed from the main thread of our program.
	This is really cumbersome with Go. Go is a highly concurrent language with goroutines.
	Goroutines can freely jump from thread to thread, which makes the previous requirement
	seemingly impossible. Not all is lost, however. Go's runtime provides a convenient
	function runtime.LockOSThread, which locks current goroutine on it's current thread.
	PixelGL uses this functionality and provides you with a simpler interface to it.
	You don't have to deal with the main thread stuff at all with Pixel. You can run your game concurrently,
	however you want. You only need to allow Pixel to use the main thread. */
	pixelgl.Run(run)
}
