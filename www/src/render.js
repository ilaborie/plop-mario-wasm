import {SpriteSheet, Sprite, System, Position} from "plop-mario-wasm"
import {loadBackgroundSprites, loadLevel, loadMarioSprite} from "./assets";


class Compositor {
    constructor() {
        this.layers = [];
    }

    draw(context) {
        this.layers.forEach(layer => layer.draw(context));
    }
}

/**
 *
 * @param canvas: HTMLCanvasElement
 */
export function render(canvas) {
    const context = canvas.getContext("2d");

    const sprites = loadBackgroundSprites();
    const mario = loadMarioSprite();
    const level = loadLevel('lvl_1-1');

    Promise.all([sprites, mario, level])
        .then(([sprites, mario, level]) => {
            let system = System.new(level, sprites, mario);

            function update() {
                system.draw_all(context);
                system.move_player();

                requestAnimationFrame(update);
            }

            update();
        });
}
