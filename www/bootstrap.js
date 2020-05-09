//  Configure logger
import {ConsoleAppender, defaultConfig, Logger, LogLevel} from "plop-logger";

const appender = new ConsoleAppender(console);
appender.formatLevel = level => {
    switch (level) {
        case LogLevel.Trace:
            return '🐾';
        case LogLevel.Debug:
            return '🐛';
        case LogLevel.Info:
            return 'ℹ️ ';
        case LogLevel.Warn:
            return '⚠️ ';
        case LogLevel.Error:
            return '💥';
        default:
            return LogLevel[level];
    }
}
Logger.config = {
    ...defaultConfig,
    appender
};

const logger = Logger.getLogger("assets");
const cache = new Map();
window.loader = {
    loadImage: (url) => {
        return new Promise(resolve => {
            const cached = cache.get(url);
            if (cached) {
                resolve(cached)
            } else {
                logger.info("Loading image", url);
                const img = new Image();
                cache.set(url, img);
                img.onload = () => resolve(img);
                img.src = url;
            }
        });
    }
};

// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
import("./src")
    .catch(e => console.error("Error importing `index.js`:", e));
