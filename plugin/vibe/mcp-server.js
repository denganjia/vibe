const { McpServer } = require("@modelcontextprotocol/sdk/server/mcp.js");
const { StdioServerTransport } = require("@modelcontextprotocol/sdk/server/stdio.js");
const { z } = require("zod");
const packageJson = require("./package.json");
const { acquireLocks, releaseLocks } = require("./scripts/lock.js");
const { createTask } = require("./scripts/task.js");
const { getTaskStatus, listTasks } = require("./scripts/status.js");

/**
 * Vibe MCP Server
 * 
 * This server provides standardized tools for workspace operations,
 * replacing legacy shell script executions.
 */

// Initialize MCP Server
const server = new McpServer({
  name: "vibe",
  version: packageJson.version,
});

/**
 * Tool: vibe_ping
 * Simple health check tool to verify MCP transport layer.
 */
server.tool(
  "vibe_ping",
  {},
  async () => {
    return {
      content: [{ type: "text", text: "pong" }],
    };
  }
);

/**
 * Tool: vibe_acquire_lock
 * Acquire locks for specific files to prevent concurrent modification.
 */
server.tool(
  "vibe_acquire_lock",
  {
    taskId: z.string().describe("The ID of the task acquiring the locks"),
    filePaths: z.array(z.string()).describe("List of relative file paths to lock"),
  },
  async ({ taskId, filePaths }) => {
    try {
      const success = acquireLocks(process.cwd(), taskId, filePaths);
      if (success) {
        return {
          content: [{ type: "text", text: `Successfully acquired locks for task ${taskId}` }],
        };
      } else {
        return {
          content: [{ type: "text", text: `Failed to acquire locks for task ${taskId}. Some files may be locked by another task.` }],
          isError: true,
        };
      }
    } catch (error) {
      return {
        content: [{ type: "text", text: `Error acquiring locks: ${error.message}` }],
        isError: true,
      };
    }
  }
);

/**
 * Tool: vibe_release_lock
 * Release all locks held by a specific task.
 */
server.tool(
  "vibe_release_lock",
  {
    taskId: z.string().describe("The ID of the task releasing its locks"),
  },
  async ({ taskId }) => {
    try {
      releaseLocks(process.cwd(), taskId);
      return {
        content: [{ type: "text", text: `Released all locks for task ${taskId}` }],
      };
    } catch (error) {
      return {
        content: [{ type: "text", text: `Error releasing locks: ${error.message}` }],
        isError: true,
      };
    }
  }
);

/**
 * Tool: vibe_create_task
 * Create a new task artifact in the workspace.
 */
server.tool(
  "vibe_create_task",
  {
    goal: z.string().max(1000).describe("The primary objective of the task"),
    context: z.array(z.string()).optional().describe("Relevant context or @-references"),
    file_scope: z.array(z.string()).optional().describe("Files expected to be modified"),
    constraints: z.array(z.string()).optional().describe("Implementation constraints"),
    expected_output: z.string().optional().describe("Description of what success looks like"),
  },
  async (taskData) => {
    try {
      const task = createTask(process.cwd(), taskData);
      return {
        content: [{ type: "text", text: `Task created with ID: ${task.id}\n\n${JSON.stringify(task, null, 2)}` }],
      };
    } catch (error) {
      return {
        content: [{ type: "text", text: `Error creating task: ${error.message}` }],
        isError: true,
      };
    }
  }
);

/**
 * Tool: vibe_get_status
 * Get the current status of a task.
 */
server.tool(
  "vibe_get_status",
  {
    taskId: z.string().describe("The ID of the task to query"),
  },
  async ({ taskId }) => {
    try {
      const status = getTaskStatus(process.cwd(), taskId);
      return {
        content: [{ type: "text", text: `Task ${taskId} status: ${status}` }],
      };
    } catch (error) {
      return {
        content: [{ type: "text", text: `Error getting status: ${error.message}` }],
        isError: true,
      };
    }
  }
);

/**
 * Tool: vibe_list_tasks
 * List all tasks in the workspace.
 */
server.tool(
  "vibe_list_tasks",
  {},
  async () => {
    try {
      // Capture listTasks output (it uses console.log which we've redirected)
      let output = "";
      const originalError = console.error;
      console.error = (...args) => {
        output += args.join(" ") + "\n";
      };
      
      listTasks(process.cwd());
      
      console.error = originalError;
      
      return {
        content: [{ type: "text", text: output || "No tasks found." }],
      };
    } catch (error) {
      return {
        content: [{ type: "text", text: `Error listing tasks: ${error.message}` }],
        isError: true,
      };
    }
  }
);

/**
 * Main entry point
 * Sets up stdio transport and connects the server.
 */
async function main() {
  const transport = new StdioServerTransport();
  
  // Ensure all console.log calls are redirected to console.error
  // to avoid corrupting the stdio stream used by MCP.
  const originalLog = console.log;
  console.log = (...args) => {
    console.error(...args);
  };

  try {
    await server.connect(transport);
    console.error("Vibe MCP Server running on stdio");
  } catch (error) {
    console.error("Failed to start Vibe MCP Server:", error);
    process.exit(1);
  }
}

main().catch((error) => {
  console.error("Unhandled error in Vibe MCP Server:", error);
  process.exit(1);
});
