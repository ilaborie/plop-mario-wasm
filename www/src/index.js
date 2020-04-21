import {set_panic_hook} from "plop-mario-wasm";
import {render} from "./render";
import {ConsoleAppender, defaultConfig, Logger, LogLevel} from "plop-logger";

(function () {
    set_panic_hook();

    //  Configure logger
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

    const canvas = document.createElement("canvas");
    canvas.id = "plop-mario";
    canvas.width = 640;
    canvas.height = 640;
    document.body.appendChild(canvas);

    render(canvas);
})();


