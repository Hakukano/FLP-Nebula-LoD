enum Level {
  Trace,
  Debug,
  Info,
  Warn,
  Error,
}

class Logger {
  log(level: Level, message: unknown) {
    switch (level) {
      case Level.Trace:
        console.trace(message);
        break;
      case Level.Debug:
        console.debug(message);
        break;
      case Level.Info:
        console.info(message);
        break;
      case Level.Warn:
        console.warn(message);
        break;
      case Level.Error:
        console.error(message);
        break;
    }
  }

  trace(message: unknown) {
    this.log(Level.Trace, message);
  }

  debug(message: unknown) {
    this.log(Level.Debug, message);
  }

  info(message: unknown) {
    this.log(Level.Info, message);
  }

  warn(message: unknown) {
    this.log(Level.Warn, message);
  }

  error(message: unknown) {
    this.log(Level.Error, message);
  }
}

export const LOGGER = new Logger();
