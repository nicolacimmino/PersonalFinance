import { Pool, QueryResult, QueryResultRow } from 'pg';
import dotenv from 'dotenv';
import { logger } from './logger';

dotenv.config();

// Create PostgreSQL connection pool
const pool = new Pool({
  connectionString: process.env.DATABASE_URL,
  max: 5, // Smaller pool for batch job
  idleTimeoutMillis: 30000,
  connectionTimeoutMillis: 2000,
});

// Handle pool errors
pool.on('error', (err) => {
  logger.error('Unexpected error on idle client', err);
  process.exit(-1);
});

/**
 * Initialize database connection
 */
export async function initDatabase(): Promise<void> {
  try {
    const client = await pool.connect();
    logger.info('Database connection established successfully');
    client.release();
  } catch (err) {
    logger.error('Error connecting to the database', { error: err });
    throw err;
  }
}

/**
 * Execute a SQL query with parameters
 * @param text SQL query string with $1, $2, etc. placeholders
 * @param params Array of parameter values
 * @returns Query result
 */
export async function query<T extends QueryResultRow = any>(
  text: string,
  params?: any[]
): Promise<QueryResult<T>> {
  const start = Date.now();
  try {
    const result = await pool.query<T>(text, params);
    const duration = Date.now() - start;
    logger.debug('Executed query', {
      text,
      duration,
      rows: result.rowCount,
    });
    return result;
  } catch (error) {
    logger.error('Database query error', { text, error });
    throw error;
  }
}

/**
 * Gracefully close the database pool
 */
export async function closePool(): Promise<void> {
  await pool.end();
  logger.info('Database pool has been closed');
}

export default pool;
