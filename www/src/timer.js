export default class Timer {

    constructor(deltaTime, update) {
        let accumulatedTime = 0;
        let lastTime = 0;
        this.updateProxy = (time) => {
            accumulatedTime += (time - lastTime) / 1000;

            while (accumulatedTime > deltaTime) {
                update(deltaTime);
                accumulatedTime -= deltaTime;
            }
            lastTime = time;

            this.enqueue();
        };
    }

    enqueue() {
        requestAnimationFrame(this.updateProxy);
    }

    start() {
        this.enqueue();
    }
}