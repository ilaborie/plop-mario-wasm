import {Level, Sprite, SpriteSheet} from "plop-mario-wasm";
import {Logger} from "plop-logger";

const logger = Logger.getLogger("assets");

/**
 *
 * @param url: string
 * @returns {Promise<HTMLImageElement>}
 */
function loadImage(url) {
    return new Promise(resolve => {
        logger.info("Loading image", url);
        const img = new Image();
        img.onload = () => resolve(img);
        img.src = url;
    });
}

/**
 *
 * @returns {Promise<SpriteSheet>}
 */
export function loadBackgroundSprites() {
    let url = "assets/tiles.png";
    return loadImage(url)
        .then(img => {
            const sprites = SpriteSheet.new(img, 16, 16);
            sprites.define_tile(Sprite.Ground, 0, 0);
            sprites.define_tile(Sprite.Sky, 3, 23);
            return sprites;
        });
}

/**
 *
 * @returns {Promise<SpriteSheet>}
 */
export function loadMarioSprite() {
    let url = "assets/characters.gif";
    return loadImage(url)
        .then(img => {
            const sprites = SpriteSheet.new(img, 16, 16);
            sprites.define(Sprite.MarioIdle, 276, 44, 16, 16);
            return sprites;
        });
}

/**
 *
 * @param name: string
 * @returns {Promise<Level>}
 */
export function loadLevel(name) {
    logger.info("Loading level", name);
    return fetch(`/levels/${name}.json`)
        .then(res => res.json())
        .then(json => Level.new(json));
}