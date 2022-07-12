package game_map

import (
	"github.com/faiface/pixel"
)

// Entity - все, что может быть отрисовано на карте
type Entity struct {
	CurrentSprite *pixel.Sprite // ?? допустим ну а как анимацию делоть...
	Texture       pixel.Picture //
	Height, Width float64
	TileX, TileY  float64
	X, Y          float64
	StartFrameIdx int // с какого фрейма начинаем рисовать
	Frames        []pixel.Rect
	Children      map[string]*Entity // ну хоть тут понятно
}
