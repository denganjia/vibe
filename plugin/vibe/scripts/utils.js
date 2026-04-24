/**
 * utils.js
 * Shared utility functions for Vibe scripts.
 */

/**
 * Sanitizes an ID (task ID, run ID, etc.) to prevent path traversal.
 * Only allows alphanumeric characters, underscores, and hyphens.
 * @param {string} id The ID to sanitize.
 * @returns {string} The sanitized ID.
 */
function sanitizeId(id) {
  if (typeof id !== 'string') return '';
  return id.replace(/[^a-zA-Z0-9_-]/g, '');
}

module.exports = {
  sanitizeId
};
