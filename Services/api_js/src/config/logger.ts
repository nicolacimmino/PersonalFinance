import winston from 'winston';
import path from 'path';
import dotenv from 'dotenv';

dotenv.config();

const logLevel = process.env.LOG_LEVEL || 'info';
const nodeEnv = process.env.NODE_ENV || 'development';

// Custom format matching log4rs format: [timestamp] - message
const customFormat = winston.format.combine(
  winston.format.timestamp({ format: 'YYYY-MM-DD HH:mm:ss' }),
  winston.format.printf(({ timestamp, level, message, ...metadata }) => {
    let log = `[${timestamp}] ${level.toUpperCase()}: ${message}`;

    // Add metadata if present
    if (Object.keys(metadata).length > 0) {
      log += ` ${JSON.stringify(metadata)}`;
    }

    return log;
  })
);

// Configure transports
const transports: winston.transport[] = [
  // Console transport (always enabled)
  new winston.transports.Console({
    format: winston.format.combine(
      winston.format.colorize(),
      customFormat
    ),
  }),
];

// File transport (only in production or if explicitly enabled)
if (nodeEnv === 'production' || process.env.ENABLE_FILE_LOGGING === 'true') {
  transports.push(
    new winston.transports.File({
      filename: path.join('logs', 'run.log'),
      format: customFormat,
      maxsize: 5242880, // 5MB
      maxFiles: 5,
    })
  );
}

// Create logger instance
export const logger = winston.createLogger({
  level: logLevel,
  transports,
  exitOnError: false,
});

// Log unhandled rejections and exceptions
logger.exceptions.handle(
  new winston.transports.Console(),
  new winston.transports.File({ filename: path.join('logs', 'exceptions.log') })
);

logger.rejections.handle(
  new winston.transports.Console(),
  new winston.transports.File({ filename: path.join('logs', 'rejections.log') })
);

export default logger;
