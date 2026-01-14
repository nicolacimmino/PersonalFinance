import axios from 'axios';
import type { AxiosInstance, AxiosError, InternalAxiosRequestConfig, AxiosResponse } from 'axios';
import { config } from '@/config';

/**
 * HTTP Client
 *
 * Centralized Axios instance with request/response interceptors.
 * Automatically adds API key to all requests and handles common errors.
 */

// Create axios instance with default configuration
const httpClient: AxiosInstance = axios.create({
  baseURL: config.apiHost,
  timeout: 30000, // 30 seconds
  headers: {
    'Content-Type': 'application/json',
  },
});

/**
 * Request Interceptor
 * Automatically adds API key to all requests
 */
httpClient.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    // Get API key from localStorage
    const apiKey = localStorage.getItem('pfinanceApiKey');

    // Add API key header if available
    if (apiKey && config.headers) {
      config.headers['X-API-KEY'] = apiKey;
    }

    // Log request in development
    if (import.meta.env.DEV) {
      console.log(`[HTTP Request] ${config.method?.toUpperCase()} ${config.url}`);
    }

    return config;
  },
  (error: AxiosError) => {
    console.error('[HTTP Request Error]', error);
    return Promise.reject(error);
  }
);

/**
 * Response Interceptor
 * Handles common response patterns and errors
 */
httpClient.interceptors.response.use(
  (response: AxiosResponse) => {
    // Log response in development
    if (import.meta.env.DEV) {
      console.log(`[HTTP Response] ${response.config.method?.toUpperCase()} ${response.config.url}`, response.status);
    }

    // Return data directly for convenience
    return response.data;
  },
  (error: AxiosError) => {
    // Log error
    console.error('[HTTP Response Error]', {
      url: error.config?.url,
      method: error.config?.method,
      status: error.response?.status,
      message: error.message,
    });

    // Handle specific error cases
    if (error.response) {
      // Server responded with error status
      switch (error.response.status) {
        case 401:
          console.error('Unauthorized - Invalid API key');
          break;
        case 403:
          console.error('Forbidden - Access denied');
          break;
        case 404:
          console.error('Not Found');
          break;
        case 500:
          console.error('Internal Server Error');
          break;
        default:
          console.error(`Error ${error.response.status}: ${error.message}`);
      }
    } else if (error.request) {
      // Request was made but no response received
      console.error('No response received from server');
    } else {
      // Something else happened
      console.error('Request setup error:', error.message);
    }

    return Promise.reject(error);
  }
);

export default httpClient;
