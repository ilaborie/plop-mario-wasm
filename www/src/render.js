import {SpriteSheet, Sprite} from "plop-mario-wasm"

/**
 *
 * @param canvas: HTMLCanvasElement
 */
import {loadImage, loadLevel} from "./assets";

export function render(canvas) {
    const context = canvas.getContext("2d");

    context.fillRect(0, 0, 50, 50);

    loadImage("assets/tiles.png")
        .then(img => {
            const sprites = SpriteSheet.new(img, 16, 16);
            sprites.define(Sprite.Ground, 0, 0);
            sprites.define(Sprite.Sky, 3, 23);
            return sprites;
        }).then(sprites => {
        loadLevel('lvl_1-1')
            .then(level => sprites.draw_level(context, level));
    });
}


//
// for (let x = 0; x < 25; ++x) {
//     for (let y = 0; y < 14; ++y) {
//         sprites.draw_tile(Sprite.Sky, context, x, y);
//     }
// }
// for (let x = 0; x < 25; ++x) {
//     for (let y = 12; y < 14; ++y) {
//         sprites.draw_tile(Sprite.Ground, context, x, y);
//     }
// }