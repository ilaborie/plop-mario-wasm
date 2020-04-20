/**
 *
 * @param url: string
 * @returns {Promise<HTMLImageElement>}
 */
export function loadImage(url) {
    return new Promise(resolve => {
        const img = new Image();
        img.onload = () => resolve(img);
        img.src = url;
    });
}

/**
 *
 * @param name: string
 * @returns {Promise<Level>}
 */
export function loadLevel(name) {
    return fetch(`/levels/${name}.json`)
        .then(res => res.json());
}