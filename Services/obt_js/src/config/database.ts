import { Pool, QueryResult, QueryResultRow } from 'pg';
import dotenv from 'dotenv';
import { logger } from './logger';

dotenv.config();

const pool = new Pool({
  connectionString: process.env.DATABASE_URL,
  max: 5,
  idleTimeoutMillis: 30000,
  connectionTimeoutMillis: 2000,
});

pool.on('error', (err) => {
  logger.error('Unexpected error on idle client', err);
  process.exit(-1);
});

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

export async function closePool(): Promise<void> {
  await pool.end();
  logger.info('Database pool has been closed');
}

export default pool;
