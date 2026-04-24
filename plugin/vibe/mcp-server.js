const { McpServer } = require("@modelcontextprotocol/sdk/server/mcp.js");
const { StdioServerTransport } = require("@modelcontextprotocol/sdk/server/stdio.js");
const { z } = require("zod");
const fs = require("fs");
const path = require("path");
const yaml = require("js-yaml");
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
 * Dynamic Skill Loading
 * Scans the skills directory and registers tools based on SKILL.md metadata.
 */
async function loadSkills() {
  const skillsDir = path.join(__dirname, 'skills');
  if (!fs.existsSync(skillsDir)) return;

  const skillFolders = fs.readdirSync(skillsDir, { withFileTypes: true })
    .filter(dirent => dirent.isDirectory())
    .map(dirent => dirent.name);

  for (const folder of skillFolders) {
    const skillMdPath = path.join(skillsDir, folder, 'SKILL.md');
    if (!fs.existsSync(skillMdPath)) continue;

    try {
      const content = fs.readFileSync(skillMdPath, 'utf8');
      const match = content.match(/^---\n([\s\S]*?)\n---/);
      if (!match) continue;

      const metadata = yaml.load(match[1]);
      const skillName = metadata.name || folder;
      const toolName = `vibe_skill_${skillName.replace(/^vibe-/, "").replace(/-/g, "_")}`;
      const description = metadata.description || `Execute the ${folder} skill.`;

      // Try to find corresponding script
      const scriptPath = path.join(__dirname, 'scripts', `${folder}.js`);
      
      server.tool(
        toolName,
        description,
        {
          params: z.any().optional().describe("Skill-specific inputs as defined in SKILL.md"),
          workspaceRoot: z.string().optional().describe("Optional override for the workspace root directory")
        },
        async ({ params = {}, workspaceRoot }) => {
          try {
            if (!fs.existsSync(scriptPath)) {
              return {
                content: [{ type: "text", text: `Error: Script for skill ${folder} not found at ${scriptPath}` }],
                isError: true
              };
            }

            // Security: validate workspaceRoot is within cwd
            const cwd = process.cwd();
            const root = workspaceRoot ? path.resolve(cwd, workspaceRoot) : cwd;
            const relative = path.relative(cwd, root);
            const isOutside = relative.startsWith('..') || path.isAbsolute(relative);
            
            if (isOutside) {
               return {
                content: [{ type: "text", text: `Error: workspaceRoot must be within the current working directory` }],
                isError: true
              };
            }

            // Clear cache to allow for script updates during development
            delete require.cache[require.resolve(scriptPath)];
            const script = require(scriptPath);
            
            if (typeof script.runSkill !== 'function') {
               return {
                content: [{ type: "text", text: `Error: Script for skill ${folder} does not implement runSkill` }],
                isError: true
              };
            }

            const result = await script.runSkill(params, root);
            const textOutput = typeof result === 'string' 
              ? result 
              : (result !== undefined ? JSON.stringify(result, null, 2) : "Skill executed successfully (no output).");
              
            return {
              content: [{ type: "text", text: textOutput }]
            };
          } catch (error) {
            return {
              content: [{ type: "text", text: `Error executing skill ${skillName}: ${error.message}` }],
              isError: true
            };
          }
        }
      );
      console.error(`Registered dynamic tool: ${toolName} (${description})`);
    } catch (error) {
      console.error(`Failed to load skill from ${folder}:`, error.message);
    }
  }
}

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
      let output = "";
      listTasks(process.cwd(), (msg) => {
        output += msg + "\n";
      });
      
      return {
        content: [{ type: "text", text: output.trim() || "No tasks found." }],
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
    await loadSkills();
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
