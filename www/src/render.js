import {Fps, PlayerEntity, Sprite, System} from "plop-mario-wasm"
import {loadBackgroundSprites, loadLevel, loadMarioSprite} from "./assets";
import Timer from "./timer";


// window.addEventListener('keydown', ({code, key}) =>
//     console.log({code, key}));

/**
 *
 * @param canvas: HTMLCanvasElement
 */
export function render(canvas) {
    const context = canvas.getContext("2d");

    const sprites = loadBackgroundSprites();
    const mario = loadMarioSprite();
    const level = loadLevel('lvl_1-1');
    const gravity = 3000;

    Promise.all([sprites, mario, level])
        .then(([sprites, playerSprites, level]) => {
            const player_entity = PlayerEntity.new(Sprite.MarioIdle, playerSprites, gravity);
            player_entity.set_position(64, 180);
            player_entity.set_velocity(200, -600);

            const system = System.new(level, sprites, player_entity);
            system.register_keyboard();
            const fps = Fps.new();

            const timer = new Timer(1 / 60, (deltaTime) => {
                displayFps(fps.update());
                system.update_player(deltaTime);
                system.draw(context);
            });
            timer.start();
        });
}

function displayFps(fpsData) {
    const data = {
        latest: fpsData.latest(),
        min: fpsData.min(),
        avg: fpsData.avg(),
        max: fpsData.max()
    };
    const fpsElt = document.getElementById("fps");
    if (fpsElt) {
        Object.entries(data).forEach(([name, value]) => {
            const elt = fpsElt.querySelector(`.${name}`);
            if (elt) {
                elt.textContent = value.toFixed(1);
            }
        });
    }
}